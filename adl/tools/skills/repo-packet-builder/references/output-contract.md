# Output Contract

The repo packet builder produces a CodeBuddy review packet root.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/
```

## Required Artifacts

### run_manifest.json

Required fields:

- `schema`
- `run_id`
- `repo_name`
- `repo_ref`
- `review_mode`
- `started_at`
- `completed_at`
- `skills_used`
- `artifact_root`
- `privacy_mode`
- `publication_allowed`

### repo_scope.md

Required sections:

- `# Repo Scope`
- `## Scope Reviewed`
- `## Included Paths`
- `## Excluded Paths`
- `## Non-Reviewed Surfaces`
- `## Assumptions`
- `## Known Limits`
- `## Next Specialist Lanes`

### repo_inventory.json

Required fields:

- `schema`
- `repo_name`
- `file_count`
- `extension_counts`
- `top_level_dirs`
- `manifests`
- `docs`
- `tests`
- `ci`
- `likely_code_roots`
- `largest_files`
- `largest_code_files`

### evidence_index.json

Required fields:

- `schema`
- `evidence`

Each evidence entry should include:

- `path`
- `category`
- `line_count`
- `reason`
- `specialist_lanes`

### specialist_assignments.json

Required fields:

- `schema`
- `assignments`

Default lanes:

- `code`
- `security`
- `tests`
- `docs`
- `architecture`
- `dependencies`
- `diagrams`
- `redaction`
- `synthesis`

### lane_denominators.json

Required: `source_paths`, `source_count`, `source_sha256`, and `lanes` for code,
security, tests, docs, architecture, dependencies, and diagrams. Each lane records
its complete eligible `source_paths`/count/SHA-256, selected paths/count/SHA-256,
`exclusions.not_eligible`, `exclusions.not_sampled`, `selection_rule`, and `coverage`.
Digests hash sorted UTF-8 paths with one trailing LF per path; the empty set hashes
empty bytes. The source inventory includes all scoped Git paths, including deleted
paths in a diff; unreadable files have line_count -1. Diff mode compares the given
base to HEAD. Path scope intersects that inventory.

Classification scans the complete denominator. Manual assignments are at most 30
paths per lane, selected by category round-robin and then lexical path order. Code
requires code-category paths; dependency manifests alone cannot satisfy code review.
Security includes code, tests, dependencies and CI, including Rust source.
An absent lane has an explicit zero denominator; a present lane cannot have an
empty assignment. Selection is routing evidence, never semantic review completion.

Validate against an independently retained complete path inventory:

```sh
python3 adl/tools/skills/repo-packet-builder/scripts/validate_repo_packet.py PACKET --source-inventory SOURCE_PATHS.txt
```

The validator rejects inconsistent denominators, exclusions, digests, categories,
and assignments. Do not generate the independent inventory from the packet being
validated: that would fail to detect an omitted source path.

## Rules

- Use repo-relative paths.
- Do not write absolute host paths into packet artifacts.
- Do not include source excerpts by default.
- Do not claim review findings.
- Do not claim publication safety before redaction review.
- Do not mutate the reviewed repository.

## Success Summary

When complete, report:

- artifact root
- review mode
- generated artifacts
- included/excluded scope summary
- specialist lanes prepared
- caveats and next skill

