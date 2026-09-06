# Structured Task Prompt

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add one production provider reload owner and connect it to per-step provider resolution in the existing execution runner.

## Deliverables

- ProviderReloadOwner using the existing watcher
- Provider-only sidecar validation and atomic snapshot publication
- Production execution-runner snapshot consumption
- Focused positive negative concurrency and shutdown proof
- Operator documentation

## Acceptance

1. AC-1: A production provider owner starts the existing watcher against an explicit provider-only configuration path
2. AC-2: A valid provider or profile edit becomes active for subsequent inference without process restart
3. AC-3: Invalid edits retain the complete last-known-good snapshot with a bounded redacted diagnostic
4. AC-4: Concurrent readers observe either the old or new complete snapshot and never a partial mixture
5. AC-5: In-flight inference retains its starting snapshot while later inference consumes the accepted replacement
6. AC-6: Endpoint model parameter timeout capability and profile changes have focused positive and negative proof
7. AC-7: Credential values authority objects and executable workflow content are rejected from the provider-only sidecar
8. AC-8: Unknown mandatory parameters and unsupported capabilities fail closed
9. AC-9: Debounce duplicate event same-content rapid rewrite and watcher shutdown behavior is proven
10. AC-10: Exact production call-path formatting lint diff hygiene and independent review pass

## Dependencies

- #510 generic hot-reload substrate
- #514 shared provider inference profiles
- #551 bounded production reload precedent

## Inputs

- agent-logic/agent-design-language#622
- adl-runtime-kernel/src/config_reload.rs
- adl/src/provider/profiles.rs
- adl/src/provider/mod.rs
- adl/src/execute/runner.rs
- docs/runtime/config-hot-reload.md

## Non Goals

- Reload credentials signing keys database pools authority objects or model weights
- Create a second watcher or provider registry
- Implement MLX OCI packaging automatic tuning or provider UI
- Change an in-flight request after dispatch
- Redesign the full provider system
