# v0.92.1 ADR curation design

Issue #745 turns the already reviewed Planning #7 source packet into tracked documentation. Four new Proposed candidates (0072-0075) capture native C-SDLC v3 authority, validated configuration reload, DEC-01 source ownership, and provider profile/shadow authority. Existing ADR 0069 remains Deferred. The milestone plan explicitly routes all eight topics, including existing ADRs 0066 and 0070 and conditional v4/GCP decisions. No accepted decision is silently promoted or superseded.

Apply the packet only in the issue-bound FastWork worktree after checking candidate number collisions and source freshness. Retain a source hash manifest, focused documentation validator, and a truthful record of v2 authorization and defect #744. Native v3 remains default authority; its failed intent is preserved for #744.

Validation is documentation-only: candidate sections/status, exact eight-topic denominator, relative links, source hash checks, original ADR 0069 preservation, and diff hygiene. Independent exact-revision review precedes typed publication; required CI must settle before terminal delivery. Runtime and cloud tests are not needed because implementation behavior is unchanged.
