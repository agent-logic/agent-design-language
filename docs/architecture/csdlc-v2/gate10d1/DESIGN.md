# Gate 10D1 non-mutating deletion eligibility

`csdlc-eligibility` is a read-only authority boundary between reviewed cutover
evidence and any future deletion issue. It reads versioned Phase B/Phase C
evidence, the tracked generation selector, a proposed removed/retained line
partition, typed approval, and protected-window timestamps. It writes one atomic
decision file and has no code path for deleting, renaming, editing, staging,
committing, publishing, or closing candidate v1 paths.

The manifest must exactly partition the pinned 49,979-line Gate 1 denominator.
Every retained entry names an owner and justification. Ninety percent is the
review target; 80-89 percent requires explicit qualification in the approval;
below 80 percent is never eligible. Absolute/traversing paths, duplicate paths,
zero-line entries, malformed timestamps, and unowned retained entries are
invalid inputs.

Approval is not a boolean. It binds the approver and approval time to the exact
BLAKE3 digests of Phase C evidence and the proposed manifest. Missing or
mismatched approval, non-green phase evidence, a non-v2 selector, an active
protected window, or a deficient removal percentage produces `eligible=false`.
The decision always records `deletion_executed=false` because execution belongs
to separate issue #5306.

The tracked request intentionally contains no approval and retains the full
baseline. Its expected decision is ineligible with zero v1 mutation. Synthetic
tests cover positive eligibility only to prove decision semantics; they grant
no operational deletion authority.
