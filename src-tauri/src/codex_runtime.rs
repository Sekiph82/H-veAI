use crate::process_policy::background_command;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub const PROCESS_POLL: Duration = Duration::from_millis(50);
pub const PROBE_OUTPUT_BYTES: usize = 16 * 1024;
pub const READINESS_TIMEOUT: Duration = Duration::from_secs(5);
pub const MAX_NATIVE_HEADER_OFFSET: u64 = 1024 * 1024;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodexExecutableResolution {
    pub selected: Option<PathBuf>,
    pub skipped_candidates: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    Timeout,
    Malformed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginState {
    ChatGpt,
    ApiKey,
    NotLoggedIn,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct BoundedProcessResult {
    pub output: Output,
    pub timed_out: bool,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

pub fn resolve_codex_executable() -> CodexExecutableResolution {
    let mut entries = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    entries.extend(known_codex_install_candidates());
    resolve_codex_executable_from_entries(entries)
}

pub fn resolve_codex_executable_from_entries<I>(entries: I) -> CodexExecutableResolution
where
    I: IntoIterator<Item = PathBuf>,
{
    let mut resolution = CodexExecutableResolution::default();
    let mut seen = std::collections::HashSet::new();
    for entry in entries {
        for name in codex_executable_names() {
            let candidate = entry.join(name);
            if !candidate.is_file() || !seen.insert(candidate.clone()) {
                continue;
            }
            if !is_direct_candidate_name(name) {
                resolution.skipped_candidates = resolution.skipped_candidates.saturating_add(1);
                continue;
            }
            if is_native_codex_candidate(&candidate) {
                resolution.selected = Some(candidate);
                return resolution;
            }
            resolution.skipped_candidates = resolution.skipped_candidates.saturating_add(1);
        }
    }
    resolution
}

pub fn probe_version(path: &Path, timeout: Duration) -> Result<String, ProbeError> {
    let mut child = background_command(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| ProbeError::Failed)?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.wait_with_output().map_err(|_| ProbeError::Failed)?;
                if !status.success() {
                    return Err(ProbeError::Failed);
                }
                return parse_version(&output);
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProbeError::Timeout);
            }
            Err(_) => return Err(ProbeError::Failed),
        }
    }
}

pub fn probe_login_status(path: &Path, timeout: Duration) -> Result<LoginState, ProbeError> {
    let mut child = background_command(path)
        .args(["login", "status"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| ProbeError::Failed)?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                let output = child.wait_with_output().map_err(|_| ProbeError::Failed)?;
                return parse_login_status(&output);
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProbeError::Timeout);
            }
            Err(_) => return Err(ProbeError::Failed),
        }
    }
}

pub fn parse_version(output: &Output) -> Result<String, ProbeError> {
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let first = text.lines().next().unwrap_or_default().trim();
    if first.len() > 256 || !first.to_ascii_lowercase().contains("codex") {
        return Err(ProbeError::Malformed);
    }
    Ok(first.to_string())
}

fn parse_login_status(output: &Output) -> Result<LoginState, ProbeError> {
    let mut bytes = Vec::new();
    bytes.extend(output.stdout.iter().copied().take(PROBE_OUTPUT_BYTES));
    bytes.extend(output.stderr.iter().copied().take(PROBE_OUTPUT_BYTES));
    let text = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
    if text.contains("logged in using chatgpt") || text.contains("chatgpt login") {
        return Ok(LoginState::ChatGpt);
    }
    if text.contains("api key") || text.contains("api-key") || text.contains("apikey") {
        return Ok(LoginState::ApiKey);
    }
    if text.contains("not logged")
        || text.contains("not authenticated")
        || text.contains("login required")
    {
        return Ok(LoginState::NotLoggedIn);
    }
    Ok(LoginState::Unknown)
}

pub fn run_bounded_process(
    mut command: std::process::Command,
    stdin_bytes: Option<&[u8]>,
    timeout: Duration,
    max_stdout: usize,
    max_stderr: usize,
) -> Result<BoundedProcessResult, ProbeError> {
    let mut child = command
        .stdin(if stdin_bytes.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| ProbeError::Failed)?;
    if let Some(bytes) = stdin_bytes {
        if let Some(mut stdin) = child.stdin.take() {
            let bytes = bytes.to_vec();
            thread::spawn(move || {
                use std::io::Write;
                let _ = stdin.write_all(&bytes);
            });
        }
    }
    let stdout = child.stdout.take().ok_or(ProbeError::Failed)?;
    let stderr = child.stderr.take().ok_or(ProbeError::Failed)?;
    let stdout_thread = thread::spawn(move || read_bounded(stdout, max_stdout));
    let stderr_thread = thread::spawn(move || read_bounded(stderr, max_stderr));
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = stdout_thread.join().map_err(|_| ProbeError::Failed)?;
                let stderr = stderr_thread.join().map_err(|_| ProbeError::Failed)?;
                let mut output = Output {
                    status,
                    stdout: stdout.0,
                    stderr: stderr.0,
                };
                let stdout_truncated = stdout.1;
                let stderr_truncated = stderr.1;
                if stdout_truncated {
                    output.stdout = output.stdout.into_iter().take(max_stdout).collect();
                }
                if stderr_truncated {
                    output.stderr = output.stderr.into_iter().take(max_stderr).collect();
                }
                return Ok(BoundedProcessResult {
                    output,
                    timed_out: false,
                    stdout_truncated,
                    stderr_truncated,
                });
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL),
            Ok(None) => {
                let _ = child.kill();
                let status = child.wait().map_err(|_| ProbeError::Failed)?;
                let stdout = stdout_thread.join().map_err(|_| ProbeError::Failed)?;
                let stderr = stderr_thread.join().map_err(|_| ProbeError::Failed)?;
                return Ok(BoundedProcessResult {
                    output: Output {
                        status,
                        stdout: stdout.0.into_iter().take(max_stdout).collect(),
                        stderr: stderr.0.into_iter().take(max_stderr).collect(),
                    },
                    timed_out: true,
                    stdout_truncated: stdout.1,
                    stderr_truncated: stderr.1,
                });
            }
            Err(_) => return Err(ProbeError::Failed),
        }
    }
}

fn read_bounded<R: Read>(mut reader: R, max: usize) -> (Vec<u8>, bool) {
    let mut bytes = Vec::new();
    let mut buffer = [0u8; 4096];
    let mut truncated = false;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                let remaining = max.saturating_sub(bytes.len());
                bytes.extend_from_slice(&buffer[..count.min(remaining)]);
                if count > remaining {
                    truncated = true;
                    break;
                }
            }
            Err(_) => {
                truncated = true;
                break;
            }
        }
    }
    (bytes, truncated)
}

#[cfg(windows)]
fn codex_executable_names() -> &'static [&'static str] {
    &["codex.exe", "codex"]
}

#[cfg(not(windows))]
fn codex_executable_names() -> &'static [&'static str] {
    &["codex", "codex.exe"]
}

#[cfg(windows)]
fn is_direct_candidate_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("codex.exe")
}

#[cfg(not(windows))]
fn is_direct_candidate_name(_name: &str) -> bool {
    true
}

#[cfg(windows)]
fn known_codex_install_candidates() -> Vec<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .map(|root| vec![PathBuf::from(root).join(r"OpenAI\Codex\bin")])
        .unwrap_or_default()
}

#[cfg(not(windows))]
fn known_codex_install_candidates() -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(windows)]
fn is_native_codex_candidate(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() || metadata.len() < 64 {
        return false;
    }
    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };
    let mut dos_header = [0u8; 64];
    if file.read_exact(&mut dos_header).is_err() || &dos_header[..2] != b"MZ" {
        return false;
    }
    let pe_offset = u32::from_le_bytes([
        dos_header[0x3c],
        dos_header[0x3d],
        dos_header[0x3e],
        dos_header[0x3f],
    ]) as u64;
    if pe_offset > MAX_NATIVE_HEADER_OFFSET || pe_offset.saturating_add(26) > metadata.len() {
        return false;
    }
    if file.seek(SeekFrom::Start(pe_offset)).is_err() {
        return false;
    }
    let mut pe_header = [0u8; 26];
    if file.read_exact(&mut pe_header).is_err() || &pe_header[..4] != b"PE\0\0" {
        return false;
    }
    let machine = u16::from_le_bytes([pe_header[4], pe_header[5]]);
    let optional_magic = u16::from_le_bytes([pe_header[24], pe_header[25]]);
    matches!(machine, 0x014c | 0x8664 | 0xaa64) && matches!(optional_magic, 0x010b | 0x020b)
}

#[cfg(not(windows))]
fn is_native_codex_candidate(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn login_status_classification_never_reads_auth_files() {
        let output = Command::new("codex").output().unwrap_or_else(|_| Output {
            status: std::process::ExitStatus::default(),
            stdout: b"Logged in using ChatGPT".to_vec(),
            stderr: Vec::new(),
        });
        let state = parse_login_status(&output).unwrap();
        assert!(matches!(state, LoginState::ChatGpt | LoginState::Unknown));
    }
}
