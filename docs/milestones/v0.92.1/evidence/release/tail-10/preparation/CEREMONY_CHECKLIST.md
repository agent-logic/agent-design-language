# v0.92.1 ceremony preparation — #526

This packet prepares the ceremony; it does not authorize or record one. `ceremony-preparation.json` binds the inspection to its exact preparation revision and source hashes. No final candidate has been approved, no tag or release was created, and the notes remain the truthful generated release projection.

## Before requesting operator approval

1. Complete #522 remediation and #525 final planning review. Confirm all nine prior tail issues have reviewed-green ancestral merges at the final candidate, using the linked PR census. Issue closure and a merged failed-review report alone are insufficient.
2. Resolve the four final #821 obligations and refresh current execution evidence where required. Run `python3 .csdlc/prepared/issues/835/project_release.py --check` and its `--require-ready` gate against refreshed inputs. Retain the actual results. Do not edit generated release notes directly to remove blockers.
3. Resolve `CEREMONY_TOOLING_GAP.md`: version parity and an operational v3 ceremony gate must exist before invoking the release wrapper. Do not bypass the gate or silently use v2.
4. Integrate the final notes and all prerequisite proof. Pin the immutable candidate SHA and notes blob/SHA-256, then repeat exact-head review and required full release validation. Docs-only CI skips are not release coverage.
5. Refresh the nine-row live census, tag absence/presence and GitHub release identity. Confirm every prerequisite merge is ancestral to the candidate. If an existing tag or release appears, inspect its exact target and stop on mismatch; never overwrite it.
6. Present the exact candidate SHA, notes hash, proposed tag `v0.92.1`, intended release visibility and reviewed gate results to the operator. Retain explicit approval identifying those objects. A request to prepare #526 is not that approval.

## After explicit approval

Use the then-current, verified release owner to create the annotated tag, push that exact tag, create the release from the approved notes and publish only with approval for that action. The existing `adl/tools/release_ceremony.sh` documents this order but is not currently qualified for this milestone; its defects below must be resolved first. Do not substitute raw lifecycle commands or skip flags to bypass it.

Read back and retain:

- repository and immutable approved candidate;
- operator authorization reference and exact approved operations;
- prior-tail issue/PR numbers, reviewed heads, required checks and merge ancestry;
- notes path, Git blob, SHA-256 and the release body's actual content;
- tag name, annotated tag object and peeled commit;
- GitHub release ID, URL, draft/prerelease state, tag and publication time;
- equality checks proving tag, notes and release match the approved objects;
- errors or partial publication state if any operation fails.

Only a successful, authenticated readback receipt completes #526. Keep final receipt evidence separate from this preparation snapshot. Administrative finish and cleanup remain separate.

## Existing evidence limitations

The generated `release_evidence_report` is a bounded discovery summary, not semantic proof. Its `present` families mean matching documents exist. Current source hashes and the live census are in `ceremony-preparation.json`. Seven prior tail issues were closed at inspection; #522 and #525 were open. PR #850 retains a failed third-party review, so closure of #521 is not approval of its candidate.
