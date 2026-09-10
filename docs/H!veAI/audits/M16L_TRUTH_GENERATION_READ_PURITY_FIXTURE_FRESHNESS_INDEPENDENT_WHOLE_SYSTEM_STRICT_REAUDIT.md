# M16L Truth-Generation Bootstrap / Read-Purity / Portfolio Fixture Freshness — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16L_TRUTH_GENERATION_BOOTSTRAP_READ_PURITY_PORTFOLIO_FIXTURE_FRESHNESS_CLOSURE_LOG.md`  
Implementation commit reviewed: `042f5f3dfa520218a67021b562e60bfd5726612e`  
Final branch HEAD reviewed: `4d78d51128d4f818a2d7cdff08a7282f811c0802`

## Verdict

**FAIL / CHANGES REQUIRED**

### M16L named findings
- UCP-R36: PARTIALLY CLOSED
- UCP-R37: PARTIALLY CLOSED
- UCP-R38: PARTIALLY CLOSED

### New whole-system findings
- BLOCKER: 1
- MAJOR: 2
- MINOR: 1

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

M16L fixes the literal generation-zero CAS dead-end and removes the direct SNAPSHOT_READ materialization path for already-current rows. It also refreshes the portfolio fixture shape substantially. The remaining failure is not the old 0<0 bug itself. The blocker is that the v22 bootstrap depends on a stale DB adoption flag that the current adoption/read path no longer maintains.

---

# Confirmed progress

## UCP-R36 — literal generation-zero CAS dead-end is fixed for rows selected by v22

Migration v22 can move selected ACTIVE/ADOPTED generation-zero rows to:

- truth_generation = 1
- truth_materialized_generation = 0
- truth_sync_status = PENDING

Typed CAS outcomes now distinguish Applied / AlreadyCurrent / Superseded / InvalidGeneration.

## UCP-R37 — direct control-plane snapshot path is observational for a fully current row

`snapshot(...)` checks `truth_sync_is_current(...)` and skips materialization when status/current generation/error state is already clean.

## UCP-R38 — fixture shapes are materially fresher

The checked-in fixture now uses the final control-plane schema and correct repository owner/name/canonical-task shapes for the eight projects.

---

# UCP-R39 — BLOCKER
## v22 generation bootstrap trusts DB `control_plane_status='ADOPTED'`, but the current production adoption/physical-resolution path does not keep that DB field authoritative

Migration v18 introduced:

`control_plane_status TEXT NOT NULL DEFAULT 'UNADOPTED'`

Migration v22 bootstraps only rows satisfying:

`control_plane_status = 'ADOPTED'`

However current production `adopt(...)`:

1. creates PROJECT/STATE/HANDOFF/EVENTS/RULES files;
2. marks truth dirty;
3. materializes;
4. returns snapshot;

but does not persist `control_plane_status='ADOPTED'`.

M16L also removed snapshot-time metadata persistence in order to make reads pure.

Current physical adoption is therefore determined by `resolve_project(...)` from the actual files, while v22 determines bootstrap eligibility from a potentially stale DB flag.

### Failure mode

A project can have a fully valid adopted `.hiveai/PROJECT.json` on disk while its pre-existing DB row still says:

`control_plane_status = UNADOPTED`

Then migration v22 leaves:

- truth_generation = 0
- truth_materialized_generation = 0
- truth_sync_status = CURRENT

After startup, `snapshot(...)` sees that truth-sync tuple as CURRENT and skips materialization.

So the project can remain permanently outside the generation bootstrap even though the repository is physically adopted.

This is especially relevant to the real portfolio because the control-plane files were migrated over multiple prior M16 runs while DB metadata was historically changing across parser/remediation versions.

### Required remediation

Make adoption identity physically authoritative and DB metadata convergent.

Preferred approach:

1. add a migration/startup reconciliation step that derives adoption from the actual bounded PROJECT.json contract;
2. for ACTIVE roots with a valid `hiveai-project-control-plane/v1` PROJECT.json, persist DB control_plane_status/schema deterministically;
3. then bootstrap generation if no durable generation has ever been materialized.

Also:

- `adopt(...)` must persist its DB adoption metadata transactionally with adoption dirty intent;
- an already-adopted physical project with stale DB UNADOPTED must self-heal on startup/reconcile without user re-registration;
- a physically malformed/missing project must not be upgraded to ADOPTED merely from stale DB state.

Add exact stale-DB fixtures:

A. physical adopted + DB UNADOPTED + generation 0  
B. physical malformed + DB ADOPTED  
C. physical missing + DB ADOPTED  
D. new adopt path persists ADOPTED transactionally

The authoritative invariant is:

> DB adoption metadata is a projection of the verified physical control-plane contract, never an independent stale truth source.

---

# UCP-R40 — MAJOR
## The claimed “current target SHA” fixture is already stale for AI-Commerce-HQ at final branch state

The M16L fixture/log records AI-Commerce-HQ source SHA:

`d8bcc9cbccdd3844f99e15298b1e7a9aec9adfa4`

That is the starting HEAD.

The actual final H!veAI branch reviewed after the run is:

`4d78d51128d4f818a2d7cdff08a7282f811c0802`

and the implementation commit itself is:

`042f5f3dfa520218a67021b562e60bfd5726612e`.

The builder log nevertheless states:

> Eight fetched remote refs matched the fixture source SHA table exactly after a second freshness check.

That statement cannot describe the final H!veAI target ref for AI-Commerce-HQ.

The underlying PROJECT contract may be byte-identical across those commits, but `sourceSha` is documented as the exact target branch HEAD used for the current fixture and the prompt required final live target verification.

### Required remediation

Separate two different identities:

- `targetBranchHeadSha`: current target branch HEAD at verification time
- `contractBlobSha` or `contractSourceCommitSha`: provenance for the copied control-plane contract content

Do not overload one `sourceSha` field with both meanings.

For AI-Commerce-HQ, final verification must happen after implementation commit exists. The final log-only commit may be newer, but the contract verification must at least point to the actual implementation-era target head and state clearly when the later immutable-log commit does not change the contract files.

For all eight projects, record both branch head and contract content identity.

---

# UCP-R41 — MAJOR
## The advertised 100-read Command Center / Cockpit purity test does not exercise the real adopted project through those surfaces

The M16L log says:

> 100 control snapshots, 100 Command Center reads, and 100 Project Cockpit ... reads

The direct test actually does:

- 100 real `control_plane::snapshot` calls on the adopted test project;
- 100 Command Center snapshots against a separate **empty database**;
- 100 Project Cockpit calls against that empty database using `missing-project`, asserting only that they error.

Therefore it does not prove that 100 Command Center/Cockpit reads of the **same adopted current project** are observational.

The production paths may currently be pure, but the release-gate claim is not directly verified.

### Required remediation

Use one adopted real project fixture.

1. create/register/adopt project;
2. fully materialize it CURRENT;
3. record STATE/HANDOFF/EVENTS/event-index bytes and truth-sync DB row;
4. call Command Center 100 times and select that project;
5. call Project Cockpit 100 times for that exact project;
6. assert all bytes and truth-sync/generation/retry state are unchanged.

Also verify no Git snapshot persistence, project metadata write, task refresh, event append, or watcher mutation is triggered by these read paths.

---

# UCP-R42 — MINOR
## Canonical tracking “Current truth” text is stale after the builder modified tracking documents

M16L modified TASKS/ROADMAP/README tracking, but the canonical TASKS `Current truth` prose still says M16I is the implementation-complete current remediation and separately references M16H/M16I while omitting the latest K/L chain in that summary.

The detailed M16L section exists and M16 remains correctly OPEN, so this does not change milestone acceptance. But the file's own current-truth summary is internally stale.

### Required remediation

Update the prospective/current-status summary to point to the latest implementation-complete remediation without declaring PASS/CLOSED.

Do not erase historical remediation sections.

---

# Required closure strategy

Close R39-R42 in one continuous run.

Do not reopen or rewrite already closed historical M16 findings unless a direct regression fails.

Required final invariants:

1. physical adopted contract and DB adoption metadata converge on startup/reconcile;
2. stale DB UNADOPTED cannot prevent truth-generation bootstrap;
3. stale DB ADOPTED cannot override malformed/missing physical truth;
4. new adoption persists DB adoption + dirty generation transactionally;
5. read purity is proven through the real Command Center and Cockpit paths for the same adopted project;
6. portfolio evidence distinguishes current branch HEAD from contract content provenance;
7. canonical current-truth documentation points to the latest remediation while M16 remains OPEN;
8. all M16 R82-R85 and UCP R13-R38 regressions remain green.

Only after the next independent whole-system PASS plus owner native/visual acceptance may M16 close and M17 activate.
