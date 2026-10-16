# Post-split documentation delivery — #1269

Status as of 2026-10-15: all **13 repository documentation PRs are merged**, and
[ADL #1269](https://github.com/agent-logic/agent-design-language/issues/1269) is
closed (2026-10-15T17:25:01Z). This is documentation delivery, not final release
acceptance or a claim that every repository completed native reconciliation.

The operator expanded the original README-only issue to existing AGENTS.md,
REVIEW.md and directly related onboarding/review guidance. The
[document inventory](DOCUMENTATION_INVENTORY_1269.md) lists 27 primary documents
and 150 related candidates. A reference match is not automatic rewrite scope.
Historical records, manuscripts, licenses and implementation ownership remain
unchanged.

## Repository delivery checklist

All rows received bounded independent source/documentation review. Revisions
identify the historical content entering each PR; merge identities are listed below. The ADL revision identifies its initial
source-guidance commit; this checklist is reviewed at its containing Git commit
and the corresponding current PR head, avoiding a self-referential commit hash.

| Repository | Reviewed candidate | Changed surfaces | Delivery |
| --- | --- | --- | --- |
| `agent-design-language` | `2ac6fb7c51c1ef0fd6a2808877fe59d46ed8fec4` | `AGENTS.md`, `CONTRIBUTING.md`, `README.md`, `REVIEW.md`, `docs/README.md`, `docs/REPOSITORIES.md`, `docs/csdlc-v3/CURRENT_AUTHORITY.md`, `docs/csdlc-v3/INTENT_COMMANDS.md`, `docs/default_workflow.md`, `docs/onboarding.md`, `docs/repository-split/DOCUMENTATION_INVENTORY_1269.md` | [PR #1274](https://github.com/agent-logic/agent-design-language/pull/1274) |
| `cognitive-sdlc` | `f23598055bb73c4dba1d92bb89e695ec82e17402` | `AGENTS.md`, `README.md`, `csdlc-v3/README.md`, `tools/ci-test-dependencies.json` | [PR #292](https://github.com/agent-logic/cognitive-sdlc/pull/292) |
| `agent-logic-runtime` | `2988d1d836824dfd3a094c1d7da86d8de4b0a2b5` | `README.md` | [PR #181](https://github.com/agent-logic/agent-logic-runtime/pull/181) |
| `codefriend` | `9bbd0546b74de897e8ab1efdafef1218e4016ffa` | `README.md` | [PR #117](https://github.com/agent-logic/codefriend/pull/117) |
| `agent-logic-infrastructure` | `11eb1ed2986b100ff6ca04003a93d59ac7d16ae4` | `README.md` | [PR #40](https://github.com/agent-logic/agent-logic-infrastructure/pull/40) |
| `agent-logic-enterprise-security` | `f59969c35260c08fab03bc0d2dc840ca867fc5d6` | `README.md` | [PR #8](https://github.com/agent-logic/agent-logic-enterprise-security/pull/8) |
| `agent-logic.ai` | `318a7328cac86bcec8bfe91804333aae1af94a07` | `AGENTS.md`, `README.md` | [PR #99](https://github.com/agent-logic/agent-logic.ai/pull/99) |
| `codefriend.ai` | `42a09a59d06c21a5f24e79855c5a1d35e93413ab` | `AGENTS.md`, `README.md` | [PR #36](https://github.com/agent-logic/codefriend.ai/pull/36) |
| `universal-tool-schema` | `473c981edb116ec7c7ae8ff5bfa427da247b1f59` | `AGENTS.md`, `README.md` | [PR #2](https://github.com/agent-logic/universal-tool-schema/pull/2) |
| `strategic-cognitive-reserve` | `47f82279ae4354dc95cce6ac840e6a47c5e6a42e` | `AGENTS.md`, `README.md` | [PR #29](https://github.com/agent-logic/strategic-cognitive-reserve/pull/29) |
| `godel-hadamard-bayes-paper` | `1f9ee614d8972eb9616f26a9ba968126c27d373b` | `AGENTS.md`, `README.md` | [PR #2](https://github.com/agent-logic/godel-hadamard-bayes-paper/pull/2) |
| `cognitive-sdlc-paper` | `035106af87848bbcbc6a66875ed795bda634984c` | `AGENTS.md`, `README.md` | [PR #2](https://github.com/agent-logic/cognitive-sdlc-paper/pull/2) |
| `general-intelligence-paper-private` | `3b3438160ac406bd600d44dbcb44b3f3c65dd4e7` | `AGENTS.md`, `README.md` | [PR #2](https://github.com/agent-logic/general-intelligence-paper-private/pull/2) |

## Primary-document dispositions

24 of the 27 primary documents are updated in the linked PRs. The three retained
primary documents have explicit no-change decisions:

- ADL `SECURITY.md`: private vulnerability reporting and disclosure policy remain
  applicable. Its versioned architecture references retain their historical scope.
- UTS `REVIEW.md`: the descriptive-schema/non-authority distinction, focused review
  checklist and publication restrictions remain applicable. Repository privacy was
  checked; this refresh grants no public-release approval.
- Enterprise Security `README.product.md`: the Runtime interface `=0.1.0`, verified
  artifact entrypoint, four unit/six installed conformance denominator and lack of
  backend/release acceptance match the manifest and validation source.

Missing root agent/review files were candidates, not a requirement to create
boilerplate in every repository. Related guidance was changed where needed for
current entrypoints; dated architecture, review, ADR, milestone, custody and
manuscript records remain evidence of their original state. Their appearance in
the inventory does not assert that every related candidate required a rewrite.

## Validation and evidence boundaries

Performed locally: candidate `git diff --check`, inline Markdown local-target
checks, current manifest/entrypoint/source inspection and independent exact-head
review. Native candidate proof/review/publication ran for ADL, Cognitive-SDLC,
Runtime and CodeFriend. Cognitive-SDLC additionally passed 35 existing selector
tests after CI found its package README missing from the dependency map; the fix
adds that exact path to the existing prose contract lane without changing gates.

The documented build, installed-product and provider commands were inspected,
not executed for this documentation task. No local broad builds, model/provider
runs, deployments, package replacements or manuscript compilation were performed.
Repository-selected GitHub CI is separate integration evidence; skipped or absent
jobs are not passed tests. Exact current check results belong to each linked PR.

CodeFriend review corrected the required shared-library version to `0.1.1`, the
complete installed template-resource layout, and the 31-case smoke denominator.
The seeded-preview reservation boundary remains distinct from the sequential
public-operation/final-export flow. No new product acceptance is claimed.

## Adapter follow-up and bounded delivery exception

[ADL #1275](https://github.com/agent-logic/agent-design-language/issues/1275)
owns repository-adapter migration and native admission/setup reconciliation.
Several retained adapters still name ADL source discovery or retired conductors;
the documentation now identifies those fields as legacy metadata, not authority.
This refresh does not change adapter configuration or repair authenticated state.

The operator explicitly authorized a one-time `gh` exception for documentation
tracking issues and PR publication in Infrastructure, Enterprise Security, both
websites, UTS, Strategic Cognitive Reserve and the three paper repositories.
Those nine rows used isolated worktrees and independent review. The exception
does not authorize merge or establish native finish. The four working native
repository routes continued to use C-SDLC.

Canonical publication targets were authenticated as `agent-logic` repositories.
Three older local checkouts retained personal-account origins; their documentation
commits were checked against the actual Agent Logic base before publication.
One branch-only UTS push reached the legacy personal origin during discovery;
no PR or merge occurred there, and that unused branch was retained. No personal
source changes were imported into the canonical documentation candidates.

All thirteen merges and parent #1269 closure are now observed; the ADL repository
map is on `main` through PR #1274. Native reconciliation and cleanup where
applicable remain separate from that delivered documentation outcome. The original
publication exception above did not authorize these later merges; their separately
authorized readbacks are recorded below. Four repositories had no CI and used
explicitly approved review-only merge exceptions; absent checks are not passed tests.


## Confirmed merge readback

The authenticated observations retained in the [#1232 assessment at `cd5e9098`](https://github.com/agent-logic/agent-design-language/blob/cd5e9098d9d1a0f65d460e43d1e000f57683a33b/docs/milestones/v0.93.1/quality-gate-1232/EVIDENCE_MATRIX.json), gate `ADL-1269`, supersede the original publication-time open status. These observations do not depend on PR #1280 being merged. Its overall quality decision remains `NOT_READY_FOR_FINAL_RELEASE_ACCEPTANCE`.

| Repository / PR | Merge commit | Merged at (UTC) |
| --- | --- | --- |
| [agent-logic/agent-design-language #1274](https://github.com/agent-logic/agent-design-language/pull/1274) | `fa66dfb20ffde07e455f0f9fdce7ef32c3786796` | 2026-10-15T17:24:59Z |
| [agent-logic/cognitive-sdlc #292](https://github.com/agent-logic/cognitive-sdlc/pull/292) | `d77fff2eef02305f077f885e001ea8300933be1a` | 2026-10-14T19:10:42Z |
| [agent-logic/agent-logic-runtime #181](https://github.com/agent-logic/agent-logic-runtime/pull/181) | `e13b2b03165c94171b2160d01c90d24d4fab3a98` | 2026-10-14T19:11:33Z |
| [agent-logic/codefriend #117](https://github.com/agent-logic/codefriend/pull/117) | `69677b4443ecd1472a66150bc33481b7d0b8fbb6` | 2026-10-14T19:12:19Z |
| [agent-logic/agent-logic-infrastructure #40](https://github.com/agent-logic/agent-logic-infrastructure/pull/40) | `6aa48960dde398cd459b487a9452dc9a7b3c757f` | 2026-10-14T19:32:57Z |
| [agent-logic/agent-logic-enterprise-security #8](https://github.com/agent-logic/agent-logic-enterprise-security/pull/8) | `51a1c02e1ed273b63b397b3371d0230158a0450c` | 2026-10-14T19:33:04Z |
| [agent-logic/agent-logic.ai #99](https://github.com/agent-logic/agent-logic.ai/pull/99) | `4756b251b7d144bf4d6c6d25ea8403a49f8288a3` | 2026-10-14T19:33:10Z |
| [agent-logic/codefriend.ai #36](https://github.com/agent-logic/codefriend.ai/pull/36) | `106d202bc11ef5c2a0f367429770b08b3c869ea4` | 2026-10-14T19:33:16Z |
| [agent-logic/universal-tool-schema #2](https://github.com/agent-logic/universal-tool-schema/pull/2) | `dea902201e1823c42c928ea0bc9b8bc36c842210` | 2026-10-15T17:20:40Z |
| [agent-logic/strategic-cognitive-reserve #29](https://github.com/agent-logic/strategic-cognitive-reserve/pull/29) | `f5cf19a986b554035f02a698ee2e1388f05620a4` | 2026-10-14T19:33:22Z |
| [agent-logic/godel-hadamard-bayes-paper #2](https://github.com/agent-logic/godel-hadamard-bayes-paper/pull/2) | `2ac23cdd95fb76ecd9f9af70ff1e3b78d5286361` | 2026-10-15T17:20:46Z |
| [agent-logic/cognitive-sdlc-paper #2](https://github.com/agent-logic/cognitive-sdlc-paper/pull/2) | `77f7a19e94f1743dbcbb3f8a5a50fa759f7a8080` | 2026-10-15T17:20:53Z |
| [agent-logic/general-intelligence-paper-private #2](https://github.com/agent-logic/general-intelligence-paper-private/pull/2) | `536f2cbdc1de0deac6371076c2caf7eebd450992` | 2026-10-15T17:20:59Z |
