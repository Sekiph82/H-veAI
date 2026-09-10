import { invoke } from "@tauri-apps/api/core";

export type ControlPlaneGitState = {
  localStatus: string;
  remoteStatus: string;
  syncStatus: string;
  branch: string | null;
  headSha: string | null;
  upstream: string | null;
  ahead: number | null;
  behind: number | null;
  dirty: boolean;
  conflicted: boolean;
};

export type ControlPlaneSnapshot = {
  schema: string;
  projectId: string;
  projectKey: string | null;
  displayName: string;
  adopted: boolean;
  health: string;
  workflowState: string | null;
  canonicalTaskSource: string | null;
  currentTaskId: string | null;
  currentTaskTitle: string | null;
  currentMilestone: string | null;
  currentCycle: string | null;
  requiredActor: string | null;
  remoteRepository: string | null;
  autoFastForwardEnabled: boolean;
  nextAction: string | null;
  blockers: string[];
  progressPercent: number | null;
  resumePointer: string | null;
  eventCount: number;
  lastEventAt: string | null;
  git: ControlPlaneGitState;
  sourcePrecedence: string[];
  warnings: string[];
};

export type ControlPlaneSummary = {
  status: string;
  health: string;
  projectKey: string | null;
  workflowState: string | null;
  currentTaskId: string | null;
  nextAction: string | null;
  syncStatus: string;
  adopted: boolean;
};

export type GitSyncPlan = {
  action: string;
  safe: boolean;
  reason: string;
  fetchRequired: boolean;
  mergeRequired: boolean;
};

export type LocalRepairPlan = {
  action: string;
  safe: boolean;
  reason: string;
  preservesLocalData: boolean;
  targetPath: string | null;
};

export const getControlPlaneSnapshot = (projectId: string) =>
  invoke<ControlPlaneSnapshot>("hiveai_control_plane_snapshot", { projectId });

export const adoptControlPlane = (projectId: string) =>
  invoke<ControlPlaneSnapshot>("hiveai_control_plane_adopt", { projectId });

export const upgradeControlPlane = (projectId: string) =>
  invoke<ControlPlaneSnapshot>("hiveai_control_plane_upgrade", { projectId });

export const setControlPlaneAutoFastForward = (projectId: string, enabled: boolean) =>
  invoke<void>("hiveai_control_plane_set_auto_fast_forward", { projectId, enabled });

export const syncControlPlaneRemote = (projectId: string, autoFastForwardEnabled: boolean) =>
  invoke<GitSyncPlan>("hiveai_control_plane_sync_remote", { projectId, autoFastForwardEnabled });

export const getLocalRepairPlan = (projectId: string) =>
  invoke<LocalRepairPlan>("hiveai_control_plane_local_repair_plan", { projectId });
