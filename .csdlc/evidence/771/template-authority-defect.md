# V3F-EVIDENCE-R1: native issue cards emit retired authority instructions

At source `7c5b7873418249726d7c7d14f6e7f787358a80bf`, native registry loading in `csdlc-v3/src/commands/local/mod.rs` uses top-level template paths from `docs/templates/prompts/current.json`; issue initialization renders those templates.

The active `1.0.4/sip.md` constraints direct lifecycle work through typed v2, and `1.0.4/sor.md` says V3-F is pending and v2 remains live authority. Native initialization therefore emits current routing instructions that contradict the completed #505 cutover and canonical native selector. The independent evidence review records this as V3F-EVIDENCE-R1, a current V3-F AC4 routing defect; resolved CORP-A dispositions remain unchanged.

Proposed bounded repair: intentionally version the active template set with corrected native authority and explicit exception language, regenerate matching structure schemas, and add focused native bootstrap/renderer proof. Preserve old template versions and historical issue505 receipts. No authority selector or runtime behavior change is needed.
