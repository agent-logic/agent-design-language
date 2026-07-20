# Issue #5597 Design

Preserve the legacy/import `1.0.3` and native compact v2 `1.0.0` card families as distinct authorities. Add a generation-aware native-v2 entry to the active registry and validate it against a crate-owned compact-shape manifest before initialization writes any state.

Extend native typed input, migration, and semantic edits so SIP operator constraints and SRP review scope are explicit and lossless. Permit preparation-safe SIP constraint replacement, STP acceptance-criteria replacement, and bound SRP replanning. STP replacement must atomically reject any SPP step or VPP lane coverage gap. Review assignment must synchronize authoritative scope into SRP immediately.

Existing native `1.0.0` records remain readable unchanged. Legacy `1.0.3` import remains a separate migration path. No template relabeling, bulk migration, product work, AWS, or raw GitHub CLI is in scope.
