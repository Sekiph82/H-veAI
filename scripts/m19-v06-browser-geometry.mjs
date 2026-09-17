import { spawn } from "node:child_process";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import process from "node:process";
import { once } from "node:events";
import WebSocket from "ws";

const port = 5174;
const chromePath = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
const shim = String.raw`(() => {
  const records = [
    { id: "geometry-active", name: "Active Geometry", originalPath: "C:\\Work\\Active", normalizedPath: "c:\\work\\active", status: "ACTIVE", priority: 1, preferredBuilder: "Codex", preferredAuditor: "Audit", taskSourcePolicy: "GITHUB_TASKS_ONLY", registeredAt: "2026-09-17T00:00:00Z", lastValidatedAt: "2026-09-17T00:00:00Z", repository: { id: "repo-active", isGitRepository: true, repositoryRoot: "C:\\Work\\Active", currentBranch: "main", headSha: "head-active", preferredRemoteUrl: "https://github.com/example/active", defaultBranch: "main", githubOwner: "example", githubRepo: "active", remotes: [] } },
    { id: "geometry-missing", name: "Missing Geometry", originalPath: "C:\\Work\\Missing", normalizedPath: "", status: "MISSING", priority: 2, preferredBuilder: "Codex", preferredAuditor: null, taskSourcePolicy: "GITHUB_TASKS_ONLY", registeredAt: "2026-09-17T00:00:00Z", lastValidatedAt: null, repository: { id: "repo-missing", isGitRepository: true, repositoryRoot: null, currentBranch: "main", headSha: null, preferredRemoteUrl: "https://github.com/example/missing", defaultBranch: "main", githubOwner: "example", githubRepo: "missing", remotes: [] } },
    { id: "geometry-third", name: "Third Geometry", originalPath: "C:\\Work\\Third", normalizedPath: "c:\\work\\third", status: "ACTIVE", priority: 0, preferredBuilder: null, preferredAuditor: null, taskSourcePolicy: "GITHUB_TASKS_ONLY", registeredAt: "2026-09-17T00:00:00Z", lastValidatedAt: null, repository: { id: "repo-third", isGitRepository: true, repositoryRoot: "C:\\Work\\Third", currentBranch: "main", headSha: "head-third", preferredRemoteUrl: "https://github.com/example/third", defaultBranch: "main", githubOwner: "example", githubRepo: "third", remotes: [] } }
  ];
  const callbacks = new Map();
  let callbackId = 0;
  const invoke = async (command, args) => {
    if (command === "hiveai_projects_list") {
      const query = args?.query ?? {};
      let value = [...records];
      if (query.status) value = value.filter((record) => record.status === query.status);
      if (!query.includeArchived) value = value.filter((record) => record.status !== "ARCHIVED");
      if (query.search) value = value.filter((record) => record.name.toLowerCase().includes(String(query.search).toLowerCase()));
      return value;
    }
    if (command === "hiveai_frontend_ready") return null;
    if (command === "plugin:event|listen") return ++callbackId;
    if (command === "plugin:event|unlisten") return null;
    return null;
  };
  window.__TAURI_INTERNALS__ = {
    invoke,
    transformCallback(callback) { const id = ++callbackId; callbacks.set(id, callback); return id; },
    unregisterCallback(id) { callbacks.delete(id); }
  };
  window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener() {} };
})();`;

function wait(ms) { return new Promise((resolve) => setTimeout(resolve, ms)); }

async function waitForUrl(url, timeoutMs = 15000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try { const response = await fetch(url); if (response.ok) return await response.json(); } catch {}
    await wait(100);
  }
  throw new Error(`Timed out waiting for ${url}`);
}

function cdp(ws) {
  let nextId = 0;
  const pending = new Map();
  ws.on("message", (raw) => {
    const message = JSON.parse(raw.toString());
    if (message.id && pending.has(message.id)) {
      const { resolve, reject } = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) reject(new Error(message.error.message)); else resolve(message.result);
    }
  });
  return (method, params = {}) => new Promise((resolve, reject) => {
    const id = ++nextId;
    pending.set(id, { resolve, reject });
    ws.send(JSON.stringify({ id, method, params }));
  });
}

async function geometryProbe() {
  const deadline = Date.now() + 15000;
  while (!document.querySelectorAll('article[data-testid^="registry-card-"]').length && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 50));
  const cards = [...document.querySelectorAll('article[data-testid^="registry-card-"]')];
  if (cards.length !== 3) throw new Error(`expected 3 real React registry cards, got ${cards.length}`);
  const failures = [];
  const contained = (outer, inner) => inner.left >= outer.left - 1 && inner.right <= outer.right + 1 && inner.top >= outer.top - 1 && inner.bottom <= outer.bottom + 1;
  const focusVisualIsUnclipped = (element) => {
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const outlineWidth = style.outlineStyle === "none" ? 0 : (parseFloat(style.outlineWidth) || 0);
    const visual = {
      left: rect.left - outlineWidth - 1,
      top: rect.top - outlineWidth - 1,
      right: rect.right + outlineWidth + 1,
      bottom: rect.bottom + outlineWidth + 1,
    };
    for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
      const ancestorStyle = getComputedStyle(ancestor);
      const clipsX = ["hidden", "clip", "scroll", "auto"].includes(ancestorStyle.overflowX);
      const clipsY = ["hidden", "clip", "scroll", "auto"].includes(ancestorStyle.overflowY);
      if (!clipsX && !clipsY) continue;
      const ancestorRect = ancestor.getBoundingClientRect();
      if ((clipsX && (visual.left < ancestorRect.left - 1 || visual.right > ancestorRect.right + 1)) ||
          (clipsY && (visual.top < ancestorRect.top - 1 || visual.bottom > ancestorRect.bottom + 1))) {
        return { ancestor: ancestor.tagName.toLowerCase(), overflowX: ancestorStyle.overflowX, overflowY: ancestorStyle.overflowY };
      }
    }
    return null;
  };
  for (const card of cards) {
    const footer = card.querySelector('.registry-card-foot');
    const footerRect = footer.getBoundingClientRect();
    for (const button of footer.querySelectorAll('button')) if (!contained(footerRect, button.getBoundingClientRect())) failures.push(`${card.dataset.testid}:footer-containment`);
    const actions = [...footer.querySelectorAll('.icon-button')].map((button) => button.getBoundingClientRect());
    for (let index = 1; index < actions.length; index++) if (actions[index - 1].right > actions[index].left + 1 && actions[index - 1].bottom > actions[index].top + 1) failures.push(`${card.dataset.testid}:action-overlap`);
    const workspace = footer.querySelector('.registry-workspace-action');
    if (!workspace || !workspace.textContent.includes('Local workspace') || workspace.getBoundingClientRect().width <= 0) failures.push(`${card.dataset.testid}:workspace-label`);
    const open = footer.querySelector('.secondary-button');
    if (!open || open.getBoundingClientRect().width <= 0) failures.push(`${card.dataset.testid}:open-cockpit`);
  }
  if (document.documentElement.scrollWidth > window.innerWidth || document.body.scrollWidth > window.innerWidth) failures.push('document-horizontal-overflow');
  const workspace = cards[0].querySelector('.registry-workspace-action');
  workspace.focus();
  const style = getComputedStyle(workspace);
  if (document.activeElement !== workspace || workspace.getBoundingClientRect().width <= 0 || (style.outlineStyle === 'none' && style.boxShadow === 'none')) failures.push('keyboard-focus-visibility');
  const clipping = focusVisualIsUnclipped(workspace);
  if (clipping) failures.push(`focus-visual-clipped:${JSON.stringify(clipping)}`);
  return { width: window.innerWidth, height: window.innerHeight, cards: cards.length, failures };
}

async function run() {
  const [width, height] = process.env.HIVEAI_GEOMETRY_VIEWPORT.split("x").map(Number);
  const profile = await mkdtemp(join(tmpdir(), "hiveai-v06-geometry-"));
  const vite = spawn(process.execPath, [join(process.cwd(), "node_modules/vite/bin/vite.js"), "--host", "127.0.0.1", "--port", String(port)], { cwd: process.cwd(), windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
  let viteOutput = "";
  vite.stdout.on("data", (chunk) => { viteOutput += chunk.toString(); });
  vite.stderr.on("data", (chunk) => { viteOutput += chunk.toString(); });
  let chrome;
  try {
    await new Promise((resolve, reject) => {
      const deadline = setTimeout(() => reject(new Error(`Timed out waiting for Vite; output=${viteOutput}`)), 15000);
      const poll = setInterval(() => {
        if (viteOutput.includes("Local:")) { clearTimeout(deadline); clearInterval(poll); resolve(); }
        else if (vite.exitCode !== null) { clearTimeout(deadline); clearInterval(poll); reject(new Error(`Vite exited ${vite.exitCode}; output=${viteOutput}`)); }
      }, 50);
    });
    chrome = spawn(chromePath, ["--headless=new", "--disable-gpu", "--no-sandbox", `--window-size=${width},${height}`, `--remote-debugging-port=9229`, `--user-data-dir=${profile}`, "about:blank"], { windowsHide: true, stdio: "ignore" });
    const targets = await waitForUrl("http://127.0.0.1:9229/json");
    const target = targets.find((item) => item.type === "page");
    if (!target?.webSocketDebuggerUrl) throw new Error("Chrome page target unavailable");
    const ws = new WebSocket(target.webSocketDebuggerUrl);
    await new Promise((resolve, reject) => { ws.once("open", resolve); ws.once("error", reject); });
    const send = cdp(ws);
    await send("Page.addScriptToEvaluateOnNewDocument", { source: shim });
    await send("Page.enable");
    await send("Runtime.enable");
    await send("Page.navigate", { url: `http://127.0.0.1:${port}/projects` });
    const result = await send("Runtime.evaluate", { awaitPromise: true, returnByValue: true, expression: `(${geometryProbe.toString()})()` });
    const value = result.result?.value;
    if (!value || typeof value.width !== "number") throw new Error(`geometry probe returned no value: ${JSON.stringify(result)}`);
    if (value?.failures?.length) throw new Error(JSON.stringify(value));
    console.log(JSON.stringify({ route: "/projects", surface: "production React Projects", viewport: value }));
    ws.close();
  } finally {
    if (chrome && !chrome.killed) { chrome.kill(); await once(chrome, "exit").catch(() => undefined); }
    if (vite && !vite.killed) { vite.kill(); await once(vite, "exit").catch(() => undefined); }
    for (let attempt = 0; attempt < 5; attempt++) {
      try { await rm(profile, { recursive: true, force: true }); break; }
      catch (error) { if (attempt === 4) throw error; await wait(100); }
    }
  }
}

for (const [width, height] of [[1536, 900], [900, 900], [640, 900]]) {
  process.env.HIVEAI_GEOMETRY_VIEWPORT = `${width}x${height}`;
  // The page uses the real responsive CSS; launch a fresh target per viewport.
  // CDP defaults are intentionally overridden by Chrome's --window-size below.
  // eslint-disable-next-line no-await-in-loop
  await run();
}
