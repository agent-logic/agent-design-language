# Internal Review Packet Assignment Recheck

## Result

**PASS**

The repaired deterministic assignment artifact is reproducible, bound to the exact review target, nonempty across all nine required lanes, materially populated for the code/tests/security/dependency lanes, and explicit about the clean primary source checkout and registered issue-output worktree.

## Metadata

- Generator: `.csdlc/prepared/issues/313/build_internal_review_assignments.rb`
- Assignment artifact: `docs/reviews/v0.92/internal-review-5846/specialist_assignments.json`
- Exact target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Source checkout kind recorded: `clean_primary_checkout`
- Output worktree recorded: `issue_313_registered_fastwork_worktree`
- Recheck date: 2026-08-25 UTC
- Findings: 0

## Deterministic Regeneration

The generator was rerun with:

```text
ruby .csdlc/prepared/issues/313/build_internal_review_assignments.rb \
  <clean-primary-checkout> \
  .adl/reviews/313-assignment-recheck \
  c6792e54df1db5969fa28c59b6dfe4c714ed5559
```

The regenerated JSON was byte-identical to the packet artifact (`cmp` exit status `0`). `ruby -c` also reported `Syntax OK`.

## Exact-Target And Checkout Truth

- The assignment records the full target SHA `c6792e54df1db5969fa28c59b6dfe4c714ed5559`, not the abbreviated packet-builder display ref.
- The generator inventories with `git -C <source-root> ls-tree -r --name-only <target-sha>`, so lane membership is derived from the exact committed tree rather than the output worktree's dirty or later state.
- Live source verification found the primary checkout clean on branch `main` at the exact target SHA.
- Live worktree registration found the issue-313 FastWork worktree registered on `codex/313-v092-internal-review-preparation`; it is the issue-output worktree and may advance beyond the reviewed source target while collecting review artifacts.
- The portable labels `clean_primary_checkout` and `issue_313_registered_fastwork_worktree` make the two roles explicit without embedding machine-local absolute paths in the JSON artifact.

## Required Lane Denominator

All nine required lanes are present and nonempty:

| Lane | Assigned paths |
|---|---:|
| architecture | 736 |
| code | 1,442 |
| dependencies | 61 |
| docs | 6,349 |
| security | 1,756 |
| tests | 834 |
| lifecycle | 6,050 |
| demos | 1,216 |
| release_publication | 4,909 |

The generator fails closed with `abort "empty <lane> assignment"` if any required lane resolves empty.

## Material Surface Checks

- Code: includes Rust, Ruby, Python, shell, JavaScript, TypeScript, and C# implementation paths while excluding paths classified as test/fixture surfaces.
- Tests: includes conventional `test`, `tests`, and `fixture` path segments plus `_test` and `.test` filename forms; 834 exact-target paths are assigned.
- Security: includes authentication, security, privacy, secret, signing, TLS, credential, sandbox, permission, and redaction name surfaces. The lane is deliberately broad and synthesis must distinguish executable source from retained evidence.
- Dependencies: includes actual `Cargo.toml`/`Cargo.lock` pairs, GitHub workflow dependency bootstrap, and `adl/docker/adl-builder/Dockerfile`; it no longer consists of lifecycle `.csdlc/locks/*.lock` sentinels.

Machine assertions confirmed that each of these four lanes contains at least one materially matching path, in addition to the nonempty-count check.

## Validation Performed

- `ruby -c .csdlc/prepared/issues/313/build_internal_review_assignments.rb` — passed.
- Exact generator rerun against the clean primary checkout and full target SHA — passed.
- Byte comparison between regenerated and packet `specialist_assignments.json` — passed.
- JSON assertion for schema metadata, exact target, source/output role labels, and all counts greater than zero — passed.
- JSON assertions for material code, test, security, and dependency path inclusion — passed.
- Live Git status/HEAD/branch verification of the source checkout — clean `main` at the exact target.
- Live registered-worktree verification of the issue-output worktree — present on the expected issue branch.

## Limitations

- This recheck validates deterministic assignment, target provenance, topology labels, and material path inclusion. It does not judge whether every assigned path deserves equal specialist attention.
- Name-based security and release classification intentionally over-selects some retained evidence; specialist review and synthesis remain responsible for prioritization and deduplication.
- This artifact does not re-run specialist reviews or modify their findings.
