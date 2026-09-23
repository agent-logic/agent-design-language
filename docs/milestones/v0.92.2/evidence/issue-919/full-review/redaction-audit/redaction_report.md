# Redaction And Evidence Audit

## Verdict

- Status: fail
- Files scanned: 143
- Blockers: 1
- Warnings: 0
- Info: 0

## Publication Recommendation

- Recommendation: block_publication
- Audience: public_candidate

## Scope

- Artifact root: full-review
- Started at: 2026-09-23T17:35:10Z
- Completed at: 2026-09-23T17:35:11Z

## Findings

- [blocker] private_host_path in lanes/tests-history/path.json:1 - Private host path appears in an artifact intended to be portable. Sample: `{"safe":"/Users/example/config"}`

## Evidence Boundary Notes

- Secret-like samples are masked.
- Paths are relative to the audited artifact root.
- This audit does not mutate source artifacts.
- This audit does not replace specialist code, security, docs, or test review.

## Required Follow-Up

- Block sharing until the owning artifact producer removes or replaces unsafe evidence.
