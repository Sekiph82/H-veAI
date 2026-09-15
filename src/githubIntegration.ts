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

export const getGitHubIntegrationSnapshot = (projectId: string) =>
  invoke<GitHubIntegrationSnapshot>("hiveai_github_integration_snapshot", { projectId });
