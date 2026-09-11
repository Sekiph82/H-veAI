import { describe, expect, it } from "vitest";
import { getRemoteTasksView, type RemoteTaskRow } from "../src/projectCockpit";
import type { GitHubTrackingSnapshot } from "../src/commandCenter";

const row: RemoteTaskRow = {
  id: "M21-R02",
  title: "Repair remote task cache",
  status: "IN_PROGRESS",
  sourcePath: "TASKS.md",
  sourceLine: 12,
};

const remote = (overrides: Partial<GitHubTrackingSnapshot> = {}) =>
  ({
    projectKey: "github:Sekiph82/H-veAI@main",
    displayName: "H-veAI",
    repository: "Sekiph82/H-veAI",
    branch: "main",
    remoteHead: "head",
    totalTasks: 1,
    remoteHealth: "CURRENT",
    error: null,
    ...overrides,
  }) as GitHubTrackingSnapshot;

const view = (remoteSnapshot: GitHubTrackingSnapshot, rows: RemoteTaskRow[]) =>
  getRemoteTasksView({ githubTracking: remoteSnapshot, remoteTasks: rows });

describe("M21-R02 remote Tasks truthfulness", () => {
  it("renders current populated rows as canonical remote tasks", () => {
    expect(view(remote(), [row])).toMatchObject({
      state: "CURRENT_POPULATED",
      title: "Canonical remote tasks",
      detail: "1 remote TASKS.md row(s)",
      showCachedWarning: false,
    });
  });

  it("renders a confirmed current zero-task document as canonical empty", () => {
    expect(
      view(remote({ totalTasks: 0 }), []),
    ).toMatchObject({
      state: "CURRENT_EMPTY",
      title: "No canonical remote tasks",
      detail: "GitHub root TASKS.md was observed successfully and contains no parseable task rows.",
      showCachedWarning: false,
    });
  });

  it("labels stale cached rows as stale instead of current", () => {
    expect(
      view(remote({ remoteHealth: "STALE", error: "GitHub timeout" }), [row]),
    ).toMatchObject({
      state: "STALE_CACHED",
      title: "Showing stale remote tasks",
      showCachedWarning: true,
    });
  });

  it.each(["UNAVAILABLE", "ERROR"])(
    "labels %s with no rows as a remote failure, not canonical empty",
    (remoteHealth) => {
      const result = view(
        remote({ remoteHealth, totalTasks: null, error: "GitHub observation failed" }),
        [],
      );
      expect(result).toMatchObject({
        state: "REMOTE_UNAVAILABLE",
        title: "Remote tasks unavailable",
        showCachedWarning: true,
      });
      expect(result?.detail).toContain("GitHub observation failed");
    },
  );

  it("labels a populated count with no rows as structural inconsistency", () => {
    expect(
      view(remote({ totalTasks: 3 }), []),
    ).toMatchObject({
      state: "STRUCTURALLY_INCONSISTENT",
      title: "Remote tasks need refresh",
      showCachedWarning: true,
    });
  });
});
