# Medium Series — September 2026

The current thirteen-article Medium series, one article per day, for the Agent
Logic soft launch.

These files are the canonical copy of what goes to Medium. They are byte-identical
to the review copies in the `medium-drafts` Google Drive folder.

## Running Order

| Day | File | Subject |
|-----|------|---------|
| 1 | `01-what-is-adl.md` | The company and the three-layer platform |
| 2 | `02-adl-runtime-and-csm.md` | The runtime — AEE, determinism, cost control |
| 3 | `03-uts-and-acc.md` | Governed tools: UTS + ACC, and how MCP composes |
| 4 | `04-the-freedom-gate.md` | Execution authority and trajectory validation |
| 5 | `05-the-cognitive-sdlc.md` | C-SDLC — typed lifecycle records |
| 6 | `06-codefriend.md` | The product |
| 7 | `07-continuous-adversarial-verification.md` | CAV |
| 8 | `08-cost-and-resource-governance.md` | Budgets, ceilings, spend attribution |
| 9 | `09-routing-cheapest-capable.md` | Routing and escalation |
| 10 | `10-godel-agents-and-ghb.md` | Governed behavior-change management |
| 11 | `11-cognitive-spacetime-manifold.md` | CSM and the Principle of Cognitive Relativity |
| 12 | `12-social-intelligence.md` | Theory of mind as evidence-bound runtime state |
| 13 | `13-whats-next-for-adl.md` | Roadmap, claim standard, A2A positioning |

The arc runs value → mechanism → product → frontier. Speculative and
philosophical material sits in the back half, where it rewards readers already
convinced.

## Relationship To The v0.92 Packet

This series supersedes the ten-article packet in the sibling numbered directories,
which was the v0.92 editorial work. That packet is retained as the record of the
earlier draft round. The running order, titles, and article count all changed;
CodeFriend was split into its own piece and a routing/escalation article was added.

## Packet Shape

These are plain article files. They do not carry the `SOURCE_PACKET.md` /
`ARTICLE.md` / `EDITORIAL_REVIEW.md` shape that
`.csdlc/evidence/5844/validate-article-series.rb` enforces, and that validator
does not inspect this directory — it checks the ten original slugs by name.

If these become the validated packet, the validator's slug list and the required
per-article artifacts both need updating first.

## Draft Completion

Claude completed all thirteen article drafts. On 2026-09-20, all thirteen
filenames, byte counts, and SHA-256 content hashes matched the Google Drive
`medium-drafts` review copies. The operator accepted this drafting scope for
issue #912 closure. Further article review and editing will be tracked in a
separate issue to be opened later. This records draft completion, not final
editorial approval or external publication.

## Before Publication

Open items are tracked outside this directory:

- repo status drift, including the birthday claim in `docs/milestones/v0.92/README.md`
  and `docs/planning/ADL_FEATURE_LIST.md` — these contradict the birthday feature
  doc, which correctly states the birth event is not claimed
- `PUBLICATION_DISPOSITION.md` still records external publication as unauthorized
- the supporting Principle of Cognitive Relativity and CSM terminology updates
  are included in `docs/GLOSSARY.md` and `docs/explainers/CSM.md`; their editorial
  review continues with the articles
- AL-001 shows the card order as `STP → SIP → SOR`; the correct order is
  `SIP → STP → SPP → SRP → SOR`
