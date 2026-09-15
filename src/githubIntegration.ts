import { invoke } from "@tauri-apps/api/core";

export type GitHubIntegrationSnapshot = {
  projectId: string;
  repository: {
    owner: string;
    name: string;
    fullName: string;
    defaultBranch: string;
    trackedBranch: string;
    remoteHead: string | null;
    private: boolean | null;
    archived: boolean | null;
    htmlUrl: string | null;
    description: string | null;
  };
  branches: Array<{ name: string; sha: string | null; protected: boolean | null }>;
  commits: Array<{
    sha: string;
    message: string;
    author: string | null;
    authoredAt: string | null;
    committedAt: string | null;
    htmlUrl: string | null;
  }>;
  pullRequests: Array<{
    number: number;
    title: string;
    state: string;
    draft: boolean | null;
    merged: boolean;
    author: string | null;
    createdAt: string | null;
    updatedAt: string | null;
    sourceBranch: string | null;
    targetBranch: string | null;
    headSha: string | null;
    baseSha: string | null;
    htmlUrl: string | null;
    changedFiles: number | null;
    additions: number | null;
    deletions: number | null;
    reviewStatus: string | null;
    checkStatus: string | null;
    detailState: string;
    filesState: string;
    reviewsState: string;
    commentsState: string;
    checksState: string;
    files: Array<{ filename: string; status: string | null; additions: number | null; deletions: number | null; changes: number | null; patchExcerpt: string | null }>;
    reviews: Array<{ id: number; user: string | null; state: string | null; submittedAt: string | null; bodyExcerpt: string | null }>;
    checks: Array<{ id: number; name: string | null; status: string | null; conclusion: string | null; detailsUrl: string | null }>;
    comments: string[];
    rawTaskReferences: string[];
    rawSessionReferences: string[];
    taskLinks: string[];
    sessionLinks: string[];
  }>;
  issues: Array<{
    number: number;
    title: string;
    state: string;
    labels: string[];
    author: string | null;
    createdAt: string | null;
    updatedAt: string | null;
    htmlUrl: string | null;
    bodyExcerpt: string | null;
    comments: string[];
    rawTaskReferences: string[];
    rawSessionReferences: string[];
    taskLinks: string[];
    sessionLinks: string[];
  }>;
  actions: Array<{
    id: number;
    name: string | null;
    event: string | null;
    status: string | null;
    conclusion: string | null;
    branch: string | null;
    headSha: string | null;
    pullRequestNumbers: number[];
    createdAt: string | null;
    updatedAt: string | null;
    failedLogSummary: string | null;
    failedLogEvidence: Array<{ jobId: number; jobName: string | null; excerpt: string }>;
    jobsState: string;
    logsState: string;
    jobs: Array<{
      id: number;
      name: string | null;
      status: string | null;
      conclusion: string | null;
      steps: Array<{ name: string | null; status: string | null; conclusion: string | null; number: number | null }>;
    }>;
  }>;
  releases: Array<{
    id: number;
    name: string | null;
    tagName: string | null;
    targetCommitish: string | null;
    draft: boolean;
    prerelease: boolean;
    publishedAt: string | null;
    htmlUrl: string | null;
  }>;
  tags: Array<{ name: string; commitSha: string | null; protected: boolean | null }>;
  local: {
    available: boolean;
    branch: string | null;
    headSha: string | null;
    upstream: string | null;
    aheadCount: number | null;
    behindCount: number | null;
    detached: boolean;
    dirty: boolean;
    health: string | null;
    error: string | null;
  };
  reconciliation: {
    state: string;
    localBranch: string | null;
    localHead: string | null;
    remoteBranch: string;
    remoteHead: string | null;
    localDirty: boolean;
    evidence: string[];
  };
  remoteHealth: string;
  fetchedAt: string;
  cache: {
    schemaVersion: number;
    state: string;
    fetchedAt: string | null;
    lastKnownGoodAt: string | null;
    ageSeconds: number | null;
    provenance: string;
    resources: Array<{
      kind: string;
      state: string;
      fetchedAt: string | null;
      lastKnownGoodAt: string | null;
      error: string | null;
    }>;
  };
  mutationPolicy: {
    remoteMutations: string;
    pullRequestCreation: string;
    workflowRetry: string;
    localGitMutations: string;
    confirmationRequired: boolean;
  };
  warnings: string[];
};

const MAX_GITHUB_BRANCHES = 25;
const MAX_GITHUB_COMMITS = 25;
const MAX_GITHUB_PULL_REQUESTS = 10;
const MAX_GITHUB_ISSUES = 20;
const MAX_GITHUB_ACTIONS = 20;
const MAX_GITHUB_RELEASES = 10;
const MAX_GITHUB_TAGS = 25;
const MAX_GITHUB_WARNINGS = 32;
const MAX_GITHUB_RESOURCES = 8;
const MAX_GITHUB_PR_FILES = 25;
const MAX_GITHUB_PR_REVIEWS = 10;
const MAX_GITHUB_PR_CHECKS = 20;
const MAX_GITHUB_ACTION_JOBS = 20;
const MAX_GITHUB_ACTION_STEPS = 25;
const MAX_GITHUB_ACTION_LOG_EVIDENCE = 20;

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);
const isString = (value: unknown): value is string => typeof value === "string";
const isBoolean = (value: unknown): value is boolean => typeof value === "boolean";
const isNumber = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);
const isNullableString = (value: unknown): value is string | null => value === null || isString(value);
const isNullableBoolean = (value: unknown): value is boolean | null => value === null || isBoolean(value);
const isNullableNumber = (value: unknown): value is number | null => value === null || isNumber(value);
const isStringArray = (value: unknown, max: number): value is string[] =>
  Array.isArray(value) && value.length <= max && value.every(isString);
const isBoundedArray = <T>(value: unknown, max: number, guard: (item: unknown) => item is T): value is T[] =>
  Array.isArray(value) && value.length <= max && value.every(guard);

const isRepository = (value: unknown): value is GitHubIntegrationSnapshot["repository"] =>
  isRecord(value) && isString(value.owner) && isString(value.name) && isString(value.fullName) &&
  isString(value.defaultBranch) && isString(value.trackedBranch) && isNullableString(value.remoteHead) &&
  isNullableBoolean(value.private) && isNullableBoolean(value.archived) && isNullableString(value.htmlUrl) &&
  isNullableString(value.description);

const isReconciliation = (value: unknown): value is GitHubIntegrationSnapshot["reconciliation"] =>
  isRecord(value) && isString(value.state) && isNullableString(value.localBranch) &&
  isNullableString(value.localHead) && isString(value.remoteBranch) && isNullableString(value.remoteHead) &&
  isBoolean(value.localDirty) && isStringArray(value.evidence, 32);

const isBranch = (value: unknown): value is GitHubIntegrationSnapshot["branches"][number] =>
  isRecord(value) && isString(value.name) && isNullableString(value.sha) && isNullableBoolean(value.protected);

const isCommit = (value: unknown): value is GitHubIntegrationSnapshot["commits"][number] =>
  isRecord(value) && isString(value.sha) && isString(value.message) && isNullableString(value.author) &&
  isNullableString(value.authoredAt) && isNullableString(value.committedAt) && isNullableString(value.htmlUrl);

const isPullRequestFile = (value: unknown): value is GitHubIntegrationSnapshot["pullRequests"][number]["files"][number] =>
  isRecord(value) && isString(value.filename) && isNullableString(value.status) && isNullableNumber(value.additions) &&
  isNullableNumber(value.deletions) && isNullableNumber(value.changes) && isNullableString(value.patchExcerpt);

const isPullRequestReview = (value: unknown): value is GitHubIntegrationSnapshot["pullRequests"][number]["reviews"][number] =>
  isRecord(value) && isNumber(value.id) && isNullableString(value.user) && isNullableString(value.state) &&
  isNullableString(value.submittedAt) && isNullableString(value.bodyExcerpt);

const isCheck = (value: unknown): value is GitHubIntegrationSnapshot["pullRequests"][number]["checks"][number] =>
  isRecord(value) && isNumber(value.id) && isNullableString(value.name) && isNullableString(value.status) &&
  isNullableString(value.conclusion) && isNullableString(value.detailsUrl);

const isPullRequest = (value: unknown): value is GitHubIntegrationSnapshot["pullRequests"][number] =>
  isRecord(value) && isNumber(value.number) && isString(value.title) && isString(value.state) &&
  isNullableBoolean(value.draft) && isBoolean(value.merged) && isNullableString(value.author) &&
  isNullableString(value.createdAt) && isNullableString(value.updatedAt) && isNullableString(value.sourceBranch) &&
  isNullableString(value.targetBranch) && isNullableString(value.headSha) && isNullableString(value.baseSha) &&
  isNullableString(value.htmlUrl) && isNullableNumber(value.changedFiles) && isNullableNumber(value.additions) &&
  isNullableNumber(value.deletions) && isNullableString(value.reviewStatus) && isNullableString(value.checkStatus) &&
  isString(value.detailState) && isString(value.filesState) && isString(value.reviewsState) &&
  isString(value.commentsState) && isString(value.checksState) &&
  isBoundedArray(value.files, MAX_GITHUB_PR_FILES, isPullRequestFile) &&
  isBoundedArray(value.reviews, MAX_GITHUB_PR_REVIEWS, isPullRequestReview) &&
  isBoundedArray(value.checks, MAX_GITHUB_PR_CHECKS, isCheck) && isStringArray(value.comments, 10) &&
  isStringArray(value.rawTaskReferences, 32) && isStringArray(value.rawSessionReferences, 32) &&
  isStringArray(value.taskLinks, 32) && isStringArray(value.sessionLinks, 32);

const isIssue = (value: unknown): value is GitHubIntegrationSnapshot["issues"][number] =>
  isRecord(value) && isNumber(value.number) && isString(value.title) && isString(value.state) &&
  isStringArray(value.labels, 20) && isNullableString(value.author) && isNullableString(value.createdAt) &&
  isNullableString(value.updatedAt) && isNullableString(value.htmlUrl) && isNullableString(value.bodyExcerpt) &&
  isStringArray(value.comments, 10) && isStringArray(value.rawTaskReferences, 32) &&
  isStringArray(value.rawSessionReferences, 32) && isStringArray(value.taskLinks, 32) &&
  isStringArray(value.sessionLinks, 32);

const isActionStep = (value: unknown): value is GitHubIntegrationSnapshot["actions"][number]["jobs"][number]["steps"][number] =>
  isRecord(value) && isNullableString(value.name) && isNullableString(value.status) &&
  isNullableString(value.conclusion) && isNullableNumber(value.number);

const isActionJob = (value: unknown): value is GitHubIntegrationSnapshot["actions"][number]["jobs"][number] =>
  isRecord(value) && isNumber(value.id) && isNullableString(value.name) && isNullableString(value.status) &&
  isNullableString(value.conclusion) && isBoundedArray(value.steps, MAX_GITHUB_ACTION_STEPS, isActionStep);

const isActionLogEvidence = (value: unknown): value is GitHubIntegrationSnapshot["actions"][number]["failedLogEvidence"][number] =>
  isRecord(value) && isNumber(value.jobId) && isNullableString(value.jobName) && isString(value.excerpt);

const isAction = (value: unknown): value is GitHubIntegrationSnapshot["actions"][number] =>
  isRecord(value) && isNumber(value.id) && isNullableString(value.name) && isNullableString(value.event) &&
  isNullableString(value.status) && isNullableString(value.conclusion) && isNullableString(value.branch) &&
  isNullableString(value.headSha) && isBoundedArray(value.pullRequestNumbers, 20, isNumber) &&
  isNullableString(value.createdAt) && isNullableString(value.updatedAt) && isNullableString(value.failedLogSummary) &&
  isBoundedArray(value.failedLogEvidence, MAX_GITHUB_ACTION_LOG_EVIDENCE, isActionLogEvidence) &&
  isString(value.jobsState) && isString(value.logsState) && isBoundedArray(value.jobs, MAX_GITHUB_ACTION_JOBS, isActionJob);

const isRelease = (value: unknown): value is GitHubIntegrationSnapshot["releases"][number] =>
  isRecord(value) && isNumber(value.id) && isNullableString(value.name) && isNullableString(value.tagName) &&
  isNullableString(value.targetCommitish) && isBoolean(value.draft) && isBoolean(value.prerelease) &&
  isNullableString(value.publishedAt) && isNullableString(value.htmlUrl);

const isTag = (value: unknown): value is GitHubIntegrationSnapshot["tags"][number] =>
  isRecord(value) && isString(value.name) && isNullableString(value.commitSha) && isNullableBoolean(value.protected);

const isLocalEvidence = (value: unknown): value is GitHubIntegrationSnapshot["local"] =>
  isRecord(value) && isBoolean(value.available) && isNullableString(value.branch) && isNullableString(value.headSha) &&
  isNullableString(value.upstream) && isNullableNumber(value.aheadCount) && isNullableNumber(value.behindCount) &&
  isBoolean(value.detached) && isBoolean(value.dirty) && isNullableString(value.health) && isNullableString(value.error);

const isCacheResource = (value: unknown): value is GitHubIntegrationSnapshot["cache"]["resources"][number] =>
  isRecord(value) && isString(value.kind) && isString(value.state) && isNullableString(value.fetchedAt) &&
  isNullableString(value.lastKnownGoodAt) && isNullableString(value.error);

const isCache = (value: unknown): value is GitHubIntegrationSnapshot["cache"] =>
  isRecord(value) && isNumber(value.schemaVersion) && isString(value.state) && isNullableString(value.fetchedAt) &&
  isNullableString(value.lastKnownGoodAt) && isNullableNumber(value.ageSeconds) && isString(value.provenance) &&
  isBoundedArray(value.resources, MAX_GITHUB_RESOURCES, isCacheResource);

const isMutationPolicy = (value: unknown): value is GitHubIntegrationSnapshot["mutationPolicy"] =>
  isRecord(value) && isString(value.remoteMutations) && isString(value.pullRequestCreation) &&
  isString(value.workflowRetry) && isString(value.localGitMutations) && isBoolean(value.confirmationRequired);

export function isValidGitHubIntegrationSnapshot(value: unknown): value is GitHubIntegrationSnapshot {
  return isRecord(value) && isString(value.projectId) && isRepository(value.repository) &&
    isBoundedArray(value.branches, MAX_GITHUB_BRANCHES, isBranch) &&
    isBoundedArray(value.commits, MAX_GITHUB_COMMITS, isCommit) &&
    isBoundedArray(value.pullRequests, MAX_GITHUB_PULL_REQUESTS, isPullRequest) &&
    isBoundedArray(value.issues, MAX_GITHUB_ISSUES, isIssue) &&
    isBoundedArray(value.actions, MAX_GITHUB_ACTIONS, isAction) &&
    isBoundedArray(value.releases, MAX_GITHUB_RELEASES, isRelease) &&
    isBoundedArray(value.tags, MAX_GITHUB_TAGS, isTag) && isLocalEvidence(value.local) &&
    isReconciliation(value.reconciliation) && isString(value.remoteHealth) && isString(value.fetchedAt) &&
    isCache(value.cache) && isMutationPolicy(value.mutationPolicy) && isStringArray(value.warnings, MAX_GITHUB_WARNINGS);
}

export const getGitHubIntegrationSnapshot = (projectId: string) =>
  invoke<GitHubIntegrationSnapshot>("hiveai_github_integration_snapshot", { projectId });
