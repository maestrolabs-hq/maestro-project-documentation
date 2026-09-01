# maestro-core

<span class="maestro-status" data-status="built">Built foundation</span>
<span class="maestro-status" data-status="designed">Designed orchestration</span>

## Role

`maestro-core` is the boundary clients ask and Maestro answers through. It is
the estate owner for the protocol and orchestrator-facing command surface.

## Owns

- protocol vocabulary and envelope shape;
- the `maestro` binary boundary;
- supervisor and ledger designs until implementation gives them a narrower
  home.

## Does not own

It does not own pi runtime configuration or organization repository policy. It
also does not know a particular agent implementation.

## Primary sources

- [README](https://github.com/maestrolabs-hq/maestro-core/blob/main/README.md)
- [Context](https://github.com/maestrolabs-hq/maestro-core/blob/main/CONTEXT.md)
- [North star](https://github.com/maestrolabs-hq/maestro-core/blob/main/NORTHSTAR.md)
- [Open work](https://github.com/maestrolabs-hq/maestro-core/blob/main/TODO.md)
- [Architecture documents](https://github.com/maestrolabs-hq/maestro-core/tree/main/docs)
