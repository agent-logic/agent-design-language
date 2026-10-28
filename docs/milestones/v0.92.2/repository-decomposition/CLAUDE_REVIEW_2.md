# Claude Fable 5 — corrected candidate review

Reviewed candidate: `fa1f16322083ae13b4980bdd733442918da39b56`, PLAN SHA-256 `56c45939c6477ffae3e4ba89191aa8ef60a0889ec144c842962cb9bf43ddc6c6`. Provider result status `ok`.

## Verification of F10 correction

**F10 is resolved.** The prior contradiction — an unconditional claim of C-SDLC/Runtime independence alongside the retained, inventory-evidenced `csdlc-v2 -> adl-resilience` path dependency — has been corrected coherently in three places:

1. **Section 3 (`cognitive-sdlc`)** now scopes the independence claim to "the target C-SDLC v3 product" and explicitly labels it "a target invariant to prove, not a claim about every retained monorepo artifact." The `csdlc-v2 -> adl-resilience` edge is acknowledged as a known cross-boundary build dependency, with RD-01 assigned to determine its disposition without deleting or stranding authorized rollback support.
2. **Preconditions** add an explicit, correctly scoped gate: the edge must receive an RD-01 disposition *before any extraction removes its required source or artifact*, without creating a blanket gate on unrelated product analysis. This correctly protects against RD-05 (Runtime extraction, which moves `adl-resilience`) breaking the restricted v2 rollback build, while leaving ADL/infrastructure/demo work ungated.
3. **RD-04 proof scope** extracts v3 only; v2 remains behind as the rollback path, consistent with "later deletion is code reduction, not migration." The independence proof therefore no longer implicitly asserts something the inventory falsifies.

The correction preserves the hypothesis-falsification framing (H1–H3, with "missing evidence never counts as survival") rather than reclassifying the edge as accidental by definition. The dependency-direction diagram's "no functional dependency" statement is now consistent, since the v2 edge is a build dependency explicitly carried as an RD-01 disposition item.

## Remaining review against the inventory

I checked the corrected text against the source inventory for any analogous unconditional claims contradicted by recorded edges:

- **`adl/Cargo.toml -> adl-runtime, adl-runtime-kernel`**: these edges invert the intended target direction (`agent-design-language <--- agent-logic-runtime`). This is *not* a new actionable finding because the plan explicitly frames all inventory edges as "current monorepo edges, not the permitted post-extraction topology," requires RD-01 to classify and remove or replace every relevant edge before extraction, and names `adl/` mixed responsibilities as "the critical first deliverable" in Risks. Unlike the pre-correction F10, no section asserts current ADL/Runtime source independence as fact.
- **`adl-runtime -> adl-resilience, adl-runtime-kernel`** and the `adl-v2` workspace edges fall inside single proposed boundaries and raise no contradiction.
- **`.csdlc/prepared/issues/*` Cargo manifests** are covered by the repository-governance ownership candidate for immutable historical evidence and by the RD-01 denominator requirement (evidence dependency classification, orphan reporting).

## Minor observations (non-actionable)

- Section 2 states "The Runtime and CSM have no functional, build, or release dependency on C-SDLC" declaratively rather than as a target invariant. No inventory evidence contradicts this direction, RD-01 must still attempt falsification of H1–H3, and preconditions gate extraction on the complete denominator — so this is a stylistic asymmetry, not a safety gap.
- RD-05's rollback boundary does not restate the v2-edge disposition gate, but the global precondition binds all extractions, so no per-step restatement is required.

## Conclusion

**Approved.** The single actionable finding (F10) from the previous review is corrected, the correction is internally consistent across the boundary description, preconditions, dependency diagram, and RD-04/RD-05 sequencing, and no new actionable findings arise from comparison against the source inventory. Approval is planning-review evidence only; per the plan's own ledger and decision sections, it does not constitute decision readiness (fresh Claude/Gemini reviews remain outstanding) and authorizes no extraction, repository creation, visibility change, or v0.93 implementation.
