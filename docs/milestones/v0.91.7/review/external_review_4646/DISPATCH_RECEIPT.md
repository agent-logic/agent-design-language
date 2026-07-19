# WP-19 External Review Dispatch Receipt

Status: superseded_stale_target

Issue: #4646

This receipt is outside `REVIEW_CORPUS.v1.txt` and preserves the historical
target recorded by PR #5579. Later merged evidence changed the required corpus,
so this receipt no longer proves current WP-19 completion. A replacement target
and digest must be recorded immediately before the next external dispatch.

| Field | Value |
| --- | --- |
| Repository | `danielbaustin/agent-design-language` |
| Review owner | WP-19 / #4646 |
| Pull request or branch | PR `#5579`; `codex/4646-v0917-external-review` |
| Base branch | `main` |
| Exact target commit SHA | `bd1c12537b28122e187ce1ba9a19349731fd2825` |
| Review corpus digest | `8ae1ddd98b86ded8ef52018d0df4eb045455f586292b90954fe0056e8d18e37c` |
| Corpus size | 29 manifest entries expanding to 66 tracked blobs |
| Historical review recorded | 2026-07-19 |
| Superseded because | Required corpus changed after the target, including merged #5571 publication-boundary evidence. |
| Replacement review | Not run |

The digest is SHA-256 over the locale-sorted `git ls-tree -r` records for the
paths in `REVIEW_CORPUS.v1.txt` at the exact target commit.
