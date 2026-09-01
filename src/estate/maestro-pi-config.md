# maestro-pi-config

<span class="maestro-status" data-status="built">Built configuration tool</span>
<span class="maestro-status" data-status="designed">Designed memory shim</span>

## Role

`maestro-pi-config` is the single home for changes that modify pi's behavior.
It makes those changes versioned and reproducible instead of machine memory.

## Owns

- captured pi configuration;
- planning, applying, and removing repository-owned machine changes;
- pi runtime extensions and their client-side integration boundary.

## Does not own

It does not own orchestration policy, durable event delivery, or the consumer
of captured memory. Those responsibilities cross into `maestro-core` only
through an explicit client boundary.

## Primary sources

- [README](https://github.com/maestrolabs-hq/maestro-pi-config/blob/main/README.md)
- [Context](https://github.com/maestrolabs-hq/maestro-pi-config/blob/main/CONTEXT.md)
- [North star](https://github.com/maestrolabs-hq/maestro-pi-config/blob/main/NORTHSTAR.md)
- [Open work](https://github.com/maestrolabs-hq/maestro-pi-config/blob/main/TODO.md)
- [Architecture](https://github.com/maestrolabs-hq/maestro-pi-config/blob/main/docs/architecture.md)
