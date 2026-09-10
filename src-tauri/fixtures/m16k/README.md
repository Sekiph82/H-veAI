# M16L Literal Portfolio Fixtures

These fixtures are refreshed from the current target branch refs, using the literal `.hiveai/PROJECT.json`, `.hiveai/RULES.md`, `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, and `.hiveai/EVENTS.jsonl` contracts. `targetBranchHeadSha` records the exact remote branch HEAD, while `projectBlobSha` and `rulesBlobSha` record contract-content provenance. They are test inputs only; the seven external repositories are not mutated by H!veAI tests.

The Level Factory row preserves its project-specific `governance.builderMayMutateHandoff: false` contract and owner-approved HANDOFF authority wording. The other rows preserve their current portable identity, canonical task source, state, handoff, event, and event-source shapes.

## Reproducible refresh

From a PowerShell prompt, fetch each target branch and inspect the exact source files without changing any target working tree:

```powershell
$targets = @(
  @('AI-Commerce-HQ', 'H!veAI'), @('Bulk-Edit', 'main'), @('fmcg-erp-system', 'main'),
  @('FormuLab', 'feature/laboratory-stability'), @('PackLab', 'main'),
  @('PackLab 3D', 'main'), @('ScrubBots', 'main'), @('ScrubBots - Pixel Art Generator', 'main')
)
foreach ($target in $targets) {
  Push-Location $target[0]
  git fetch origin $target[1]
  git show "origin/$($target[1]):.hiveai/PROJECT.json"
  git show "origin/$($target[1]):.hiveai/RULES.md"
  git show "origin/$($target[1]):.hiveai/STATE.json"
  git show "origin/$($target[1]):.hiveai/HANDOFF.md"
  git show "origin/$($target[1]):.hiveai/EVENTS.jsonl"
  git rev-parse "origin/$($target[1])"
  Pop-Location
}
```

Each row stores separate `targetBranchHeadSha`, `projectBlobSha`, and `rulesBlobSha` values. The checked-in fixture is updated only after this read-only refresh and all eight target heads plus contract blob identities are verified against the fetched refs.
