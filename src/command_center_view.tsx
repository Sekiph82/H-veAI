import { ArrowUpRight, RefreshCw, Search, ShieldCheck } from "lucide-react";
import React from "react";
import { Link, useNavigate } from "react-router-dom";
import { DatabaseStatusPanel } from "./components/DatabaseStatusPanel";
import { RuntimeStatusPanel } from "./components/RuntimeStatusPanel";
import { WatcherStatusPanel } from "./components/WatcherStatusPanel";
import { LoadingState, MetricCard, SectionHeader } from "./components/ui";
import {
  getCommandCenterSnapshot,
  listenForGitHubTrackingUpdate,
  previewSnapshot,
  registryFallback,
  refreshGitHubTracking,
  selectGitHubTrackingProject,
  type CommandCenterSnapshot,
} from "./commandCenter";
import { isTauriDesktop } from "./projectRegistry";
import { useProjectRegistry } from "./registryContext";

const count = (value: number | null | undefined) => value == null ? "-" : String(value);

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
  const generation = React.useRef(0);
  const refresh = React.useCallback(() => {
    const currentGeneration = ++generation.current;
    if (!desktop) {
      setSnapshot(previewSnapshot());
      setLoading(false);
      return;
    }
    setLoading(true);
    void getCommandCenterSnapshot().then((next) => {
      if (currentGeneration !== generation.current) return;
      setSnapshot(next && next.projects ? next : registryFallback(records));
      setError(null);
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
  const data = snapshot && snapshot.kpis && Array.isArray(snapshot.projects) ? snapshot : (desktop ? registryFallback(records) : previewSnapshot());
  const current = data.projects.find((project) => project.projectId === selectedProjectId) ?? data.projects[0] ?? null;
  const visibleProjects = data.projects.slice(0, 8);
  const hiddenProjectCount = Math.max(0, data.projects.length - visibleProjects.length);
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
    void refreshGitHubTracking().then(() => refresh()).catch(() => refresh());
  }, [desktop, refresh]);
  return <div className="command-center" aria-label="Command Center overview">
    {error ? <div className="safe-notice" role="alert">{error}</div> : null}
    {data.warnings.slice(0, 3).map((warning) => <div className="safe-notice" role="alert" key={warning}>{warning}</div>)}
    <header className="command-heading"><div><h1>Global Overview</h1><h1 className="sr-only">Command Center</h1><span className="sr-only">Project operations</span></div><button className="secondary-button" type="button" onClick={manualRefresh} disabled={loading}><RefreshCw size={15} className={loading ? "spin" : undefined} /> Refresh</button></header>
    <section className="command-kpis" aria-label="Portfolio metrics"><MetricCard label="Projects" value={String(data.kpis.projects)} detail="GitHub-tracked portfolio" /><MetricCard label="Active tasks" value={count(data.kpis.activeTasks)} detail="Remote tracker exact counts" tone="blue" /><MetricCard label="Needs attention" value={count(data.kpis.needsAttention)} detail="Remote health and workflow" tone="warning" /><MetricCard label="Running" value={count(data.kpis.running)} detail="Remote workflow states" tone="running" /><MetricCard label="Completed tasks" value={count(data.kpis.completedTasks)} detail="Remote tracker exact counts" tone="audit" /><MetricCard label="Portfolio health" value={data.kpis.healthDetail} detail={data.kpis.authorityDetail} tone="external" /></section>
    <div className="command-layout">
      <section className="command-projects panel"><SectionHeader title="Projects" detail={`${data.projects.length} registered workspace${data.projects.length === 1 ? "" : "s"}`} action={<Link className="text-link" to="/projects">All <ArrowUpRight size={13} /></Link>} /><div className="project-rail">{visibleProjects.map((project) => <button className={project.projectId === selectedProjectId ? "project-rail-row project-rail-row-selected" : "project-rail-row"} type="button" key={project.projectId} aria-pressed={project.projectId === selectedProjectId} title={project.name} onClick={() => selectProject(project.projectId, true)}><strong>{project.name}</strong></button>)}{hiddenProjectCount ? <span className="rail-overflow">+{hiddenProjectCount} more projects</span> : null}{loading ? <LoadingState /> : !data.projects.length ? <span className="rail-empty">{desktop ? "No registered projects yet." : "Native project data unavailable in browser preview."}</span> : null}</div><button className="rail-footer" type="button" onClick={() => navigate("/projects")}>View all projects <ArrowUpRight size={13} /></button></section>
      <CommandCenterProjectPanel project={current} snapshot={data} emptyName={currentName} onOpen={() => current && navigate(`/projects/${encodeURIComponent(current.projectId)}`)} />
      <aside className="command-right-rail"><section className="right-panel brief-compact"><SectionHeader title="AI Engineering Brief" detail="Factual inputs" />{data.engineeringBrief.facts.map((fact) => <div className="brief-line" key={fact.label}><ShieldCheck size={15} /><div><strong>{fact.label}: {fact.value}</strong><small>{fact.source}{fact.provenance.sourcePath ? ` | ${fact.provenance.sourcePath}` : ` | ${fact.provenance.sourceClass}`}</small></div></div>)}{!data.engineeringBrief.facts.length ? <div className="brief-line">Native factual brief unavailable.</div> : null}</section><section className="right-panel assistant-compact" aria-label="Needs Your Attention"><SectionHeader title="Needs Your Attention" detail={`${data.attention.length} items`} />{data.attention.slice(0, 5).map((item) => <button className="attention-line" type="button" key={item.id} onClick={() => item.projectId && selectProject(item.projectId, true)}><strong>{item.projectName || "Portfolio evidence"}</strong><span>{item.state} | {item.title}</span></button>)}{!data.attention.length ? <div className="assistant-message">No attention items.</div> : null}</section><section className="right-panel queue-compact"><SectionHeader title="Active Work Queue" detail={`${data.workQueue.length} bounded items`} />{data.workQueue.slice(0, 5).map((item) => <div className="queue-mini-row" key={item.id}><strong>{item.projectName}</strong><span>{item.stage} | {item.task}</span></div>)}{!data.workQueue.length ? <div className="assistant-message">No active work evidence.</div> : null}</section><section className="right-panel system-compact"><SectionHeader title="System Status" detail="Native panels" /><div className="system-row"><span>Snapshot</span><b>{snapshot ? "Current" : "Unavailable"}</b></div><div className="system-row"><span>Warnings</span><b>{data.warnings.length}</b></div><details className="system-detail"><summary>Detailed health</summary><RuntimeStatusPanel /><DatabaseStatusPanel /><WatcherStatusPanel /></details></section></aside>
    </div>
    <section className="panel command-recent-activity" aria-label="Recent activity"><SectionHeader title="Recent activity" detail={`${data.recentActivity.length} bounded events`} /><div className="activity-list">{data.recentActivity.slice(0, 5).map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.projectName} | {item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div><span className="status-badge">{item.state ?? "EVIDENCE"}</span></div>)}{!data.recentActivity.length ? <div className="rail-empty">No recent activity evidence.</div> : null}</div><Link className="text-link" to="/activity">View activity <ArrowUpRight size={13} /></Link></section>
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
    <>{authorityNotice}<div className="cockpit-body"><div className="current-task"><div className="task-kicker">CURRENT TASK <span>{project.totalTasks == null ? "Unavailable" : `${project.completedTasks ?? 0} / ${project.totalTasks}`}</span></div><h3>{currentTask?.title ?? "Current task unavailable"}</h3><p>{project.nextAction ?? "No next action is declared by the remote tracker."}</p><div className="task-meta"><span>Health: {project.health}</span><span>{project.currentState ? `State: ${project.currentState}` : "Workflow state unavailable."}</span><span>Milestone: {remote?.currentMilestone ?? "Unavailable"}</span><span>Required actor: {remote?.requiredActor ?? "Unavailable"}</span></div></div><div className="workflow-mini"><div className="task-kicker">PROJECT STATUS</div><div className="workflow-step workflow-active"><span>1</span><div><strong>{project.health}</strong><small>{project.refreshStatus ?? "Remote snapshot unavailable"}</small></div></div></div></div></>
  ) : tab === "Tasks" ? (
    <div className="cockpit-body"><div className="metric-mini-grid"><span><b>{project.activeTasks == null ? "Unavailable" : project.activeTasks}</b>Active/open</span><span><b>{project.completedTasks == null ? "Unavailable" : project.completedTasks}</b>Completed</span><span><b>{project.totalTasks == null ? "Unavailable" : project.totalTasks}</b>Total</span><span><b>{project.progressPercent == null ? "Unavailable" : `${project.progressPercent}%`}</b>Progress</span></div><div className="subtask-list"><div><strong>Current task</strong><span>{currentTask?.title ?? "Unavailable"}</span></div><div><strong>Next action</strong><span>{project.nextAction ?? "Unavailable"}</span></div></div></div>
  ) : tab === "Workflow" ? (
    <div className="cockpit-body"><div className="current-task"><div className="task-kicker">WORKFLOW</div><h3>{project.currentState ?? remote?.workflowState ?? "Unavailable"}</h3><p>Required actor: {remote?.requiredActor ?? "Unavailable"}</p><div className="task-meta">{(remote?.blockers ?? []).length ? remote?.blockers.map((blocker) => <span key={blocker}>Blocker: {blocker}</span>) : <span>No remote blockers declared.</span>}</div></div><div className="subtask-list"><div>{project.nextAction ?? "No next action declared."}<b>Next action</b></div></div></div>
  ) : tab === "Audit" ? (
    <div className="cockpit-body"><div className="current-task"><div className="task-kicker">AUDIT EVIDENCE</div><h3>{project.health === "HEALTHY" ? "No current attention required" : project.health}</h3><p>{project.warnings.length ? project.warnings.join(" | ") : "No project-specific audit warning is present in the current remote snapshot."}</p></div></div>
  ) : (
    <div className="cockpit-body"><div className="compact-activity">{activity.map((item) => <div className="activity-row" key={item.id}><time>{item.occurredAt}</time><div className="activity-copy"><strong>{item.event}</strong><span>{item.kind}{item.actor ? ` | ${item.actor}` : ""}</span></div></div>)}{!activity.length ? <div className="rail-empty">No project activity evidence is available.</div> : null}</div></div>
  );
  return <section className="command-cockpit panel"><div className="cockpit-title"><div><div className="cockpit-inline-label"><span className="eyebrow">Current project</span><h2>{project?.name ?? emptyName ?? "No registered project"}</h2></div><span>{project ? `${project.registryStatus} | ${project.provenanceMode}` : "Native project identity unavailable"}</span></div><span className={`health-label health-${project?.health.toLowerCase() ?? "unknown"}`}>{project?.health ?? "UNKNOWN"}</span><button className="secondary-button cockpit-open" type="button" disabled={!project} onClick={onOpen}>Open cockpit <ArrowUpRight size={14} /></button></div><div className="cockpit-tabs" role="tablist" aria-label="Command Center project views">{(["Cockpit", "Tasks", "Workflow", "Audit", "Logs"] as CommandCenterTab[]).map((item) => <button key={item} type="button" role="tab" aria-selected={tab === item} className={tab === item ? "tab-active" : ""} onClick={() => setTab(item)}>{item}</button>)}</div>{tabContent}<div className="cockpit-bottom"><div><SectionHeader title="Remote source" detail="GitHub-authoritative" /><div className="compact-activity"><div className="activity-row"><div className="activity-copy"><strong>{remote?.repository ?? "Unavailable"}</strong><span>{remote?.branch ?? "Unavailable"} @ {remote?.remoteHead?.slice(0, 12) ?? "HEAD unavailable"}</span></div></div></div></div><div><SectionHeader title="Project metrics" detail="Current signal" /><div className="metric-mini-grid"><span><b>{count(project?.activeTasks)}</b>Active</span><span><b>{count(project?.completedTasks)}</b>Completed</span><span><b>{project?.progressPercent == null ? "-" : `${project.progressPercent}%`}</b>Completion</span><span><b>{project?.taskAuthority ?? "-"}</b>Authority</span></div></div></div></section>;
}
