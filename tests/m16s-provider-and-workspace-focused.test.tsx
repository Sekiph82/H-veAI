import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";
import { ProjectRegistryCard } from "../src/components/ProjectRegistryCard";

const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

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
  beforeEach(() => {
    Object.defineProperty(window, "__TAURI_INTERNALS__", { configurable: true, value: {} });
    invoke.mockReset();
    invoke.mockImplementation((command: string) => {
      if (command === "hiveai_projects_list") return Promise.resolve([baseProject]);
      if (command === "hiveai_audit_provider_readiness") return Promise.resolve({ provider: "OpenAI", status: "CONFIGURED_UNVERIFIED", configured: true, model: "gpt-audit-test", credentialSource: "OPENAI_API_KEY environment", errorCategory: null });
      if (command === "hiveai_audit_provider_check_readiness") return Promise.resolve({ provider: "OpenAI", status: "AUTH_ERROR", configured: false, model: "gpt-audit-test", credentialSource: "OPENAI_API_KEY environment", errorCategory: "provider rejected the configured credential" });
      return Promise.resolve({});
    });
  });

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

  it("exposes Repair local workspace for a missing path", () => {
    const props = callbacks();
    render(<ProjectRegistryCard project={{ ...baseProject, originalPath: "C:\\Projects\\H-veAI", normalizedPath: "c:\\projects\\h-veAI", status: "MISSING" }} {...props} />);
    expect(screen.getByRole("button", { name: "Repair local workspace for H!veAI" })).toBeInTheDocument();
  });

  it("uses the explicit readiness command and keeps page load at configuration-only state", async () => {
    window.history.pushState({}, "", "/settings");
    render(<App />);
    expect(await screen.findByText("CONFIGURED_UNVERIFIED")).toBeInTheDocument();
    expect(invoke).not.toHaveBeenCalledWith("hiveai_audit_provider_check_readiness");
    fireEvent.click(screen.getByRole("button", { name: "Check readiness" }));
    await waitFor(() => expect(invoke).toHaveBeenCalledWith("hiveai_audit_provider_check_readiness"));
    expect(await screen.findByText("AUTH_ERROR")).toBeInTheDocument();
  });
});
