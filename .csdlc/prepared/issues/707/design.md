# Issue #707 design: coherent Runtime generation and live A2A deployment

Use one canonical, dependency-version-independent encoding for Runtime configuration receipt identity. CSM, Guardian, and Kernel must derive byte-identical generation and receipt digests from the same init and installed binary-generation name even when Cargo resolves their package graphs independently. The install receipt continues to bind the three executable hashes and source revision; no validation is weakened.

Focused proof builds the three production binaries through their normal manifests, prepares one config receipt with CSM, and verifies that Guardian and Kernel accept it. A live rollout installs the three artifacts as one generation, starts only through canonical CSM and launchd, verifies owned readiness, and exercises the merged A2A action-selection path. The proof distinguishes the operator-facing reply from a separately addressed Beacon-to-Ember work item and confirms Ember receives it.

The current `main-cea5219f6-20260903` generation remains the last-known-good rollback target until the new generation passes every check.
