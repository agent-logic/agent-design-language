# Issue 5906 design: deterministic multiple-merge closeout precedence

## Problem

GitHub can retain more than one merged pull request as a closing reference for
one issue when a later corrective PR also uses a closing keyword. Historical
finish currently rejects every such case, even when GitHub provides a unique
latest merge timestamp and the request pins that exact PR, head, and merge SHA.

## Design

Extend closing-PR identity with GitHub `mergedAt`. For merged historical
reconciliation:

1. preserve the existing single-merged-candidate rule;
2. ignore closed-unmerged candidates for merged precedence, and require every
   merged candidate to have a valid RFC 3339 `mergedAt` timestamp;
3. parse each timestamp to an absolute instant and require one unique latest
   instant (never compare timestamp strings); and
4. require the request to name that exact latest candidate.

Missing or malformed merged timestamps, tied latest instants, or a request
naming an earlier PR remain reconciliation errors. Closed-unmerged attempts do
not interfere with merged precedence. Exact issue/repository/PR/head/merge
checks are unchanged.

## Scope

- `csdlc-v2/src/github.rs`
- `csdlc-v2/src/finish.rs`
- focused `csdlc-v2` historical-finish tests

## Non-goals

- changing routine review, publication, or finish gates;
- selecting by PR number;
- rewriting merged PR bodies;
- broad closeout redesign.
