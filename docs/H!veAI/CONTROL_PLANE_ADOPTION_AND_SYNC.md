# Adoption and Safe Synchronization

H!veAI discovers registered projects through the Project Registry. A project is adopted only through the explicit native adoption command. The command creates missing v1 control-plane files with create-new semantics and leaves existing canonical task, rules, Git, and provider files untouched.

Live synchronization watches the canonical task source, control-plane files, .git/HEAD, .git/index, and declared event sources. Events are debounced and coalesced per project. Internal lifecycle events invalidate the affected project immediately; periodic safety reconciliation is bounded.

For Git projects, fetch and fast-forward are separate decisions. A clean branch with ahead=0 and behind>0 may be fast-forwarded only when the owner setting is enabled. Divergence, local commits, dirty state, conflicts, detached HEAD, or invalid identity never triggers reset, rebase, stash, discard, or merge. Non-Git projects retain their local data and receive a repair/connect plan that requires an owner-selected clone or backup path.
