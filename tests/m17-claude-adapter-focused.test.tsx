import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const project = [{
  id: "claude-project",
  name: "Claude project",
  originalPath: "C:\\Projects\\Claude",
  normalizedPath: "C:\\Projects\\Claude",
  status: "ACTIVE",
  priority: 1,
  preferredBuilder: null,
  preferredAuditor: null,
  taskSourcePolicy: "DISCOVER_STANDARD_FILES",
  preferredAgentProvider: "CLAUDE",
  registeredAt: "2026-09-14T10:00:00Z",
  lastValidatedAt: "2026-09-14T10:00:00Z",
  repository: null,
}];
const readiness = [
  {
    provider: "CODEX",
    available: true,
    version: "codex-cli",
    readinessState: "VERSION_VERIFIED_AUTH_UNKNOWN",
    diagnosticCode: null,
    diagnosticMessage: null,
    capabilities: ["START", "LIST", "STOP"],
    supportsPty: false,
    supportsResume: false,
    checkedAt: "2026-09-14T10:00:00Z",
  },
  {
    provider: "CLAUDE",
    available: true,
    version: "2.1.270 (Claude Code)",
    readinessState: "READY",
    diagnosticCode: null,
    diagnosticMessage: "Managed login verified",
    capabilities: ["START", "LIST", "STOP", "BOUNDED_STREAM_JSON", "RESUME", "CONTINUE", "PERMISSION_PROMPTS"],
    supportsPty: false,
    supportsResume: true,
    checkedAt: "2026-09-14T10:00:00Z",
  },
];
const session = {
  id: "hiveai-session-1",
  provider: "CLAUDE",
  projectId: "claude-project",
  taskId: null,
  operationKind: "FREEFORM_PROJECT_OPERATION",
  state: "ORPHANED",
  cwd: "C:\\Projects\\Claude",
  startedAt: "2026-09-14T10:01:00Z",
  endedAt: "2026-09-14T10:01:04Z",
  exitCode: null,
  stdout: "The Claude process was orphaned after restart.",
  stderr: "",
  stdoutTruncated: false,
  stderrTruncated: false,
  finalResponse: null,
  finalResponseTruncated: false,
  finalResponseState: "UNAVAILABLE",
  finalResponseRole: null,
  diagnosticCode: "CLAUDE_PROCESS_NOT_OWNED_AFTER_RESTART",
  diagnosticMessage: "The provider process is no longer owned; explicit resume is available.",
  promptReference: null,
  promptBody: "Continue the bounded repository task.",
  promptId: null,
  promptVersionId: null,
  promptVersion: null,
  promptVersionSha256: null,
  providerVersion: "2.1.270 (Claude Code)",
  providerSessionId: "claude-provider-session-1",
  providerSessionProvenance: { provider: "CLAUDE", providerSessionId: "claude-provider-session-1" },
  providerCwdIdentity: "C:\\Projects\\Claude",
  elapsedMs: 4000,
  supportsResume: true,
  supportsPty: false,
  events: [],
};

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.sessionStorage.clear();
  window.history.pushState({}, "", "/agents");
  invoke.mockReset();
  invoke.mockImplementation((command: string) => {
    if (command === "hiveai_projects_list") return Promise.resolve(project);
    if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
    if (command === "hiveai_agent_sessions_list") return Promise.resolve([session]);
    if (command === "hiveai_agent_resume") return Promise.resolve({ ...session, state: "STARTING" });
    if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
    if (command === "hiveai_git_diff") return Promise.resolve({ text: "", truncated: false });
    return Promise.resolve({});
  });
});

describe("M17 Claude Code adapter", () => {
  it("exposes truthful readiness capabilities and resumes the exact provider session", async () => {
    render(<App />);
    expect(await screen.findByTestId("claude-readiness")).toHaveTextContent("READY");
    expect(screen.getByTestId("claude-readiness")).toHaveTextContent("2.1.270 (Claude Code)");
    expect(screen.getByTestId("claude-readiness")).toHaveTextContent("RESUME");

    fireEvent.click(await screen.findByRole("button", { name: /View CLAUDE FREEFORM_PROJECT_OPERATION ORPHANED/i }));
    expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("ORPHANED");
    fireEvent.click(screen.getByRole("button", { name: "Resume exact session" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_agent_resume", {
      projectId: "claude-project",
      sessionId: "hiveai-session-1",
    }));
  });

  it("shows a persisted live permission attention state and diagnostic", async () => {
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve(project);
      if (command === "hiveai_agent_readiness") return Promise.resolve(readiness);
      if (command === "hiveai_agent_sessions_list") {
        return Promise.resolve([{ ...session, state: "WAITING_PERMISSION", diagnosticCode: "CLAUDE_PERMISSION_REQUIRED", diagnosticMessage: "Claude is waiting for an explicit permission decision." }]);
      }
      if (command === "hiveai_git_snapshot") return Promise.resolve({ stagedFiles: [], unstagedFiles: [], untrackedFiles: [], conflictedFiles: [] });
      if (command === "hiveai_git_diff") return Promise.resolve({ text: "", truncated: false });
      return Promise.resolve({});
    });
    render(<App />);
    fireEvent.click(await screen.findByRole("button", { name: /View CLAUDE FREEFORM_PROJECT_OPERATION WAITING_PERMISSION/i }));
    expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("WAITING PERMISSION");
    expect(screen.getByTestId("agent-session-detail")).toHaveTextContent("Claude is waiting for an explicit permission decision.");
  });
});
