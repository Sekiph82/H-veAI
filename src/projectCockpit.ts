import { invoke } from "@tauri-apps/api/core";
import type { GitDiff, GitSnapshot } from "./gitEngine";
import type { ProjectRecord } from "./projectRegistry";
import type { DiscoveredProjectSource } from "./taskSources";
import type { TaskIntelligenceSnapshot } from "./taskIntelligence";
import type { WorkflowEvent, WorkflowTask } from "./workflow";
import type { ControlPlaneSnapshot } from "./controlPlane";
import type { GitHubTrackingSnapshot } from "./commandCenter";

export type CockpitTestRun = {
  id: string;
  taskId: string | null;
  command: string;
  result: string;
  outputMetadata: string | null;
  startedAt: string;
  finishedAt: string | null;
};

export type CockpitAuditFinding = {
  id: string;
  severity: string;
  title: string;
  detail: string | null;
  filePath: string | null;
  lineNumber: number | null;
  createdAt: string;
};

export type CockpitAudit = {
  id: string;
  taskId: string | null;
  result: string;
  summary: string | null;
  confidence: number | null;
  createdAt: string;
  findings: CockpitAuditFinding[];
};

export type CockpitAgentSession = {
  id: string;
  taskId: string | null;
  provider: string;
  state: string;
  startedAt: string | null;
  endedAt: string | null;
  createdAt: string;
};

export type CockpitPermission = {
  id: string;
  sessionId: string | null;
  permissionKind: string;
  requestedResource: string | null;
  state: string;
  decidedBy: string | null;
  createdAt: string;
  decidedAt: string | null;
};

export type CockpitActivity = {
  id: string;
  kind: string;
  event: string;
  state: string | null;
  actor: string | null;
  occurredAt: string;
  source: string;
};

export type CockpitFileEntry = {
  path: string;
  role: string;
  status: string;
  sourceKind: string | null;
  evidence: string;
};

export type RemoteTaskRow = {
  id: string;
  title: string;
  status: string;
  sourcePath: string;
  sourceLine: number;
};

export type ProjectDashboardResolution = {
  projectId: string;
  manifestStatus: string;
  manifestPath: string;
  schema: string | null;
  projectKey: string | null;
  repository: string | null;
  branchPolicy: string | null;
  dashboardMode: string | null;
  trackingMode: string | null;
  refreshPolicy: string | null;
  taskAuthority: string;
  canonicalTaskSource: string | null;
  roles: Record<string, Array<{ path: string; role: string; status: string; exists: boolean; contained: boolean }>>;
  provenanceMode: string;
  materialized: {
    projectStatus: string | null;
    health: string | null;
    currentMilestone: string | null;
    currentTaskTitle: string | null;
    currentTaskId: string | null;
    declaredWorkflowState: string | null;
    progressRaw: string | null;
    progressPercent: number | null;
    requiredActor: string | null;
    nextAction: string | null;
    waitingOn: string | null;
    lastMeaningfulUpdate: string | null;
    currentWork: Array<{ id: string; item: string; status: string; ownerActor: string; evidenceSource: string }>;
    blockersWaiting: string[];
    milestoneSummary: string[];
    qualityVerification: Array<{ label: string; value: string }>;
    recentMeaningfulActivity: string[];
    provenance: Array<{ label: string; value: string }>;
  };
  warnings: string[];
};

export type ProjectCockpitSnapshot = {
  project: ProjectRecord;
  projectSummary: {
    projectId: string;
    health: string;
    manifestStatus: string;
    taskAuthority: string;
    provenanceMode: string;
    currentTask: { taskId: string; title: string; sourcePath: string; parsedStatus: string; workflowState: string | null; requiredActor: string | null } | null;
    currentState: string | null;
    lastAction: { summary: string; occurredAt: string; actor: string | null } | null;
    nextAction: string | null;
    allowedActors: string[];
    totalTasks: number | null;
    activeTasks: number | null;
    completedTasks: number | null;
    progressPercent: number | null;
    warnings: string[];
    refreshStatus: string | null;
    refreshAt: string | null;
    refreshError: string | null;
  };
  dashboard: ProjectDashboardResolution;
  controlPlane?: ControlPlaneSnapshot;
  taskIntelligence: TaskIntelligenceSnapshot | null;
  taskIntelligenceError: string | null;
  workflow: { projectId: string; tasks: WorkflowTask[] };
  workflowHistory: WorkflowEvent[];
  git: GitSnapshot | null;
  gitError: string | null;
  gitDiff: GitDiff | null;
  gitDiffError: string | null;
  sources: DiscoveredProjectSource[];
  sourcesError: string | null;
  tests: CockpitTestRun[];
  audits: CockpitAudit[];
  agentSessions: CockpitAgentSession[];
  permissions: CockpitPermission[];
  activity: CockpitActivity[];
  files: CockpitFileEntry[];
  warnings: string[];
  generatedAt: string;
  githubTracking?: GitHubTrackingSnapshot;
  remoteTasks: RemoteTaskRow[];
  remotePrimary?: {
    repository: string;
    branch: string;
    remoteHead: string | null;
    remoteHealth: string;
    fetchedAt: string;
    updatedAt: string | null;
    updatedBy: string | null;
    currentMilestone: string | null;
    currentSprint: string | null;
    currentTaskId: string | null;
    currentTaskTitle: string | null;
    currentTaskStatus: string | null;
    workflowState: string | null;
    requiredActor: string | null;
    nextAction: string | null;
    blockers: string[];
    progressScopeType: string | null;
    progressScopeId: string | null;
    progressCompleted: number | null;
    progressTotal: number | null;
    progressPercent: number | null;
    lastCompletedTaskId: string | null;
    lastCompletedTaskTitle: string | null;
  };
  localWorkspaceTelemetry?: {
    localPath: string;
    localBranch: string | null;
    localHead: string | null;
    localHealth: string | null;
    warnings: string[];
    gitError: string | null;
    gitDiffError: string | null;
  };
};

export type RemoteTasksViewState =
  | "CURRENT_POPULATED"
  | "CURRENT_EMPTY"
  | "STALE_CACHED"
  | "STALE_EMPTY"
  | "DEGRADED_CACHED"
  | "REMOTE_UNAVAILABLE"
  | "STRUCTURALLY_INCONSISTENT";

export type RemoteTasksView = {
  state: RemoteTasksViewState;
  title: string;
  detail: string;
  showCachedWarning: boolean;
};

export function getRemoteTasksView(
  snapshot: Pick<ProjectCockpitSnapshot, "githubTracking" | "remoteTasks">,
): RemoteTasksView | null {
  const remote = snapshot.githubTracking;
  if (!remote) return null;

  const rows = snapshot.remoteTasks ?? [];
  const health = remote.remoteHealth.toUpperCase();
  if (health === "CURRENT") {
    if ((remote.totalTasks ?? 0) > 0 && rows.length === 0) {
      return {
        state: "STRUCTURALLY_INCONSISTENT",
        title: "Remote tasks need refresh",
        detail: `The remote snapshot reports ${remote.totalTasks} task(s) but contains no materialized rows.`,
        showCachedWarning: true,
      };
    }
    if ((remote.totalTasks ?? 0) === 0 && rows.length === 0) {
      return {
        state: "CURRENT_EMPTY",
        title: "No canonical remote tasks",
        detail: "GitHub root TASKS.md was observed successfully and contains no parseable task rows.",
        showCachedWarning: false,
      };
    }
    return {
      state: "CURRENT_POPULATED",
      title: "Canonical remote tasks",
      detail: `${rows.length} remote TASKS.md row(s)`,
      showCachedWarning: false,
    };
  }

  if (health === "STALE") {
    if (rows.length > 0) {
      return {
        state: "STALE_CACHED",
        title: "Showing stale remote tasks",
        detail: `${rows.length} cached remote TASKS.md row(s). ${remote.error ?? "A fresh GitHub observation is pending."}`,
        showCachedWarning: true,
      };
    }
    return {
      state: "STALE_EMPTY",
      title: "Remote tasks unavailable",
      detail: `The last remote snapshot is stale and contains no usable task rows. ${remote.error ?? "A fresh GitHub observation is pending."}`,
      showCachedWarning: true,
    };
  }

  if (rows.length > 0) {
    return {
      state: "DEGRADED_CACHED",
      title: "Showing cached remote tasks",
      detail: `${rows.length} cached remote TASKS.md row(s). ${remote.error ?? `GitHub remote status is ${health}.`}`,
      showCachedWarning: true,
    };
  }

  return {
    state: "REMOTE_UNAVAILABLE",
    title: "Remote tasks unavailable",
    detail: `GitHub remote observation is ${health.toLowerCase()}: ${remote.error ?? "no usable remote snapshot is available."}`,
    showCachedWarning: true,
  };
}

export const getProjectCockpitSnapshot = (projectId: string) =>
  invoke<ProjectCockpitSnapshot>("hiveai_project_cockpit_snapshot", { projectId });
