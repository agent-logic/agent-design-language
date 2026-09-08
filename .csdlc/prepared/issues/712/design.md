# Runtime startup simplification

## Decision

Runtime startup uses one canonical config file and one canonical Runtime binary. CSM manages operator intent; Guardian supervises exactly one Runtime child; Kernel validates the config it actually loads and publishes that config hash. No cross-binary prepared receipt is required.

## Startup

1. CSM asks Guardian to start with the canonical config path.
2. Guardian starts the stable Runtime binary with that path.
3. Kernel parses and validates the config before binding listeners.
4. Readiness reports the loaded config hash and child process identity.

## Reload

CSM writes a candidate beside the active config. Kernel-compatible validation occurs before replacement. A successful atomic rename installs the candidate and Guardian reloads or restarts the child. Failure leaves the active config and running child unchanged. Recovery chooses the active file, never an abandoned candidate or backup.

## Security boundary

Retain API authentication, signed ACIP messages, agent identity, replay protection, and audit. Remove startup ceremony that does not defend a trust boundary.

## Compatibility

Existing agent roster, providers, Shepherd, A2A, observability, and checkpoints remain behaviorally unchanged.
