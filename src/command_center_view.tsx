import { ArrowUpRight, RefreshCw, Search, ShieldCheck } from "lucide-react";
import React from "react";
import { Link, useNavigate } from "react-router-dom";
import { DatabaseStatusPanel } from "./components/DatabaseStatusPanel";
import { RuntimeStatusPanel } from "./components/RuntimeStatusPanel";
import { WatcherStatusPanel } from "./components/WatcherStatusPanel";
import { LoadingState, MetricCard, SectionHeader, formatPercent } from "./components/ui";
import {
  getCommandCenterSnapshot,
  listenForGitHubTrackingUpdate,
  recordNextBestTaskHistory,
  previewSnapshot,
  registryFallback,
  refreshGitHubTracking,
  selectGitHubTrackingProject,
  type CommandCenterSnapshot,
} from "./commandCenter";
import { isTauriDesktop } from "./projectRegistry";
import { useProjectRegistry } from "./registryContext";

const count = (value: number | null | undefined) => value == null ? "-" : String(value);

export function semanticAttentionKey(item: { projectId?: string; taskId?: string | null; category?: string; state?: string; title?: string; detail?: string; evidence?: string[]; issueKey?: string }) {
  if (item.issueKey?.trim()) return item.issueKey.trim().toLowerCase();
  const signal = `${item.category ?? ""} ${item.state ?? ""} ${item.detail ?? ""}`.toLowerCase();
  const family = item.evidence?.length || item.detail?.trim() ? "evidence" : signal.includes("rate limit") || signal.includes("github 403") || signal.includes("github 429")
    ? "rate_limit" : signal.includes("blocked") || signal.includes("blocker") || signal.includes("dependency")
      ? "blocker" : signal.includes("wait") || signal.includes("human")
        ? "wait" : signal.includes("fail") || signal.includes("error")
          ? "failure" : "issue";
  const evidence = (item.evidence ?? []).map((value) => value.toLowerCase().replace(/[^a-z0-9:/@._-]+/g, " ").trim()).filter(Boolean).join("|");
  const detail = (item.detail ?? "").toLowerCase().replace(/[^a-z0-9:/@._-]+/g, " ").trim();
  return `${item.projectId ?? ""}:${item.taskId ?? ""}:${family}:${evidence || detail || "no-source-evidence"}`;
}

function NextBestTaskPanel({ data }: { data: CommandCenterSnapshot }) {
  const m19 = data.engineeringBrief.m19 ?? data.m19;
  const recommendation = m19?.recommended;
  const semanticKey = semanticAttentionKey;
  const seen = new Set<string>();
  const legacyAttention = data.attention.filter((item) => { const key = semanticKey(item); if (seen.has(key)) return false; seen.add(key); return true; });
  const m19Attention = (m19?.attention ?? []).filter((item) => { const key = semanticKey(item); if (seen.has(key)) return false; seen.add(key); return true; });
  return <section className="right-panel next-task-compact" aria-label="M19 Engineering Brief">
    <SectionHeader title="M19 Engineering Brief" detail={m19 ? `${m19.candidateCount} eligible` : "Native factual inputs"} />
    {data.engineeringBrief.facts.map((fact) => <div className="brief-line" key={`fact-${fact.label}`}><ShieldCheck size={15} /><div><strong>{fact.label}: {fact.value}</strong><small>{fact.source}{fact.provenance.sourcePath ? ` | ${fact.provenance.sourcePath}` : ` | ${fact.provenance.sourceClass}`}</small></div></div>)}
    {m19 ? <><div className="brief-line"><ShieldCheck size={15} /><div><strong>Portfolio state</strong><small>{m19.activeProjects} active projects · comparison {m19.comparison.state}</small></div></div>
    {recommendation ? <>
      <div className="brief-line"><ShieldCheck size={15} /><div><strong>{recommendation.projectName} · {recommendation.taskTitle}</strong><small>{recommendation.taskId} · rank {recommendation.rank} · score {recommendation.score} · factual state {recommendation.factualState}</small></div></div>
      <p className="assistant-message">{recommendation.explanation}</p>
      <details className="system-detail"><summary>Why this is recommended</summary><div className="brief-line"><div><strong>Eligibility</strong><small>{recommendation.eligibilityReason}</small></div></div>{recommendation.scoreComponents.map((component) => <div className="brief-line" key={component.key}><div><strong>{component.label}: {component.points > 0 ? "+" : ""}{component.points}</strong><small>{component.evidence}</small></div></div>)}<div className="brief-line"><div><strong>Evidence</strong><small>{recommendation.evidence.join(" | ")}</small></div></div>{recommendation.uncertainty.length ? <div className="brief-line"><div><strong>Uncertainty</strong><small>{recommendation.uncertainty.join(" | ")}</small></div></div> : null}</details>
    </> : <div className="assistant-message">No eligible task can be recommended from current evidence.</div>}
    {m19.alternatives.length ? <details className="system-detail"><summary>Lower-ranked alternatives</summary>{m19.alternatives.map((alternative) => <div className="brief-line" key={`${alternative.projectId}:${alternative.taskId}`}><div><strong>#{alternative.rank} {alternative.projectName} · {alternative.taskTitle}</strong><small>{alternative.explanation}</small></div></div>)}</details> : null}
    {m19Attention.length ? <details className="system-detail" open><summary>M19 attention</summary>{m19Attention.slice(0, 8).map((item) => <div className="brief-line" key={semanticKey(item)}><div><strong>{item.projectName} · {item.title}</strong><small>{item.category} · {item.detail} · {item.evidence.join(" | ")}</small></div></div>)}</details> : null}
    {m19.unavailableInputs.length ? <div className="assistant-message">Unavailable inputs: {m19.unavailableInputs.join(" | ")}</div> : null}</> : <div className="assistant-message">M19 decision inputs unavailable; only native factual inputs are currently available.</div>}
    {legacyAttention.length ? <details className="system-detail"><summary>Portfolio attention</summary>{legacyAttention.slice(0, 8).map((item) => <div className="brief-line" key={item.id}><div><strong>{item.projectName || "Portfolio evidence"} · {item.title}</strong><small>{item.state} · {item.category}</small></div></div>)}</details> : null}
  </section>;
}

function Activity({ snapshot }: { snapshot: CommandCenterSnapshot }) {
  const [search, setSearch] = React.useState("");
  const [kind, setKind] = React.useState("ALL");
  const kinds = Array.from(new Set(snapshot.recentActivity.map((item) => item.kind)));
  const items = snapshot.recentActivity.filter((item) => {
    const text = `${item.projectName} ${item.event} ${item.kind} ${item.actor ?? ""}`.toLowerCase();
    return (!search || text.includes(search.toLowerCase())) && (kind === "ALL" || kind === item.kind);
  });
  return <section className="panel command-activity-filter">
    <SectionHeader title="Recent Activity" detail={`${items.length} bounded events`} />
    <div className="activity-filter-controls"><Search size={14} /><input aria-label="Search recent activity" value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Search activity" /><select aria-label="Filter activity type" value={kind} onChange={(event) => setKind(event.target.value)}><option value="ALL">All types</option>{kinds.map((value) => <option value={value} key={value}>{value}</option>)}</select></div>
    <div className="activity-list">{items.slice(0, 50).map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.projectName} | {item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div><span className="status-badge">{item.state ?? "EVIDENCE"}</span></div>)}{!items.length ? <div className="rail-empty">No matching activity evidence.</div> : null}</div>
  </section>;
}

export function CommandCenterLive() {
  const desktop = isTauriDesktop();
  const navigate = useNavigate();
  const { selectedProjectId, selectProject, records } = useProjectRegistry();
  const [snapshot, setSnapshot] = React.useState<CommandCenterSnapshot | null>(null);
  const [loading, setLoading] = React.useState(desktop);
  const [error, setError] = React.useState<string | null>(null);
  const [m19, setM19] = React.useState<CommandCenterSnapshot["m19"]>(undefined);
  const generation = React.useRef(0);
  const refresh = React.useCallback(() => {
    const currentGeneration = ++generation.current;
    if (!desktop) {
      setSnapshot(previewSnapshot());
      setM19(undefined);
      setLoading(false);
      return;
    }
    setLoading(true);
    setM19(undefined);
    void getCommandCenterSnapshot().then((next) => {
      if (currentGeneration !== generation.current) return;
      setSnapshot(next && next.projects ? next : registryFallback(records));
      setError(null);
      const nextBest = next?.engineeringBrief?.m19 ?? next?.m19;
      setM19(nextBest && Array.isArray(nextBest.unavailableInputs) && Array.isArray(nextBest.attention) ? nextBest : undefined);
    }).catch((caught) => {
      if (currentGeneration !== generation.current) return;
      setSnapshot(registryFallback(records));
      setError(caught instanceof Error ? caught.message : String(caught));
    }).finally(() => {
      if (currentGeneration === generation.current) setLoading(false);
    });
  }, [desktop, records]);
  React.useEffect(() => { refresh(); }, [refresh]);
  React.useEffect(() => {
    if (!desktop) return;
    const internals = (window as Window & { __TAURI_INTERNALS__?: { transformCallback?: unknown } }).__TAURI_INTERNALS__;
    if (typeof internals?.transformCallback !== "function") return;
    let active = true;
    let cleanup: (() => void) | undefined;
    void listenForGitHubTrackingUpdate(() => { if (active) refresh(); }).then((unlisten) => { if (active) cleanup = unlisten; else unlisten(); }).catch(() => undefined);
    return () => { active = false; cleanup?.(); };
  }, [desktop, refresh]);
  const baseData = snapshot && snapshot.kpis && Array.isArray(snapshot.projects) ? snapshot : (desktop ? registryFallback(records) : previewSnapshot());
  const data = m19 ? { ...baseData, m19 } : baseData;
  const current = data.projects.find((project) => project.projectId === selectedProjectId) ?? data.projects[0] ?? null;
  const visibleProjects = data.projects;
  const currentName = current?.name ?? (!desktop ? "Preview / Native data unavailable" : null);
  React.useEffect(() => {
    if (data.projects.length && (!selectedProjectId || !data.projects.some((project) => project.projectId === selectedProjectId))) selectProject(data.projects[0].projectId, true);
  }, [data.projects, selectedProjectId, selectProject]);
  React.useEffect(() => {
    if (desktop) void selectGitHubTrackingProject(selectedProjectId).catch(() => undefined);
  }, [desktop, selectedProjectId]);
  const manualRefresh = React.useCallback(() => {
    if (!desktop) {
      refresh();
      return;
    }
    setLoading(true);
    void refreshGitHubTracking()
      .then(() => recordNextBestTaskHistory())
      .then(() => refresh())
      .catch(() => refresh());
  }, [desktop, refresh]);
  return <div className="command-center" aria-label="Command Center overview">
    {error ? <div className="safe-notice" role="alert">{error}</div> : null}
    {data.warnings.slice(0, 3).map((warning) => <div className="safe-notice" role="alert" key={warning}>{warning}</div>)}
    <header className="command-heading"><div><h1>Global Overview</h1><h1 className="sr-only">Command Center</h1><span className="sr-only">Project operations</span></div><button className="secondary-button" type="button" onClick={manualRefresh} disabled={loading}><RefreshCw size={15} className={loading ? "spin" : undefined} /> Refresh</button></header>
    <section className="command-kpis" aria-label="Portfolio metrics"><MetricCard label="Projects" value={String(data.kpis.projects)} detail="GitHub-tracked portfolio" /><MetricCard label="Active tasks" value={count(data.kpis.activeTasks)} detail="Remote tracker exact counts" tone="blue" /><MetricCard label="Needs attention" value={count(data.kpis.needsAttention)} detail="Remote health and workflow" tone="warning" /><MetricCard label="Running" value={count(data.kpis.running)} detail="Remote workflow states" tone="running" /><MetricCard label="Completed tasks" value={count(data.kpis.completedTasks)} detail="Remote tracker exact counts" tone="audit" /><MetricCard label="Portfolio health" value={data.kpis.healthDetail} detail={data.kpis.authorityDetail} tone="external" /></section>
    <div className="command-layout">
      <section className="command-projects panel"><SectionHeader title="Projects" detail={`${data.projects.length} registered workspace${data.projects.length === 1 ? "" : "s"}`} action={<Link className="text-link" to="/projects">All <ArrowUpRight size={13} /></Link>} /><div className="project-rail">{visibleProjects.map((project) => <button className={project.projectId === selectedProjectId ? "project-rail-row project-rail-row-selected" : "project-rail-row"} type="button" key={project.projectId} aria-pressed={project.projectId === selectedProjectId} title={project.name} onClick={() => selectProject(project.projectId, true)}><strong>{project.name}</strong></button>)}{loading ? <LoadingState /> : !data.projects.length ? <span className="rail-empty">{desktop ? "No registered projects yet." : "Native project data unavailable in browser preview."}</span> : null}</div><button className="rail-footer" type="button" onClick={() => navigate("/projects")}>View all projects <ArrowUpRight size={13} /></button></section>
      <CommandCenterProjectPanel project={current} snapshot={data} emptyName={currentName} onOpen={() => current && navigate(`/projects/${encodeURIComponent(current.projectId)}`)} />
      <aside className="command-right-rail"><NextBestTaskPanel data={data} /><section className="right-panel queue-compact"><SectionHeader title="Active Work Queue" detail={`${data.workQueue.length} bounded items`} />{data.workQueue.slice(0, 5).map((item) => <div className="queue-mini-row" key={item.id}><strong>{item.projectName}</strong><span>{item.stage} | {item.task}</span></div>)}{!data.workQueue.length ? <div className="assistant-message">No active work evidence.</div> : null}</section><section className="right-panel system-compact"><SectionHeader title="System Status" detail="Native panels" /><div className="system-row"><span>Snapshot</span><b>{snapshot ? "Current" : "Unavailable"}</b></div><div className="system-row"><span>Warnings</span><b>{data.warnings.length}</b></div><details className="system-detail"><summary>Detailed health</summary><RuntimeStatusPanel /><DatabaseStatusPanel /><WatcherStatusPanel /></details></section></aside>
    </div>
   </div>;
}

type CommandCenterTab = "Cockpit" | "Tasks" | "Workflow" | "Audit" | "Logs";

function CommandCenterProjectPanel({
  project,
  snapshot,
  emptyName,
  onOpen,
}: {
  project: CommandCenterSnapshot["projects"][number] | null;
  snapshot: CommandCenterSnapshot;
  emptyName: string | null;
  onOpen: () => void;
}) {
  const [tab, setTab] = React.useState<CommandCenterTab>("Cockpit");
  React.useEffect(() => setTab("Cockpit"), [project?.projectId]);
  const activity = project
    ? snapshot.recentActivity.filter((item) => item.projectId === project.projectId).slice(0, 8)
    : [];
  const remote = project?.githubTracking;
  const currentTask = project?.currentTask;
  const authorityNotice = project?.taskAuthority === "NOT_CANONICALIZED" ? <div className="safe-notice" role="status">TASK AUTHORITY NOT YET CANONICALIZED</div> : null;
  const tabContent = !project ? (
    <div className="rail-empty">Select a registered project to inspect live project data.</div>
  ) : tab === "Cockpit" ? (
    <>{authorityNotice}<div className="cockpit-body"><div className="current-task"><div className="task-kicker">CURRENT TASK <span>{project.totalTasks == null ? "Unavailable" : `${project.completedTasks ?? 0} / ${project.totalTasks}`}</span></div><h3>{currentTask?.title ?? "Current task unavailable"}</h3><p>{project.nextAction ?? "No next action is declared by repository-root TASKS.md."}</p><div className="task-meta"><span>Health: {project.health}</span><span>{project.currentState ? `State: ${project.currentState}` : "Workflow state unavailable."}</span><span>Milestone: {project.currentMilestone ?? "Unavailable"}</span><span>Required actor: {project.requiredActor ?? "Unavailable"}</span></div></div><div className="workflow-mini"><div className="task-kicker">PROJECT STATUS</div><div className="workflow-step workflow-active"><span>1</span><div><strong>{project.health}</strong><small>{project.reconciliationState}</small></div></div></div></div></>
  ) : tab === "Tasks" ? (
    <div className="cockpit-body"><div className="metric-mini-grid"><span><b>{project.activeTasks == null ? "Unavailable" : project.activeTasks}</b>Active/open</span><span><b>{project.completedTasks == null ? "Unavailable" : project.completedTasks}</b>Completed</span><span><b>{project.totalTasks == null ? "Unavailable" : project.totalTasks}</b>Total</span><span><b>{project.progressPercent == null ? "Unavailable" : formatPercent(project.progressPercent)}</b>Progress</span></div><div className="subtask-list"><div><strong>Current task</strong><span>{currentTask?.title ?? "Unavailable"}</span></div><div><strong>Next action</strong><span>{project.nextAction ?? "Unavailable"}</span></div></div></div>
  ) : tab === "Workflow" ? (
    <div className="cockpit-body"><div className="current-task"><div className="task-kicker">WORKFLOW</div><h3>{project.currentState ?? "Unavailable"}</h3><p>Required actor: {project.requiredActor ?? "Unavailable"}</p><div className="task-meta">{project.blockers.length ? project.blockers.map((blocker) => <span key={blocker}>Blocker: {blocker}</span>) : <span>No current blockers declared.</span>}</div></div><div className="subtask-list"><div>{project.nextAction ?? "No next action declared by repository-root TASKS.md."}<b>Next action</b></div></div></div>
  ) : tab === "Audit" ? (
    <div className="cockpit-body"><div className="current-task"><div className="task-kicker">AUDIT EVIDENCE</div><h3>{project.health === "HEALTHY" ? "No current attention required" : project.health}</h3><p>{project.warnings.length ? project.warnings.join(" | ") : "No project-specific audit warning is present in the current remote snapshot."}</p></div></div>
  ) : (
    <div className="cockpit-body"><div className="compact-activity">{activity.map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div></div>)}{!activity.length ? <div className="rail-empty">No project activity evidence is available.</div> : null}</div></div>
  );
  return <section className="command-cockpit panel"><div className="cockpit-title"><div><div className="cockpit-inline-label"><span className="eyebrow">Current project</span><h2>{project?.name ?? emptyName ?? "No registered project"}</h2></div><span>{project ? `${project.registryStatus} | ${project.provenanceMode}` : "Native project identity unavailable"}</span></div><span className={`health-label health-${project?.health.toLowerCase() ?? "unknown"}`}>{project?.health ?? "UNKNOWN"}</span><button className="secondary-button cockpit-open" type="button" disabled={!project} onClick={onOpen}>Open cockpit <ArrowUpRight size={14} /></button></div><div className="cockpit-tabs" role="tablist" aria-label="Command Center project views">{(["Cockpit", "Tasks", "Workflow", "Audit", "Logs"] as CommandCenterTab[]).map((item) => <button key={item} type="button" role="tab" aria-selected={tab === item} className={tab === item ? "tab-active" : ""} onClick={() => setTab(item)}>{item}</button>)}</div>{tabContent}<div className="cockpit-bottom"><div><SectionHeader title="Remote source" detail="GitHub-authoritative" /><div className="compact-activity"><div className="activity-row"><div className="activity-copy"><strong>{remote?.repository ?? "Unavailable"}</strong><span>{remote?.branch ?? "Unavailable"} @ {remote?.remoteHead?.slice(0, 12) ?? "HEAD unavailable"}</span></div></div></div></div><div><SectionHeader title="Project metrics" detail="Current signal" /><div className="metric-mini-grid"><span><b>{count(project?.activeTasks)}</b>Active</span><span><b>{count(project?.completedTasks)}</b>Completed</span><span><b>{project?.progressPercent == null ? "-" : formatPercent(project.progressPercent)}</b>Completion</span><span><b>{project?.taskAuthority ?? "-"}</b>Authority</span></div></div></div></section>;
}
