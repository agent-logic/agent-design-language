# Issue 301 design: durable provenance for title-only issue updates

## Boundary

Issue 301 changes only the typed GitHub issue-update owner and focused tests. It does not touch lifecycle stores, card identity, projection recovery, or coverage tooling.

## Design

`issue_update` will treat title-only provenance as an operation receipt rather than rewriting the issue body. The owner computes a canonical request fingerprint over the governed mutation fields, repository, issue number, and operation key. It searches issue comments for the operation key before mutation. An existing receipt with the same fingerprint reconciles idempotently after exact issue readback; the same key with a different fingerprint fails closed.

For a new operation, the owner reads and retains the exact pre-mutation issue body and remote revision metadata, sends one title-only PATCH, then reads the issue again. It must observe the requested title and byte-identical body before posting one provenance comment containing the operation key, request fingerprint, exact pre/post body digest, and observed pre/post remote revision metadata. It finally reads both issue and comment back and returns `reconciled=true` only when the title, unchanged body, unique receipt, and fingerprint all agree.

GitHub does not provide a relied-upon conditional PATCH mechanism in this contract. Therefore the design does not claim atomicity across GET, PATCH, and receipt POST. A concurrent body mutation before or during the PATCH is detected by the post-PATCH byte comparison and fails closed without issuing a provenance receipt. A body mutation after that comparison but before or after receipt creation remains a disclosed residual window: final readback detects it and returns reconciliation failure, but cannot undo the remote title change or comment. Retry never overwrites the body and reconciles only from exact current state plus the matching fingerprinted receipt. The scripted provider fixture deterministically injects body drift before PATCH, during PATCH handling, after PATCH/before receipt, and after receipt/before final readback.

Body-bearing updates retain their existing marker-in-body behavior. The new receipt path is selected only when `body` is absent and the request otherwise mutates the issue.

## Failure behavior

Failure to read or preserve the body, establish a unique key-to-fingerprint mapping, observe the requested title, create/read the receipt, or pass final drift checks returns a typed error; it never reports success from title equality alone. No provider-level atomicity is claimed: the issue mutation is one PATCH, and provenance is a separately reconciled comment.

## Validation

Focused owner tests cover title-only preservation/provenance, same-key idempotent retry, conflicting fingerprint rejection, partial-operation recovery, four deterministic concurrent-body-drift boundaries, and compatibility with body-bearing updates. Strict Clippy covers the owner crate. The local scripted provider fixture supplies network behavior without mutating GitHub.
