# [v0.91.8][csdlc-v2] Complete typed preparation-to-implementation replanning

Closes #5600

Adds card-owned typed replacement operations for every remaining SIP, STP,
SPP, and SRP planning collection. A coherent acceptance-plan operation
atomically replaces STP acceptance criteria, SPP plan steps, and VPP validation
lanes, permitting cardinality changes without an invalid intermediate state
while preserving generation, digest, claim ownership, phase authorization,
complete projection regeneration, audit, backup, and commit semantics.

The #5337 preparation fixture performs the complete conversion through the
real `csdlc-edit` JSON CLI, changes acceptance cardinality from two to three,
and proves cross-card validation plus typed doctor. Negative tests cover empty
and misowned fields, stale generation and digest, wrong claims, incomplete and
duplicate acceptance mappings, pre-Bound and post-Implemented rejection,
progress smuggling, and atomic failure behavior.

Validation is fully green with no deferred lane: focused Gate 2 tests 41/41,
every C-SDLC v2 target, `cargo fmt --check`, strict all-target Clippy, and typed
doctor. The first exact-revision review found four implementation defects; all
were fixed. The remediation review passed with no actionable findings and
explicitly marked all four prior findings fixed.
