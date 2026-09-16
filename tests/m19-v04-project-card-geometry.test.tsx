import { render } from "@testing-library/react";
import React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ProjectRegistryCard } from "../src/components/ProjectRegistryCard";
import type { ProjectRecord } from "../src/projectRegistry";

const project = (id: string, status: ProjectRecord["status"] = "ACTIVE"): ProjectRecord => ({
  id, name: `Project ${id}`, originalPath: `C:\\Work\\${id}`, normalizedPath: `c:\\work\\${id}`,
  status, priority: 0, preferredBuilder: "Codex", preferredAuditor: "Claude", taskSourcePolicy: "DISCOVER_STANDARD_FILES",
  registeredAt: "2026-09-16T00:00:00Z", lastValidatedAt: "2026-09-16T00:00:00Z", repository: null,
});

describe("M19 V04 mounted project-card geometry", () => {
  afterEach(() => vi.restoreAllMocks());

  it.each([1536, 900, 640])("keeps footer and delete containment at viewport width %s", (width) => {
    Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function () {
      if (this.classList.contains("registry-card")) return DOMRect.fromRect({ x: 0, y: 0, width: width >= 720 ? 480 : width - 24, height: 320 });
      if (this.classList.contains("registry-card-foot")) return DOMRect.fromRect({ x: 0, y: 280, width: width >= 720 ? 448 : width - 56, height: 32 });
      if (this.classList.contains("registry-danger")) return DOMRect.fromRect({ x: width >= 720 ? 410 : width - 100, y: 280, width: 31, height: 31 });
      return DOMRect.fromRect({ x: 0, y: 0, width: 1, height: 1 });
    });
    const view = render(<div className="registry-grid">{[project("one"), project("two", "MISSING"), project("three", "ARCHIVED")].map((item) => <ProjectRegistryCard key={item.id} project={item} onOpen={() => undefined} onArchive={() => undefined} onRemove={() => undefined} onRepair={() => undefined} onPriority={() => undefined} />)}</div>);
    expect(view.container.querySelectorAll(".registry-card")).toHaveLength(3);
    for (const card of view.container.querySelectorAll<HTMLElement>(".registry-card")) {
      const footer = card.querySelector<HTMLElement>(".registry-card-foot")!;
      const remove = card.querySelector<HTMLButtonElement>(".registry-danger")!;
      const cardBox = card.getBoundingClientRect();
      const footerBox = footer.getBoundingClientRect();
      const removeBox = remove.getBoundingClientRect();
      expect(card.querySelector(".registry-workspace-action")).toHaveTextContent("Local workspace");
      expect(footer.classList.contains("registry-card-foot")).toBe(true);
      expect(remove.getAttribute("aria-label")).toMatch(/^Remove Project /);
      expect(remove.classList.contains("registry-danger")).toBe(true);
      expect(footerBox.right).toBeLessThanOrEqual(cardBox.right);
      expect(removeBox.right).toBeLessThanOrEqual(footerBox.right);
    }
  });
});
