# Issue #686 Design: Governed Runtime configuration generations

## Decision

Represent each committed Runtime configuration as an immutable generation
directory containing the canonical configuration plus a versioned receipt. The
receipt binds the canonical content digest, configuration schema version,
redacted secret references, and compatible Runtime binary generation. A single
atomically replaced `current` reference names the committed configuration
generation.

CSM owns candidate construction and activation. Guardian and the Runtime kernel
receive the generation identifier and receipt digest explicitly and reject any
file, receipt, or binary-generation mismatch before readiness. Status and
readiness report those same values; they do not independently infer authority
from the mutable source path.

## Receipt contract

The receipt is canonical JSON with a fixed schema identifier. It includes only:

- configuration generation identifier;
- canonical configuration content SHA-256;
- configuration schema version;
- normalized secret *references* with values redacted/absent;
- compatible installed Runtime binary generation;
- receipt digest derived from canonical receipt content.

Unknown schema versions, malformed paths, duplicate/conflicting references,
receipt mutation, incompatible binary generations, and digest mismatch reject
before service mutation.

## Transaction and recovery

Reuse #589's candidate, backup, readiness, and commit transaction. Candidate
generation files and receipt are durably completed before the active reference
can move. The active-reference replacement is atomic and retains enough
transaction state to classify recovery:

- before activation: discard the incomplete candidate and retain prior current;
- after pointer replacement but before candidate readiness: validate the
  candidate receipt, then continue or restore prior current;
- after candidate readiness but before commit cleanup: commit only when the
  running Runtime reports the exact candidate generation and receipt digest;
- after ordinary candidate failure: restore the prior committed reference and
  verify its receipt before reporting recovery complete.

Recovery never treats a responsive unrelated Runtime, a bare source-file hash,
or a partially written receipt as committed authority.

## Component handoff

CSM validates the selected receipt and passes its generation identity to the
Guardian. Guardian binds its child launch to that identity. The kernel validates
the on-disk active reference and receipt before declaring readiness. CSM status,
kernel readiness, and Guardian-owned process observations must all agree on the
same generation identifier and receipt digest.

## Validation

Focused deterministic tests use isolated filesystem/process fixtures and inject
failpoints at each transaction boundary. They prove rejection before mutation,
secret redaction, cross-component identity agreement, candidate commit, and
prior-generation restoration. No test addresses or mutates the live Wuji
Runtime.
