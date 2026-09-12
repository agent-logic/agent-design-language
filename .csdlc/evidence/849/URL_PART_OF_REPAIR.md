# URL-form PartOf review repair

The user identified P2 ambiguity at PR #952 head `5b68a26681`: a canonical
`Part of #505` plus `Part of https://github.com/other/repo/issues/506` was
accepted, as was Closing plus a URL-form PartOf directive. Detection only
counted PartOf references containing `#`.

Repair source: `01488a46877fdaf045929847223fc2b285a50a49`. Detection now also
counts URL-form references, including angle and Markdown wrappers and labels
with spaces. Admission still requires a single canonical qualified reference
(or same-repository short reference); URL-only, conflicting and mixed directives
are rejected before any durable intent or merge PUT.

The focused regression failed before the fix on the canonical-plus-conflicting
URL case. After repair, all 17 merge tests passed in 10.60 seconds, including 32 new
cases across 2 modes, 2 aliases, 4 wrappers and canonical present/absent. Each new case
asserts rejection, zero PUTs and absent intent. Existing canonical/split-repo
positive and replay cases also passed. Clippy all-targets with -D warnings
passed in 3.37 seconds; formatting and diff hygiene passed.

Independent reviewer sprint8_909 accepted this exact source with no actionable
findings, inspected the 17-test log and 32-case rejection assertions, and verified
canonical admission/qualification remain unchanged. Local logs are preserved
under `.adl/runs/849/url-part-of-{before,after,clippy}.log`.

PVF remains the existing deterministic fake authenticated transport/local Git
required owner contract. No full-suite or green CI claim is made: the separate
UTS inventory failure remains recorded and unapproved for repair. No shared
binary installation, live merge or inventory changes were performed.
