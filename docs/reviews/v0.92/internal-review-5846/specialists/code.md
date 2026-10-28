# Code Correctness Specialist Review

## Metadata

- Skill: `repo-review-code`
- Reviewer identity: Codex code specialist (`/root/review_313_code`)
- Target: `agent-logic/agent-design-language` at exact revision `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Date: 2026-08-25 UTC
- Artifact: `docs/reviews/v0.92/internal-review-5846/specialists/code.md`
- Review mode: repository packet, correctness lane, targeted inspection
- Finding count: 2 (`P1`: 1, `P2`: 1)

## Findings

- P1: Global evidence truncation starves specialist lanes of their actual review surfaces
  File: `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py:276`
  Role: code
  Scenario: Build a repository packet for this repository, where hundreds of tracked lockfiles and manifests sort ahead of executable code and tests. `build_evidence` assigns manifests a score of 50, globally truncates the scored list to 120 entries at line 320, and only afterward derives and caps each lane independently in `assignments_from_evidence`.
  Impact: The packet reports that specialist lanes are prepared while supplying no executable source to the code lane and no files at all to the tests lane. A reviewer following the assignment artifact cannot perform the intended repository review, so downstream synthesis can mistake structurally present but substantively empty lanes for coverage.
  Evidence: At the exact target, `docs/reviews/v0.92/internal-review-5846/specialist_assignments.json` assigns 30 paths to `code`; every one is a `.csdlc/locks/*.lock` file. Its `tests` assignment is empty. In contrast, `docs/reviews/v0.92/internal-review-5846/repo_inventory.json` reports 889 Rust, 242 Python, 304 Ruby, and 546 shell files, and its sampled test inventory is non-empty. The scoring and global truncation are visible at `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py:284-320`; lane derivation occurs only later at lines 323-333. Build and cap evidence per lane, or otherwise guarantee representative non-empty lane coverage before emitting the assignment packet.

- P2: Worktree detection is coupled to an obsolete directory-name convention
  File: `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py:260`
  Role: code
  Disposition: disputed for this packet after exact-source reconciliation.
  Scenario: Build a packet from an ADL issue worktree under the repository-mandated `<FastWork>/adl-worktrees/...` parent.
  Original concern: The implementation determines worktree state solely with `".worktrees" in repo_root.parts` at lines 260 and 455, which is not a reliable general Git-worktree test.
  Correction: `run_manifest.json` and `packet_assignment_recheck.md` prove that the clean primary checkout was the packet source and the issue worktree was only the output location. Therefore `is_worktree: false` is truthful for this packet, and the original claim that this execution exercised the weak detector is withdrawn. The generic detector concern remains a tooling hypothesis, not a finding proven by WP-25 evidence.

## Assumptions

- The packet's declared 23,622 tracked-file inventory is the review denominator.
- Exact target revision `c6792e54df1db5969fa28c59b6dfe4c714ed5559` is authoritative even though the shared issue worktree advanced with issue-313 lifecycle records during concurrent review; product files inspected are unchanged from the target unless read explicitly with `git show <target>:<path>`.
- Specialist assignments are intended to be actionable review routing, not merely schema placeholders, as described by the multi-agent review suite and packet output contract.

## Reviewed Surfaces

- Packet metadata and routing artifacts: all 5 packet files present before specialist output (`run_manifest.json`, `repo_scope.md`, `repo_inventory.json`, `evidence_index.json`, `specialist_assignments.json`).
- Packet-builder implementation: tracked-file discovery, classification, evidence scoring, global truncation, lane derivation, inventory metadata, and run-manifest metadata in `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py`.
- Packet-builder contract test: `adl/tools/test_repo_packet_builder_skill_contracts.sh`.
- Representative high-risk production surfaces at the exact target: resident ACC/tool governance in `adl/src/resident_tool_execution.rs`, resident authority validation in `adl-runtime/src/resident_agent.rs`, adaptive-learning validation in `adl-runtime-kernel/src/adaptive_learning.rs`, Memory Palace record conversion in `adl-runtime-kernel/src/memory_palace.rs`, and kernel supervision lifecycle in `adl-runtime-kernel/src/supervisor.rs`.
- Release delta orientation: executable paths changed between `v0.91.8` and the exact target, emphasizing Runtime, Runtime kernel, ADL, C-SDLC v2, and AWS validation surfaces (402 matching Rust/Python/Ruby/shell paths).

## Scope And Denominator

- Declared repository denominator: 23,622 tracked files.
- Inventory composition used for orientation: 889 `.rs`, 242 `.py`, 304 `.rb`, 546 `.sh`, 341 manifests, 17 CI files, 40 sampled docs, and 40 sampled tests.
- Deep inspection was targeted to packet-construction correctness and representative production runtime authority paths; this was not a line-by-line audit of all 23,622 files.
- Excluded: generated/vendor/cache directories declared by the packet, external services, paid cloud execution, and issue `#269`.

## Validation Performed

- `git cat-file -t c6792e54df1db5969fa28c59b6dfe4c714ed5559` — proved the requested target exists as a commit.
- `git show -s --format='%H%n%P%n%ci%n%s' c6792e54df1db5969fa28c59b6dfe4c714ed5559` — captured exact target identity and provenance.
- `git diff --name-status c6792e54df1db5969fa28c59b6dfe4c714ed5559..HEAD` — established that concurrent local advancement was confined to issue-313 lifecycle preparation paths.
- `git diff --name-only v0.91.8..c6792e54df1db5969fa28c59b6dfe4c714ed5559` filtered to executable suffixes — oriented the release code denominator (402 paths).
- Parsed `specialist_assignments.json` — observed `code: 30`, `tests: 0`, with all 30 code assignments ending in `.lock`.
- Inspected exact-target production files with `git show c6792e54df1db5969fa28c59b6dfe4c714ed5559:<path>` so concurrent worktree commits could not alter reviewed source.
- No broad compilation or runtime suite was run in this lane; the packet defect makes its generated code/test routing non-proving, and concurrent review lanes share the worktree.

## Residual Risk

- The generated packet did not provide a representative executable-code assignment, so this lane manually inspected the builder and selected high-risk Runtime surfaces. Most of the 402 release-delta executable paths remain outside deep manual review here.
- No runtime, concurrency, provider, AWS, or end-to-end behavior was executed by this specialist.
- Security, test adequacy, documentation truth, architecture, and dependency risks belong to their respective specialist lanes and synthesis.
- The two findings must be remediated and the packet regenerated before the packet can serve as trustworthy denominator/routing evidence for a claimed complete internal review.
