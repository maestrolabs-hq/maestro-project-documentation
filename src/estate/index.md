# Ecosystem map

The estate separates orchestration, machine configuration, organization policy,
and cross-project explanation.

| Repository | Estate role | Status |
| --- | --- | --- |
| [maestro-core](maestro-core.md) | protocol and orchestrator-facing boundary | <span class="maestro-status" data-status="designed">Designed</span> |
| [maestro-governance](maestro-governance.md) | desired repository state and drift | <span class="maestro-status" data-status="built">Built</span> |
| [maestro-pi-config](maestro-pi-config.md) | reproducible pi configuration | <span class="maestro-status" data-status="built">Built</span> |
| This book | shared estate view and public roadmap | <span class="maestro-status" data-status="in-progress">In progress</span> |

```text
pi runtime ──client boundary──> maestro-core
     │                              │
     └── configuration owner        └── orchestration owner
              │
              └──────── repositories <── governance baseline
```

The arrows describe relationships, not deployment topology. Follow each
repository page to its primary architecture and status documents.
