# Recursive Rust source-size evidence

Baseline: `a71d699d52831b32bb68ed9c7c7e837925949de4`

Candidate: `e986de6d06aacd385de93dd033def77a718c1581`

Scope: `adl/src/resilience.rs`, recursive `adl/src/resilience/`, recursive `adl/tests/`; tracked `.rs` files only.

| Measure | Lines |
|---|---:|
| baseline | 26911 |
| candidate | 27628 |
| added | 5992 |
| deleted | 5275 |
| unchanged | 21636 |
| net_change | 717 |
| cross_path_identical_nonblank_line_pairs | 2786 |

Resilience family: 5,278 → 5,995 lines. Recursive family net change: +717 lines. A smaller facade alone is not evidence of overall code reduction.

Git blobs; LF physical lines including blanks/comments and unterminated final line; per-path SequenceMatcher autojunk=false.

Lexical exact nonblank line pairs in deleted/added diff ranges, different paths, sorted greedy one-to-one; ambiguous repeated content is only a relocation candidate; pairs remain in gross added/deleted totals; consecutive pairs encoded as runs with SHA-256 of concatenated hexadecimal line hashes.

Git diff --find-renames=50% --name-status -z; heuristic evidence, not semantic proof.

File-by-file inventories, blob identities and relocation locators are retained in `measurement.json`. Size, matching lines and rename detection do not prove behavior preservation or narrower validation impact. No LoC quota applies.
