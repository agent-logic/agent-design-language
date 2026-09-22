# v0.92.2 documentation and review handoff

**Draft for review; acceptance pending #916.** The operator authorized #917 documentation work while independent #915 qualification is still open. This packet supports that review; it is not a release approval or authorization to send private material externally.

## Candidate and scope

ADL implementation main: `2b047c8dc34e133ee1353ec265507a17c523e06f` (merged #1137/#1138). Website main: `a45e339c13b24716edbd3fadf29dddff36ffe02e` (merged CodeFriend.ai #18). The inherited #916 evidence checkpoint is `51c9fdee5a` and includes a test-only control-readiness correction; it is not a merged release candidate. This documentation branch builds on that checkpoint. [Manifest](HANDOFF_MANIFEST.json) binds each inspected document's bytes and the exact full parent revision, plus all 69 canonical task identities.

Review the current [quality decision](../issue-916/QUALITY_DECISION.json), [task ledger](../issue-916/TASK_LEDGER.json), [24-prerequisite assessment](../issue-916/PREREQUISITE_ACCEPTANCE.json) and [post-merge checks](../issue-916/POSTMERGE_VERIFICATION.json). The canonical [document inventory](../../CANONICAL_DOC_INVENTORY_v0.92.2.md) includes every milestone Markdown document recursively, beyond the original hand-picked planning list. The manifest binds that full set plus required machine-readable contracts and evidence. Historical study documents are retained unchanged; their statistics were not recomputed.

## What is established

- Merged ADL and website integration has 154 affected native tests and 141 website tests passing, with no failures or skips. The earlier 396-test native census belongs to its earlier source revision; do not add these overlapping denominators together.
- #916 planning parity passes 69 task identities and 298 negative fixtures. Five historical Runtime criteria replay against exact retained evidence; those results do not qualify a new integrated installed candidate.
- The repaired PAIR discovery endpoints answer successfully. No new inference or failover run is claimed. The historical PAIR experiment decision remains REPAIR; MLX has bounded smoke proof without review-speed superiority; speculative decoding remains repair/inconclusive.
- #912 and #913 satisfied the operator-accepted article-draft and revision-4 private PDF handoffs. Further editing and publication are separate work.
- Podcast #671 and live pilot #875 are now v0.93. Preserve all 69 original identities; distinguish explicit deferral from execution failure or completed pilot proof.

## What is not established

#915 must deliver independent installed-candidate proof for both repositories, both website modes, invitation-only access/user isolation, actual approved provider review, second-run behavior and rendered exports. Source tests, endpoint discovery, issue closure and a merged patch do not establish deployment or that qualification. The final installed binary identities, accepted manifest and #916 passing decision are absent. No external review should issue a release verdict from this draft. OBS-S3/#910 retains its separate deployment obligation. The #945 packet records acceptance of all twelve ARCH-ADR decisions, satisfying that specific #925 obligation without establishing product or release acceptance.

## Fresh-reader walkthrough

From the ADL repository root, using Python 3.9 or later:

```sh
python3 docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py
python3 docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py --self-test
python3 docs/milestones/v0.92.2/validate_planning.py --self-test
python3 docs/milestones/v0.92.2/adr/issue-911/validate_packet.py --self-test
python3 docs/milestones/v0.92.2/adr/issue-945/validate_packet.py --self-test
```

The handoff validator checks exact document digests, local linked-path availability, the 69 unique task mapping, 24 prerequisite rows, declared source identities and the pending-acceptance guard. It does not execute commands found in documents or contact external links. Planning validation reports its nonzero negative-fixture count. Per-target results are in #916's source-bound evidence. The component commands below are reconstructed from those target lists and repository manifests; they are not a retained shell transcript. Rerun affected tests when the candidate changes, rather than treating this draft as a permanent green receipt.

Read the manifest and verify its document inventory. Follow the quality decision and task ledger, sample a claimed pass and a not-proven row, then check release notes and feature requirements against that evidence. Required paths must resolve; private raw evidence must be made available before accepting a claim that requires it. Record P1/P2 findings before summary and identify the exact file, line, source revision, consequence and bounded correction. An independent walkthrough remains required before final handoff acceptance.

## Component reproduction

Use separate disposable checkouts with the required private repository access. Do not change a shared checkout to reproduce these results. These commands build local test artifacts and may download locked dependencies; they do not install a shared binary, deploy, or run paid qualification.

For native tests, use the ADL source revision `e240a1b804968f8e96be7ea6e5dbfbc2ac51ea83` recorded in [post-merge verification](../issue-916/POSTMERGE_VERIFICATION.json), including the unmerged #916 test-readiness correction. From that checkout's root, use Cargo/Rust matching its checked-in toolchain (where declared), default features, and the following explicit integration targets. Keep build output local to this disposable checkout. Tests require permission to bind local sockets. This is the 154-test target set; it is not the earlier 396-test census.

```sh
git rev-parse HEAD
CARGO_TARGET_DIR="$PWD/adl/target" cargo test --locked --manifest-path adl/Cargo.toml \
  --test codefriend_agent --test codefriend_agent_publication \
  --test codefriend_agent_receipt --test codefriend_baseline_repeat \
  --test codefriend_evidence --test codefriend_integration \
  --test codefriend_journey --test codefriend_journey_v2 \
  --test codefriend_review_assessments --test codefriend_server \
  --test codefriend_update_cycle
```

For website tests, use `agent-logic/codefriend.ai` at `a45e339c13b24716edbd3fadf29dddff36ffe02e`, Node.js 24 or later and npm. From that checkout's root:

```sh
git rev-parse HEAD
npm ci --ignore-scripts
npm test
```

Confirm the printed source revisions before running. Expected retained totals are 154 native and 141 website tests, with zero failures and zero ignored/skipped tests. Compare actual nonzero denominators and results, not elapsed times or log hashes (logs include environment-dependent paths/timings). These recipes have been checked against the source manifests and recorded target lists; they were not rerun as part of this docs-only checkpoint. Raw historical logs and precise historical host/toolchain details require the authorized evidence custodian. A different toolchain or platform is a new observation, not an exact replay claim.

## Access and evidence custody

Repository access is required for both private repositories; the operator grants it through the normal account process. Do not put tokens in URLs, command lines, reports or this packet. Tracked `.csdlc/evidence/864` creation records are available in the ADL checkout. Protected Runtime archives and review receipts remain in Git-local C-SDLC retention; an authorized custodian must supply the exact digest-bound records for raw replay. #916 recovered the #852 archive from retained closeout storage without modifying its original bytes. Private manuscript prose/PDF and retained statistical raw data are not copied into this handoff; their sanitized identity records define the limits of what can be reviewed here. If access is missing, mark the affected obligation not proven—never infer success or publish private bytes to make links work.

## Finding rubric and remaining steps

P1: false release/candidate acceptance, privacy/provenance breach, or missing required qualification presented as passed. P2: meaningful command/path drift, unsupported feature claim, missing required evidence or incorrect deferral routing. P3: clarity that impedes review without changing acceptance truth. Distinguish actual failure, missing evidence, historical limitation and recommendation.

Close documented findings, obtain independent documentation review and a fresh-reader walkthrough, then refresh this packet against #916's accepted decision. Publication finalization, internal/external release review, remediation and ceremony remain #918–#925 work in their declared order. This packet sends no external message and authorizes no deployment, provider spending or release.

## Prior-review checks applied to this milestone

Read Claude's [first](../../repository-decomposition/CLAUDE_REVIEW_1.md) and [corrected-candidate](../../repository-decomposition/CLAUDE_REVIEW_2.md) reviews. F10 remains resolved: target v3 independence is separate from the known retained v2-to-resilience dependency, and extraction cannot strand rollback support. The reviews are retained planning evidence, not permission to extract repositories.

Earlier Claude/external-review lessons were applied here: include nested packets, reconcile accepted decisions against proposal history, distinguish source-time tracker/custody observations from current state, keep immutable evidence separate from live claims, and supply actual reproduction commands. This pass corrected the #671/#875 scope drift, SIM pilot gate, twelve-decision #945 acceptance, merged website #18 status, manuscript custody date, and old #1064 candidate status. Older milestones were inspected only as review context and were not edited.

## Package metadata coverage

The [Cargo review](CARGO_MANIFEST_REVIEW.md) and [27-manifest audit](CARGO_MANIFEST_AUDIT.json) cover every tracked Cargo.toml, with 23 local dependency references checked. Main package versions remain 0.92.1 and independent components keep their declared versions. Release metadata selection belongs to #918; this handoff does not label existing binaries as a v0.92.2 release.
