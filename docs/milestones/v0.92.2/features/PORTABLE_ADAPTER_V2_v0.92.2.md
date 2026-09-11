# Portable Adapter v2

Status: planned. Owners: CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI.

Adapter v2 normalizes local checkout, GitHub, and CI inputs into one repository-packet contract. Packets use stable repo-relative identities, bounded content, declared omissions, and provenance without machine-local paths or captured credentials.

Acceptance requires fixture parity across input modes, explicit partial-input behavior, path portability, and failure tests. Jira, Linear, Slack, and broad Workspace integration are deferred.

The listed owners deliver separate working tasks, with complete consumer and failure evidence defined in the [atomic task contracts](../ATOMIC_TASK_CONTRACTS_v0.92.2.md). A feature packet or schema alone cannot close an implementation row.
