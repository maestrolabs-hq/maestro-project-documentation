# maestro-governance

<span class="maestro-status" data-status="built">Built</span>

## Role

`maestro-governance` records what repositories should look like and reports the
difference between that baseline and GitHub.

## Owns

- the organization repository baseline;
- reading settings, effective rules, and tracked files;
- applying only the setting drift its API boundary can write;
- the weekly fleet audit.

## Does not own

It does not own repository implementation or silently correct every kind of
drift. Rules, organization controls, and tracked files remain explicit
boundaries rather than simulated writes.

## Primary sources

- [README](https://github.com/maestrolabs-hq/maestro-governance/blob/main/README.md)
- [Baseline](https://github.com/maestrolabs-hq/maestro-governance/blob/main/baseline.txt)
- [North star](https://github.com/maestrolabs-hq/maestro-governance/blob/main/NORTHSTAR.md)
- [Open work](https://github.com/maestrolabs-hq/maestro-governance/blob/main/TODO.md)
- [Accepted decisions](https://github.com/maestrolabs-hq/maestro-governance/tree/main/docs/adr)
