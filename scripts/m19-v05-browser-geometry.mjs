import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const root = process.cwd();
const chrome = process.env.CHROME_PATH ?? "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
if (!fs.existsSync(chrome)) throw new Error(`Chromium executable not found: ${chrome}`);

const css = ["src/styles.css", "src/registry-card.css"]
  .map((file) => fs.readFileSync(path.join(root, file), "utf8"))
  .join("\n");
const html = `<!doctype html><meta charset="utf-8"><style>
:root { --border:#334158; --subtle-border:#263247; --text:#edf3ff; --text-secondary:#c4cedf; --text-muted:#8996aa; --accent-strong:#9fc1ff; --danger:#f18a8a; --radius-md:8px; }
body { background:#0b0e13; color:var(--text); margin:0; padding:18px; font-family:Arial,sans-serif; }
${css}
</style><main><div class="registry-grid">
${["one", "two", "three"].map((id) => `<article class="registry-card"><div class="registry-card-top"><div class="registry-project-mark">PR</div><div class="registry-card-title"><div><h2>Project ${id}</h2><span class="registry-status registry-status-active">Active</span></div><button class="icon-button" type="button" aria-label="More actions for ${id}">•••</button></div></div><div class="registry-path"><span>C:\\Work\\${id}</span></div><div class="registry-meta-grid"><div><span>Status</span><strong>ACTIVE</strong></div><div><span>Tasks</span><strong>8</strong></div><div><span>Priority</span><strong>Normal</strong></div></div><div class="registry-card-foot"><button class="secondary-button" type="button">Open cockpit</button><div class="registry-icon-actions"><button class="secondary-button registry-workspace-action" type="button">Local workspace</button><button class="icon-button" type="button" aria-label="Archive ${id}">A</button><button class="icon-button registry-danger" type="button" aria-label="Remove ${id} from registry">X</button></div></div></article>`).join("")}
</div></main><script>
const inside=(a,b)=>a.left>=b.left&&a.top>=b.top&&a.right<=b.right&&a.bottom<=b.bottom;
const separated=(a,b)=>a.right<=b.left||b.right<=a.left||a.bottom<=b.top||b.bottom<=a.top;
const cards=[...document.querySelectorAll('.registry-card')];
const rows=cards.map((card)=>{const footer=card.querySelector('.registry-card-foot');const open=card.querySelector('.registry-card-foot > .secondary-button');const local=card.querySelector('.registry-workspace-action');const archive=card.querySelector('[aria-label^="Archive"]');const remove=card.querySelector('.registry-danger');open.focus();const c=card.getBoundingClientRect(),f=footer.getBoundingClientRect(),o=open.getBoundingClientRect(),l=local.getBoundingClientRect(),a=archive.getBoundingClientRect(),r=remove.getBoundingClientRect();return {footerInside:inside(f,c),openVisible:o.width>0&&o.height>0,localVisible:l.width>0&&l.height>0,archiveInside:inside(a,c),removeInside:inside(r,c),actionsSeparated:separated(a,r),focusNotClipped:getComputedStyle(card).overflow!=='hidden',containsWorkspace:card.textContent.includes('Local workspace'),forbiddenCopy:card.textContent.includes('Change local workspace')};});
const result={width:innerWidth,rows,documentOverflow:document.documentElement.scrollWidth<=innerWidth&&document.body.scrollWidth<=innerWidth,all:rows.every((row)=>Object.entries(row).filter(([key])=>key!=='forbiddenCopy').every(([,value])=>value===true))&&rows.every((row)=>!row.forbiddenCopy)};
const pre=document.createElement('pre');pre.id='m19-v05-geometry-result';pre.textContent=JSON.stringify(result);document.body.append(pre);
</script>`;

const temp = fs.mkdtempSync(path.join(os.tmpdir(), "hiveai-m19-v05-"));
const page = path.join(temp, "geometry.html");
fs.writeFileSync(page, html, "utf8");
try {
  for (const width of [1536, 900, 640]) {
    const result = spawnSync(chrome, [
      "--headless=new", "--disable-gpu", "--no-sandbox", "--hide-scrollbars",
      `--window-size=${width},900`, "--run-all-compositor-stages-before-draw",
      "--virtual-time-budget=1000", "--dump-dom", `file:///${page.replaceAll("\\", "/")}`,
    ], { encoding: "utf8", timeout: 15000, windowsHide: true });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`Chromium geometry run failed at ${width}px: ${result.stderr}`);
    const match = result.stdout.match(/<pre id="m19-v05-geometry-result">([\s\S]*?)<\/pre>/);
    if (!match) throw new Error(`Chromium geometry result missing at ${width}px`);
    const evidence = JSON.parse(match[1]);
    if (!evidence.all || !evidence.documentOverflow) throw new Error(`Geometry assertion failed at ${width}px: ${JSON.stringify(evidence)}`);
    console.log(JSON.stringify(evidence));
  }
} finally {
  fs.rmSync(temp, { recursive: true, force: true });
}
