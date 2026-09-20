# Bounded Rust syntax admission

Architecture and local fitness share one resource boundary before parsing inert
admitted Rust source. The previous 128-unit whole-file budget rejected ordinary
files; it did not distinguish long files from deep recursive expressions.

The admission policy is:

| Resource | Limit |
| --- | --- |
| Original UTF-8 source | 400KiB per file, including comments and shebang |
| Lexical delimiter or nested block-comment depth | 32 |
| Parsed token-tree entries | 32,768 per file |
| Cumulative ancestor statement span | 2,048 token entries |
| Recursive parsing, visiting and AST destruction stack | 64MiB reserved in one worker |
| Active plus waiting parser callers | 32; one worker at a time |

A statement span counts all immediate tokens in a token-tree group between
semicolons. Nested groups do not reset their parent's span. Each child's budget
includes its ancestors' maximum spans. This bounds flat unary/operator/type and
else-if chains as well as delimiter nesting; it is conservative and can still
reject unusually complex valid Rust. Source literals and comments are opaque to
the iterative lexical delimiter check. Tokenization, parsing, visiting, and
recursive destruction all happen inside the same worker; only inert projections
leave it. The worker runs no source, macros, scripts, compilers or providers.
Worker failure and exhausted admission return the existing explicit complexity
failure, and syntax failure remains a parse failure. Unsupported constructs keep
their existing unknown/incomplete classifications.

These are input and concurrency bounds, not an OS memory quota or a preemptive
wall-clock timeout. The 64MiB stack reservation is not a measured RAM claim.
A source-derived probe on the unchanged #915 ADL `review/lanes.rs` and all five
pinned Vector Rust files accepted every file and exercised full parse, visitor
and drop. The 89,969-byte Vector parser took approximately 39ms in the first local
debug probe; a separate run measured a 9.4MB process peak RSS. These observations
are local resource evidence, not a portable performance guarantee or product
qualification. Source and command hashes are retained in #1100 evidence.

Tests cover ordinary multi-item files, the unchanged ADL source, comments/raw
strings/chars/lifetimes, malformed input, oversized source, deep groups, nested
comments, long unary/type/else-if chains, admitted near-boundary recursive work,
worker panic recovery and reentrant admission rejection. Architecture and fitness
integration tests retain honest result status. #915 must independently repeat the
installed failing commands on the final candidate; parser admission does not
resolve separate unsupported-language or missing-module coverage gaps.
