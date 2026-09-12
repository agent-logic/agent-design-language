# Refinement and supersession map

No supersession is enacted. Each proposed candidate points to existing records; the reverse index below gives the corresponding incoming refinements without editing historical accepted ADRs. Acceptance of a future contradiction requires explicit disposition and reciprocal records, not a silent status change.

| Existing record | Current recorded status | Proposed incoming refinements |
|---|---|---|
| [ADR0004](../../../../adr/0004-provider-profiles.md) | Accepted | ADR-PLAT-01 |
| [ADR0025](../../../../adr/0025-codefriend-review-packet-product-boundary.md) | Accepted | ADR-CF-01, ADR-CF-02, ADR-CF-03, ADR-CF-04, ADR-CF-05, ADR-CF-06, ADR-CF-07, ADR-CF-08, ADR-CF-09 |
| [ADR0041](../../../../adr/0041-provider-model-suitability-boundary-v2.md) | Accepted | ADR-PLAT-01 |
| [ADR0058](../../../../adr/0058-memory-palace-context-handoff-architecture.md) | Accepted | ADR-CF-07 |
| [ADR0072](../../../../architecture/adr/0072-csdlc-v3-native-authority.md) | Proposed | ADR-CSDLC-01, ADR-CSDLC-02 |
| [ADR0075](../../../../architecture/adr/0075-provider-profile-and-shadow-authority.md) | Proposed | ADR-PLAT-01 |

- ADR0058 remains the shared context-handoff decision, not a new evidence store.
- ADR0075 retains its historical v0.92.1 MLX deferral; ADR-PLAT-01 records the explicit bounded v0.92.2 admission. OCI is not promoted.
- ADR0072 is still proposed documentation of existing native v3 authority. Neither C-SDLC candidate replays V3-F or activates a new writer.
- ADR0069 stays Deferred with its original dual-client gate. The #910 static HTML deployment does not discharge it.
- #848 remains a separately owned repository-boundary decision. CF-09 records the current local product selection without prejudging that choice.

Machine-readable relationships: [relationships.json](relationships.json). Source conflicts and unresolved authority: [decision-dispositions.md](decision-dispositions.md).
