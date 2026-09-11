import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ProjectRegistryCard } from "../src/components/ProjectRegistryCard";

const baseProject = {
  id: "github-project",
  name: "H!veAI",
  originalPath: "",
  normalizedPath: "",
  status: "ACTIVE" as const,
  priority: 0,
  preferredBuilder: null,
  preferredAuditor: null,
  taskSourcePolicy: "GITHUB_ROOT_TASKS",
  registeredAt: "now",
  lastValidatedAt: null,
  repository: {
    id: "repository",
    isGitRepository: true,
    repositoryRoot: null,
    currentBranch: "main",
    headSha: "abc",
    preferredRemoteUrl: "https://github.com/Sekiph82/H-veAI.git",
    defaultBranch: "main",
    githubOwner: "Sekiph82",
    githubRepo: "H-veAI",
    remotes: [],
  },
};

const callbacks = () => ({
  onOpen: vi.fn(),
  onArchive: vi.fn(),
  onRemove: vi.fn(),
  onRepair: vi.fn(),
  onPriority: vi.fn(),
});

describe("M16S local workspace attachment", () => {
  it("exposes Attach local workspace for an ACTIVE remote-only project", () => {
    const props = callbacks();
    render(<ProjectRegistryCard project={baseProject} {...props} />);
    const action = screen.getByRole("button", { name: "Attach local workspace for H!veAI" });
    fireEvent.click(action);
    expect(props.onRepair).toHaveBeenCalledTimes(1);
  });

  it("exposes Change local workspace when an ACTIVE project already has a path", () => {
    const props = callbacks();
    render(<ProjectRegistryCard project={{ ...baseProject, originalPath: "C:\\Projects\\H-veAI", normalizedPath: "c:\\projects\\h-veai" }} {...props} />);
    expect(screen.getByRole("button", { name: "Change local workspace for H!veAI" })).toBeInTheDocument();
  });
});
