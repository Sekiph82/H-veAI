import React from "react";
import { ArrowUpRight, Check, ChevronRight, RefreshCw, ShieldCheck, Sparkles } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { EmptyState, ErrorState, LoadingState, PageHeader, SectionHeader } from "./components/ui";
import { useProjectRegistry } from "./registryContext";
import { isTauriDesktop } from "./projectRegistry";
import { createRemediationPrompt, getAudit, listAudits, runAudit, type AuditFinding, type AuditGitScope, type AuditRun, type AuditTargetOrigin } from "./auditEngine";
import { listAgentSessions, type AgentSession } from "./agentSessionCenter";
import { listWorkflowTasks, type WorkflowTask } from "./workflow";
import { TaskPicker } from "./TaskPicker";

function statusLabel(value: string) { return value.replaceAll("_", " "); }
function AuditBadge({ value }: { value: string }) { return <span className={`audit-badge audit-badge-${value.toLowerCase()}`}>{statusLabel(value)}</span>; }
function findingKey(finding: AuditFinding) { return `${finding.severity}: ${finding.title}`; }

export function AuditCenterPage() {
  const navigate = useNavigate();
  const { records } = useProjectRegistry();
  const active = records.filter((record) => record.status === "ACTIVE");
  const desktop = isTauriDesktop();
  const [projectId, setProjectId] = React.useState(active[0]?.id ?? "");
  const [taskId, setTaskId] = React.useState("");
  const [tasks, setTasks] = React.useState<WorkflowTask[]>([]);
  const [gitScope, setGitScope] = React.useState<AuditGitScope>("WORKING_TREE");
  const [baseRef, setBaseRef] = React.useState("");
  const [headSha, setHeadSha] = React.useState("");
  const [targetPreset, setTargetPreset] = React.useState<"WORKING_TREE" | "STAGED" | "AGENT_SESSION" | "PRIOR_AUDIT" | "MANUAL">("WORKING_TREE");
  const [sessions, setSessions] = React.useState<AgentSession[]>([]);
  const [sessionId, setSessionId] = React.useState("");
  const [audits, setAudits] = React.useState<AuditRun[]>([]);
  const [selectedId, setSelectedId] = React.useState<string | null>(null);
  const [selectedFindings, setSelectedFindings] = React.useState<string[]>([]);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [notice, setNotice] = React.useState<string | null>(null);
  const selected = audits.find((audit) => audit.id === selectedId) ?? audits[0] ?? null;

  const load = React.useCallback(async () => {
    if (!desktop || !projectId) return;
    setLoading(true); setError(null);
    try { const next = await listAudits(projectId); setAudits(next); setSelectedId((current) => current && next.some((audit) => audit.id === current) ? current : next[0]?.id ?? null); }
    catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); }
    finally { setLoading(false); }
  }, [desktop, projectId]);
  React.useEffect(() => { if (!active.some((record) => record.id === projectId)) setProjectId(active[0]?.id ?? ""); }, [active, projectId]);
  React.useEffect(() => { if (!desktop || !projectId) return; void listWorkflowTasks(projectId).then((result) => setTasks(result.tasks ?? [])).catch(() => setTasks([])); }, [desktop, projectId]);
  React.useEffect(() => { if (!desktop || !projectId) return; void listAgentSessions(projectId).then((result) => setSessions(Array.isArray(result) ? result : [])).catch(() => setSessions([])); }, [desktop, projectId]);
  React.useEffect(() => { void load(); }, [load]);
  React.useEffect(() => { setSelectedFindings([]); }, [selectedId]);

  const startAudit = async (priorAuditId: string | null = null) => {
    if (!desktop || !projectId) return;
    setLoading(true); setError(null); setNotice(null);
    try {
      const targetAudit = priorAuditId ? selected : targetPreset === "PRIOR_AUDIT" ? selected : null;
      const session = targetPreset === "AGENT_SESSION" ? sessions.find((item) => item.id === sessionId) ?? null : null;
      const targetOrigin: AuditTargetOrigin = priorAuditId || targetPreset === "PRIOR_AUDIT" ? "PRIOR_AUDIT" : targetPreset === "AGENT_SESSION" ? "AGENT_SESSION" : targetPreset === "STAGED" ? "REGISTERED_POLICY" : targetPreset === "MANUAL" ? "MANUAL" : "AUTO";
      const effectiveScope = targetAudit?.gitScope ?? (targetPreset === "STAGED" ? "STAGED" : targetPreset === "MANUAL" ? gitScope : "WORKING_TREE");
      const gitTarget = priorAuditId || (targetPreset === "WORKING_TREE" && !baseRef.trim() && !headSha.trim()) ? undefined : {
        scope: effectiveScope,
        baseRef: effectiveScope === "COMMIT_RANGE" ? (targetAudit?.baselineRef ?? baseRef) || null : null,
        headSha: effectiveScope === "COMMIT_RANGE" && targetPreset === "MANUAL" ? headSha || null : null,
        targetOrigin,
        auditedSessionId: session?.id ?? targetAudit?.auditedSessionId ?? null,
        auditedPromptVersionId: session?.promptVersionId ?? targetAudit?.auditedPromptVersionId ?? null,
      };
      const next = await runAudit(projectId, priorAuditId ? targetAudit?.taskId ?? null : taskId || null, priorAuditId, gitTarget);
      setAudits((current) => [next, ...current.filter((audit) => audit.id !== next.id)]); setSelectedId(next.id); setNotice(next.modelStatus === "UNAVAILABLE" ? "Evidence collected. Codex CLI audit provider is unavailable; no PASS was claimed." : "Audit completed with structured evidence.");
    }
    catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); }
    finally { setLoading(false); }
  };
  const refreshSelected = async () => { if (!selected) return; setLoading(true); try { const next = await getAudit(projectId, selected.id); setAudits((current) => current.map((audit) => audit.id === next.id ? next : audit)); } catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); } finally { setLoading(false); } };
  const createPrompt = async () => {
    if (!selected || !selectedFindings.length) return;
    setLoading(true); setError(null);
    try { const prompt = await createRemediationPrompt(projectId, selected.id, selectedFindings, `Remediate audit ${selected.id.slice(0, 8)}`, `Address only the selected persisted findings from audit ${selected.id}. Review and approve before any provider dispatch.`); setNotice("Remediation draft created. Human review is required before dispatch."); navigate(`/prompts?projectId=${encodeURIComponent(projectId)}&promptId=${encodeURIComponent(prompt.promptId)}&auditId=${encodeURIComponent(selected.id)}`); }
    catch (caught) { setError(caught instanceof Error ? caught.message : String(caught)); }
    finally { setLoading(false); }
  };
  if (!desktop) return <><PageHeader title="Audit Center" description="Independent review status across your projects." /><div className="fixture-note">Native H!veAI is required for evidence collection and audit persistence.</div><EmptyState title="Native audit unavailable" detail="Open the stable H!veAI desktop build to collect registered-project evidence." /></>;
  if (!active.length) return <><PageHeader title="Audit Center" description="Independent review status across your projects." /><EmptyState title="No active project" detail="Register an active project before starting an audit." /></>;
  return <>
    <PageHeader title="Audit Center" description="Evidence-first review for registered projects and tasks." action={<button className="secondary-button" type="button" onClick={() => void load()} disabled={loading}><RefreshCw size={15} /> Refresh evidence</button>} />
    {error ? <ErrorState detail={error} /> : null}{notice ? <div className="safe-notice" role="status"><Check size={15} />{notice}</div> : null}
    <div className="audit-layout">
      <section className="panel audit-target-panel"><SectionHeader title="Audit target" detail="Registered ACTIVE project authority" /><label>Project<select aria-label="Audit project" value={projectId} onChange={(event) => { setProjectId(event.target.value); setTaskId(""); setAudits([]); setSelectedId(null); }} disabled={loading}>{active.map((record) => <option value={record.id} key={record.id}>{record.name}</option>)}</select></label><TaskPicker tasks={tasks} value={taskId} onChange={setTaskId} disabled={loading || !projectId} ariaLabel="Audit task" /><label>Target source<select aria-label="Audit target source" value={targetPreset} onChange={(event) => setTargetPreset(event.target.value as typeof targetPreset)} disabled={loading}><option value="WORKING_TREE">Current working tree</option><option value="STAGED">Staged changes</option><option value="AGENT_SESSION">Implementation session</option><option value="PRIOR_AUDIT">Previous audit - current remediation</option><option value="MANUAL">Manual commit range (advanced)</option></select></label>{targetPreset === "AGENT_SESSION" ? <label>Agent session<select aria-label="Audit agent session" value={sessionId} onChange={(event) => setSessionId(event.target.value)} disabled={loading}><option value="">Select a session</option>{sessions.filter((session) => session.projectId === projectId).map((session) => <option value={session.id} key={session.id}>{session.provider} {session.id.slice(0, 8)} - {session.state}</option>)}</select></label> : null}{targetPreset === "MANUAL" ? <><label>Git scope<select aria-label="Audit git scope" value={gitScope} onChange={(event) => setGitScope(event.target.value as AuditGitScope)} disabled={loading}><option value="COMMIT_RANGE">Commit range</option><option value="WORKING_TREE">Working tree</option><option value="STAGED">Staged changes</option></select></label><label>Base ref<input aria-label="Audit base ref" value={baseRef} onChange={(event) => setBaseRef(event.target.value)} maxLength={256} placeholder="Required branch or commit" disabled={loading} /></label><label>Head SHA<input aria-label="Audit head SHA" value={headSha} onChange={(event) => setHeadSha(event.target.value)} maxLength={64} placeholder="Current HEAD when empty" disabled={loading} /></label></> : null}<div className="audit-target-actions"><button className="primary-button" type="button" onClick={() => void startAudit()} disabled={loading || !projectId || (targetPreset === "MANUAL" && gitScope === "COMMIT_RANGE" && !baseRef.trim()) || (targetPreset === "AGENT_SESSION" && !sessionId)}><ShieldCheck size={15} /> Start audit</button>{selected ? <button className="secondary-button" type="button" onClick={() => void startAudit(selected.id)} disabled={loading}><RefreshCw size={15} /> Re-audit</button> : null}</div><div className="audit-authority-note">Builder logs are claims only. Git, source, task, and test evidence come from native authorities.</div></section>
      <section className="panel audit-verdict-panel"><SectionHeader title="Current verdict" detail={selected ? `Audit ${selected.id.slice(0, 12)}` : "No audit selected"} />{loading && !selected ? <LoadingState label="Collecting bounded evidence..." /> : selected ? <><div className="audit-verdict-heading"><AuditBadge value={selected.verdict} /><AuditBadge value={selected.state} /></div><p className="audit-summary">{selected.summary}</p><dl className="audit-facts"><div><dt>Confidence</dt><dd><AuditBadge value={selected.confidence} /></dd></div><div><dt>Regression risk</dt><dd><AuditBadge value={selected.regressionRisk} /></dd></div><div><dt>Scope</dt><dd>{selected.gitScope ?? "WORKING_TREE"}</dd></div><div><dt>Target origin</dt><dd>{selected.targetOrigin ?? "AUTO"}</dd></div><div><dt>HEAD</dt><dd title={selected.auditedHeadSha ?? "Unavailable"}>{selected.auditedHeadSha ?? "Unavailable"}</dd></div><div><dt>Base</dt><dd title={selected.auditedBaseSha ?? selected.baselineRef ?? "Unavailable"}>{selected.auditedBaseSha ?? selected.baselineRef ?? "Unavailable"}</dd></div><div><dt>Model</dt><dd>{selected.modelStatus === "UNAVAILABLE" ? "UNAVAILABLE" : `${selected.auditorProvider ?? "Configured"} ${selected.auditorModel ?? ""}`}</dd></div><div><dt>Created</dt><dd>{selected.finishedAt ?? selected.startedAt}</dd></div></dl>{selected.task ? <div className="audit-provenance-note">Task identity {selected.task.identityStatus ?? "UNAVAILABLE"} · requirements {selected.task.requirementsStatus ?? "UNAVAILABLE"}</div> : null}{selected.auditedSessionId || selected.auditedPromptVersionId ? <div className="audit-provenance-note">Audited session {selected.auditedSessionId ?? "unavailable"} · prompt version {selected.auditedPromptVersionId ?? "unavailable"}</div> : null}{selected.modelStatus === "UNAVAILABLE" ? <div className="audit-unavailable">Codex CLI audit provider is not configured. Evidence remains visible, but no model verdict is treated as PASS.</div> : null}</> : <EmptyState title="No audit yet" detail="Start an evidence-first audit for the selected registered project." />}</section>
      <section className="panel audit-coverage-panel"><SectionHeader title="Requirement coverage" detail={selected ? `${selected.coverage.length} bounded rows` : "Awaiting audit evidence"} />{selected?.coverage.length ? <div className="audit-coverage-list">{selected.coverage.map((row) => <div className="audit-coverage-row" key={row.id}><div><strong>{row.requirementRef}</strong><span>{row.requirementText}</span><small>{row.rationale}</small></div><AuditBadge value={row.status} /></div>)}</div> : <EmptyState title="No coverage rows" detail="Task-specific criteria are marked unavailable for project audits or pending model evidence." />}</section>
      <section className="panel audit-findings-panel"><SectionHeader title="Findings" detail={selected ? `${selected.findings.length} persisted findings` : "No findings"} />{selected?.findings.length ? <><div className="audit-selection-toolbar"><span>{selectedFindings.length} selected</span><button className="primary-button" type="button" onClick={() => void createPrompt()} disabled={loading || !selectedFindings.length}><Sparkles size={15} /> Create remediation prompt</button></div><div className="audit-findings-list">{selected.findings.map((finding) => <label className={`audit-finding audit-finding-${finding.severity.toLowerCase()}`} key={finding.id}><input type="checkbox" checked={selectedFindings.includes(finding.id)} onChange={(event) => setSelectedFindings((current) => event.target.checked ? [...current, finding.id] : current.filter((id) => id !== finding.id))} disabled={finding.status !== "OPEN" || loading} /><div><div className="audit-finding-title"><AuditBadge value={finding.severity} /><strong>{findingKey(finding)}</strong></div><p>{finding.detail}</p><span>{finding.status} · {finding.confidence} confidence{finding.blocksRelease ? " · blocks release" : ""}</span><small>{finding.remediationGuidance}</small></div></label>)}</div></> : <EmptyState title="No open findings" detail="A configured model will populate structured findings; unavailable evidence is never silently promoted." />}</section>
      <section className="panel audit-history-panel"><SectionHeader title="Audit history" detail="Immutable runs and re-audit chain" />{audits.length ? <div className="audit-history-list">{audits.map((audit) => <button className={`audit-history-row${audit.id === selected?.id ? " audit-history-row-selected" : ""}`} type="button" key={audit.id} onClick={() => setSelectedId(audit.id)}><span><AuditBadge value={audit.verdict} /><strong>{audit.taskId ?? "Project audit"}</strong></span><span>{audit.auditedHeadSha?.slice(0, 12) ?? "HEAD unavailable"}</span><ChevronRight size={15} /></button>)}</div> : <EmptyState title="No audit history" detail="Previous immutable audit runs will appear here." />} {selected?.remediationSessionId ? <div className="audit-linked-session" role="status"><span>Linked remediation session</span><code>{selected.remediationSessionId}</code><button className="secondary-button" type="button" onClick={() => navigate(`/agents?projectId=${encodeURIComponent(projectId)}&sessionId=${encodeURIComponent(selected.remediationSessionId!)}`)}><ArrowUpRight size={15} /> Open in Agents</button></div> : null} {selected ? <details className="audit-evidence-details"><summary>Technical evidence and model diagnostics</summary><div className="audit-evidence-list">{selected.evidence.map((item) => <details key={item.id}><summary><AuditBadge value={item.verificationStatus} /> {item.kind} · {item.locator ?? "no locator"}</summary><p>{item.summary} · logical identity {item.logicalEvidenceId ?? "unavailable"}</p>{item.content ? <pre>{item.content}</pre> : null}</details>)}</div></details> : null}<button className="secondary-button audit-refresh-button" type="button" onClick={() => void refreshSelected()} disabled={!selected || loading}><RefreshCw size={15} /> Reload selected audit</button></section>
    </div>
  </>;
}
