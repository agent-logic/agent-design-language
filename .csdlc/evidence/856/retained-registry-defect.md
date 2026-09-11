# Retained v2 registry rejects current template 1.0.5

Detected by PR858 CI run34636240698, csdlc-v2-standalone job103385178428.
16 of22 card_identity tests fail before behavior under test with
InvalidManifest: prompt registry top-level authority is incompatible.

Source: csdlc-v2/src/registry.rs validate_native_registry accepts only1.0.3/1.0.4;
docs/templates/prompts/current.json declares1.0.5. Native family remains
csdlc_v2_native compact_native template1.0.0. #856 has not changed either
registry acceptance code or current registry.

Proposed bounded repair: add1.0.5 to explicit accepted top-level versions;
retain semver/schema/status/lifecycle/native-shape/legacy-entry checks;
extend gate9 explicit-version positive and mutation negatives. Run retained
v2 card_identity, gate9 and standalone suite. This does not authorize v2
operational use or change the generation selector.

Disposition: operator explicitly approved on 2026-09-11: "Yes, fix this compatibility defect too". Bounded #856 scope now includes accepting top-level registry1.0.5 while preserving all other generation/shape checks, extending positive and incompatibility tests, and rejecting unsupported1.0.6. No v2 lifecycle operation or selector change is authorized or performed.
