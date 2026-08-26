# Issue 514 design: PROV-A

## Outcome

Produce one shared provider inference-profile contract with deterministic Ollama materialization.

## Authority and scope

This issue owns only the declared paths below. It does not authorize adjacent sprint work,
cloud/provider mutation, credential disclosure, legal advice, or lifecycle work for another issue.

- `adl/src/provider/**`
- `adl-runtime/src/provider/**`
- `docs/provider/**`
- `docs/milestones/v0.92.1/evidence/provider/prov-a/**`
- `.csdlc/prepared/issues/514`

## Execution shape

1. Reconcile dependencies and freeze the exact issue-local denominator.
2. Produce one shared provider inference-profile contract with deterministic ollama materialization.
3. Run the planned PVF lanes and retain bounded, redacted evidence.
4. Obtain exact-head review and stop before publication unless separately authorized.

## Invariants

- Issue completion is exactly one shared provider-profile contract; provider-specific checks are evidence inputs.
- Schema, materialization, invalid-profile, last-known-good, and redaction checks pass.
- Private credentials, legal instruments, auth codes, recovery factors, and provider secrets stay outside Git.
- Any operator-only mutation requires explicit bounded authorization at execution time.

## Stop conditions

- Tools require incompatible provider configuration
- Private material would enter a profile
