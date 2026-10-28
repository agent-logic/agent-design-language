# RUST-01 recursive source audit (#836 / TPR-004)

PR #547 delivered behavior-preserving owner-boundary decomposition, not a code-size reduction quota. Its merged revision is `e986de6d06aacd385de93dd033def77a718c1581`; its first parent, `a71d699d52831b32bb68ed9c7c7e837925949de4`, is the baseline. This pair isolates that refactor, rather than attributing later milestone changes to it.

The resilience family grew from **5,278 to 5,995 physical lines (+717)** across one to twelve Rust files. The complete declared scope, including recursively tracked `adl/tests/**/*.rs`, grew from **26,911 to 27,628 (+717)**. The 64-line facade is a file-size observation, not overall code reduction. Unchanged test files remain in the denominator.

## Reproduce and validate

Run from the repository root using Python 3 and Git:

```sh
python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --baseline a71d699d52831b32bb68ed9c7c7e837925949de4 --candidate e986de6d06aacd385de93dd033def77a718c1581 --out docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json
python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --check docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json
python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/test_measure.py
```

`measurement.json` and `measurement.md` are deterministic outputs without timestamps or host paths. The report records Git version because heuristic rename detection is tool-dependent. Reproduction requires the same Git/Python algorithm implementation; byte stability is tested by generating twice with identical inputs. Git blobs, not working-tree files or top-level globs, supply the recursive source inventory. Physical lines include blanks and comments; an unterminated final line counts once.

Gross per-path additions and deletions are reported separately. Identical nonblank lines paired across changed paths remain included in both gross totals and are labeled relocation candidates. Repeated syntax can match accidentally; these pairs do not establish semantic movement. Git rename evidence uses a disclosed 50% similarity threshold. File blob identities and unchanged lines are recorded independently. No matching or counting result is behavior proof.

## Current release-document audit

The current v0.92.1 README, DESIGN, DECISIONS, WBS, milestone checklist, quality gate, feature-proof coverage and Rust resilience feature consistently promise a behavior-preserving owner-boundary refactor, with no LoC quota. The feature now links this measured correction. The release plan names RUST-01 as a gate without making a numerical reduction claim. Current-status evidence describes retained behavior proof, not a line-deletion target.

The retained #499 SOR describes a 5,278-to-64 facade change. It is historical, file-local evidence. Its `validate-validation-impact.rb` checks facade size and immediate submodule presence; it does not measure recursive source reduction or prove a smaller validation denominator. Those historical records remain intact. This packet supplies the missing recursive measurement and explicitly limits their interpretation.

## Guardrail and proof boundary

`--check` independently recomputes every scoped file, total, revision and relocation, then checks the rendered Markdown. It rejects a top-level-only scope, shortened/symbolic revision, omitted nested file, altered total, unsupported behavioral-pass flag or mismatched report claim. The focused negative fixtures exercise these cases. Future quantitative claims must cite this exact-revision recursive report or regenerate equivalent evidence; a facade count alone cannot support them.

PVF: deterministic local contract proof, small temporary Git repositories and local CPU/disk; required for #836 evidence. No runtime source changed, no Rust suite or coverage run is claimed, and source-size evidence does not substitute for RUST-01 API/behavior proof.
