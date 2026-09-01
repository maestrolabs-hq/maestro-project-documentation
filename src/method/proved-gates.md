# Proved gates

A gate is useful only if the failure it names changes its result.

<div class="maestro-claim" data-status="built" data-source="https://github.com/maestrolabs-hq/.github/blob/d28605f45755668799b180c0ad32c3ec30f3282d/profile/README.md#L17-L26" data-verified="2026-09-01">
  <p>The shared estate checks run locally and in CI, and each gate has been exercised by injecting the fault it is intended to catch.</p>
</div>

That practice changes the meaning of green. It is no longer evidence that a
command happened to exit successfully; it is evidence that the command has
observed the boundary it claims to protect.

The workflow definitions and exact tools remain authoritative in the
[organization `.github` repository](https://github.com/maestrolabs-hq/.github).
