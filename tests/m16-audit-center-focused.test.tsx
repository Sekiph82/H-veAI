import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const invoke = vi.hoisted(() => vi.fn());
const project = { id: "audit-project", name: "Audit Fixture", originalPath: "C:\\Projects\\Audit Fixture", normalizedPath: "C:\\Projects\\Audit Fixture", status: "ACTIVE", priority: 1, preferredBuilder: null, preferredAuditor: "GPT", taskSourcePolicy: "DISCOVER_STANDARD_FILES", preferredAgentProvider: "CODEX", registeredAt: "2026-09-08T10:00:00Z", lastValidatedAt: "2026-09-08T10:00:00Z", repository: null };
const finding = { id: "finding-1", findingKey: "known-bad-fixture", severity: "MAJOR", title: "Known defect", detail: "The fixture omits the required source symbol.", requirementRefs: ["req-1"], evidenceRefs: ["SOURCE_SNIPPET:src/lib.rs"], sourceLocator: "src/lib.rs:10", testLocator: "tests/audit.rs:12", confidence: "HIGH", status: "OPEN", remediationGuidance: "Add the symbol and a direct test.", blocksRelease: true, closedByAuditId: null };
const audit = { id: "audit-1", projectId: project.id, taskId: null, auditType: "IMPLEMENTATION", auditedBranch: "H!veAI", auditedHeadSha: "abc123", baselineRef: "origin/H!veAI", inputManifestSha256: "input-hash", schemaVersion: 1, verdict: "CONDITIONAL", confidence: "LOW", regressionRisk: "HIGH", state: "COMPLETED", summary: "Evidence collected; GPT provider unavailable.", startedAt: "2026-09-08T10:01:00Z", finishedAt: "2026-09-08T10:01:01Z", auditorProvider: "OPENAI_GPT", auditorModel: "UNCONFIGURED", auditorVersion: "UNAVAILABLE", modelStatus: "UNAVAILABLE", diagnostic: "AUDIT_MODEL_UNAVAILABLE", priorAuditId: null, remediationPromptId: null, remediationPromptVersionId: null, remediationSessionId: null, findings: [finding], coverage: [{ id: "coverage-1", requirementRef: "req-1", requirementText: "Required source symbol exists", status: "UNVERIFIED", evidenceRefs: ["SOURCE_SNIPPET:src/lib.rs"], rationale: "No configured model was available." }], evidence: [{ id: "evidence-1", kind: "SOURCE_SNIPPET", verificationStatus: "VERIFIED", locator: "src/lib.rs", summary: "Direct source inspection", content: "fn required_symbol() {}", contentSha256: "source-hash", byteCount: 24, truncated: false }, { id: "evidence-2", kind: "BUILDER_LOG_CLAIM", verificationStatus: "CLAIM_ONLY", locator: "M16.log", summary: "Builder claim", content: "all tests pass", contentSha256: "claim-hash", byteCount: 14, truncated: false }] };
const prompt = { promptId: "remediation-prompt", id: "remediation-version", version: 1, title: "Remediate audit", approvalState: "DRAFT", content: "Address the selected finding." };

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

beforeEach(() => {
  Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
  window.history.pushState({}, "", "/audits");
  invoke.mockReset();
  invoke.mockImplementation((command: string) => {
    if (command === "hiveai_projects_list") return Promise.resolve([project]);
    if (command === "hiveai_audits_list") return Promise.resolve([audit]);
    if (command === "hiveai_audit_get") return Promise.resolve(audit);
    if (command === "hiveai_audit_run") return Promise.resolve({ ...audit, id: "audit-2", priorAuditId: "audit-1", findings: [] });
    if (command === "hiveai_audit_create_remediation_prompt") return Promise.resolve(prompt);
    if (command === "hiveai_prompt_versions") return Promise.resolve([]);
    return Promise.resolve({});
  });
});

describe("M16 Audit Center", () => {
  it("renders structured verdict, model availability, coverage, and finding severity", async () => {
    render(<App />);
    expect(await screen.findByRole("heading", { name: "Audit Center" })).toBeInTheDocument();
    expect(await screen.findByText("Evidence collected; GPT provider unavailable.")).toBeInTheDocument();
    expect(screen.getByText(/GPT audit provider is not configured/)).toBeInTheDocument();
    expect(screen.getByText("Required source symbol exists")).toBeInTheDocument();
    expect(screen.getByText("MAJOR", { selector: ".audit-badge" })).toBeInTheDocument();
    expect(screen.getByText(/Builder logs are claims only/)).toBeInTheDocument();
    const technical = document.querySelector("details.audit-evidence-details");
    expect(technical).not.toHaveAttribute("open");
  });

  it("creates a bounded remediation draft for exactly the selected persisted finding", async () => {
    render(<App />);
    await screen.findByText(/Known defect/);
    fireEvent.click(screen.getByRole("checkbox"));
    fireEvent.click(screen.getByRole("button", { name: /Create remediation prompt/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_audit_create_remediation_prompt", { projectId: project.id, auditId: audit.id, findingIds: [finding.id], title: expect.any(String), summary: expect.any(String) }));
    expect(window.location.pathname).toBe("/prompts");
    expect(new URLSearchParams(window.location.search).get("auditId")).toBe(audit.id);
  });

  it("passes the selected immutable audit as the re-audit predecessor", async () => {
    render(<App />);
    await screen.findByText(/Known defect/);
    fireEvent.click(screen.getByRole("button", { name: /Re-audit/ }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_audit_run", { request: { projectId: project.id, taskId: null, priorAuditId: audit.id } }));
    expect(screen.getByText("No open findings")).toBeInTheDocument();
  });
});
