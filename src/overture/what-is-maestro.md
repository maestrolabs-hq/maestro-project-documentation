# What Maestro is

Maestro is the shared name for an orchestrator and the machinery that makes its
operation reproducible and reviewable.

The boundaries matter:

- [maestro-core](../estate/maestro-core.md) owns the protocol and the
  orchestrator-facing command surface.
- [maestro-pi-config](../estate/maestro-pi-config.md) owns changes to a pi
  installation and the ability to reproduce them.
- [maestro-governance](../estate/maestro-governance.md) owns the desired state
  of repositories and reports drift from it.
- The organization [`.github` repository](../evidence/sources.md) owns shared
  workflows and community policy.
- This book owns only the relationships, shared vocabulary, and public
  direction across those boundaries.

Accountability is the common thread. Delegation may move work to another agent
or repository, but it does not move responsibility for the result.
