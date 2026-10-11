# CF-05 / ADL #1150 independent qualification decision

Decision date: 2026-10-10

Decision: **QUALIFIED**

Original Q01-Q24 obligations: **24 PASS / 0 FAIL / 0 INCOMPLETE**

Release-accepted cells: **12/12**

Execution-group semantic dispositions: **23 PASS / 1 FAIL / 0 INCOMPLETE**

## Decision

The exact deployable CodeFriend candidate aa2cca757ba4e6c37cef6c1a6e013d3896cd1de1,
tree 89ca46e84d9f1060c80e47c97704eb9ab369af2a, satisfies the retained
CF-05 qualification contract. All 24 original obligations pass and all 12
surface/repository/platform cells are accepted for the deployable-candidate
release claim.

This decision preserves the execution candidate and raw provider identities
instead of relabelling them. The reviewed compatibility bridge proves that the
review, agent, evidence, ingestion, server, fitness runner, and ProductCI paths
are unchanged where retained evidence is reused. Fresh aa2 installed-product,
Q08, publication-output, and human-render evidence closes the candidate
transition.

## Final gates

### Q02 and Q03 — identity admission and isolation

Q02 passes from the real daniel-asksifu GitHub identity receiving
invitation_required while its subject was absent, followed by restored
subject-bound access. Q03 passes from fresh owner run
29d151c5086206261d273306e4f8df430e059b51eb915015: the owner loaded the
completed result, while invited second subject github_340710889 received
run_not_found for that run and result before expiry. The four provider lanes
were each attempted once; there were zero retries, unknown outcomes, or replay.

### Q08 — exact-aa2 local/CI fitness parity

Authenticated ProductCI run 38030979163, attempt 1, artifact 11661624501,
archive SHA-256 2bf22aa1d0f354c5a719d26a07c92f0fbd39a890733f4d7336bed80b943c2c0c
was independently installed for candidate, pass, fail, and error. The installed
Linux x86_64 binary SHA-256 is
55520a6994b084affcc1aeb9dd88ae6aac245a88aeef7263f2402ae47cf1cb39.
The four cases produced exits 0/0/1/2; every proof records
local_result_parity=true.

### Q19 — human report acceptance

The operator accepted the presentation quality and page integrity of the exact
12 HTML plus 12 PDF denominator. Independent review recomputed all 24 hashes
and byte counts, verified 12 scenarios and 477 rendered PDF pages, and found no
blank pages, obvious clipping, or missing-glyph blocks. The accepted denominator
is unchanged by the separate Markdown candidate refresh.

## Retained adverse evidence

local_agent_website/vector/linux/first remains a failed execution group because
its OPENPGPKEY assessment was rejected as assessment_quote_mismatch. Its
compatible repeat used the same execution identity and completed with admissible
source-grounded findings. The original Q06 criterion requires one source-traced
observation with confidence and scope limits; it does not require every
stochastic observation to pass. Q06 and the cell therefore pass while the first
failure remains visible.

The second-user denial in Q03 is an operator-browser observation corroborated
by the subject-scoped database and exact enforcement code; it is not represented
as a separately captured raw HTTP response. Exact provider billing is also not
claimed.

## Accounting and export integrity

- Provider posts: **126**
- Unknown outcomes: **0**
- Ambiguous outcomes replayed: **0**
- Conservative spend accounting: **USD 64.744** under the operator-amended
  **USD 70** ceiling
- Exports: **12 Markdown / 12 HTML / 12 PDF**
- Composite 36-export privacy scan: **0** bounded credential or machine-local
  path hits
- Provider calls used by the aa2 candidate refresh: **0**

## Principal evidence

| Evidence | SHA-256 |
|---|---|
| Q02 identity-bound admission | 5f1b547b2c08bbf418ba699f5c9f301cc07b7a074b55899b78a621e9b30069cf |
| Q03 fresh two-user isolation | df705cf4ae27c0173a629a640ed75a0817ff4d26ca274568ed416eb6b6a85b81 |
| Q19 operator acceptance | 23280a8814663cefe4542a820470f50c721a1f9bf7cf7569d22956c41b9079b6 |
| PR #95 compatibility bridge | ec4207bf23c61514af3f4c904b42d7b592f68e2c75a5e77c169938ecd5009fde |
| Bridge validation V2 | 801ebc084303e1a63aef88ad26a3cec3c545c94f7e49d41881d13cdbbae6d8bc |
| Bridge independent review | 034d80404b1db5d51d19b8e4c36968a163ae749afe486e0e0013298fb4cb26c1 |
| aa2 Q08 receipt | c4296ebeafa2e122def3b3534e0e7b8dbdc2b9163e38687c45249bdddaf54cc2 |
| aa2 Q08 report | 9b471a87fee15934d3f6ce15cf2eee39f29365c9ecfad308b4d878c446c2734e |
| aa2 output refresh | 46be4a2d87e233196dab5249620baeee2d13a22d8e6a4f22d53f6e19b1ed278a |
| Composite 36-export receipt | 4a9a845b56010c47e4dbec44ebcb4993c58619f3b872bcb827c3574e4e62db2e |
| Q19 unchanged-denominator audit | 101985adf2df163d46a91fd40330a8868000739a8f1e7c7d507ca1facf73cebe |

## Boundary

This is the final CF-05 product qualification decision. It does not claim public
deployment, launch, exact provider billing, defect-free behavior, native
terminal closeout, or superiority. Merge and issue closure remain separate
lifecycle evidence.
