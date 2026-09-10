param()
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$candidate = Join-Path $root 'src-tauri\target\release\hiveai-desktop.exe'
$stableDir = Join-Path $root 'dev-bin'
$stable = Join-Path $stableDir 'H!veAI.exe'
$icon = Join-Path $stableDir 'H!veAI.ico'
if (!(Test-Path -LiteralPath $candidate)) { throw "Missing production executable: $candidate" }
$bytes = [IO.File]::ReadAllBytes($candidate)
if ($bytes.Length -lt 2 -or [Text.Encoding]::ASCII.GetString($bytes, 0, 2) -ne 'MZ') { throw 'Production output is not a Windows PE executable.' }
New-Item -ItemType Directory -Force -Path $stableDir | Out-Null
Copy-Item -LiteralPath $candidate -Destination $stable -Force
Copy-Item -LiteralPath (Join-Path $root 'src-tauri\icons\icon.ico') -Destination $icon -Force
$process = Start-Process -FilePath $stable -PassThru
try {
  Start-Sleep -Seconds 3
  if ($process.HasExited) { throw "Published executable exited during native smoke: $($process.ExitCode)" }
} finally {
  if (!$process.HasExited) { [void]$process.CloseMainWindow(); if (!$process.WaitForExit(5000)) { $process.Kill(); [void]$process.WaitForExit(5000) } }
}
Write-Output "Stable executable: $stable"
Write-Output "SHA-256: $((Get-FileHash -LiteralPath $stable -Algorithm SHA256).Hash)"
Write-Output "Opening video SHA-256: $((Get-FileHash -LiteralPath (Join-Path $root 'src/assets/H!veAI.mp4') -Algorithm SHA256).Hash)"
