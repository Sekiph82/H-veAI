# H!veAI Versioned Artifact Naming Governance

Effective: 2026-09-11

H!veAI active prompts, builder logs, audits, and task-specific governance
artifacts use a stable work-item code and two-digit execution version.

## Filename shape

`<WORK_CODE>_<SHORT_DESCRIPTION>_VNN_<TYPE>.md`

Examples:

- `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_PROMPT.md`
- `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_LOG.md`
- `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_AUDIT.md`

## Rules

- `V01` is the first execution of a work item.
- A remediation of the same work item advances to `V02`, then `V03`, and so on.
- Do not create active `RERUN`, `RETRY`, `FINAL2`, `NEW`, or `LATEST` suffixes.
- A materially different task receives a new work-item code.
- A prompt, builder log, and independent audit for one execution share the same
  work code and version.
- Historical files retain their original names and remain immutable.
- Completion reports print the exact work code and version.

This governance document is repository content and is portable with the
standalone H!veAI checkout.
