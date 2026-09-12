# #879 review record

At initial recording, independent exact-head review was pending. Source candidate: `b91ace4be5c67e00455df62dd6ea4f3b5afb5787`.

## Preserved early root finding R1 — cumulative metadata memory bound

Original observation: the tree cache could retain 4096 responses of up to 2 MiB each despite a 1 MiB source budget; per-response bounds and deadlines alone were not a practical memory bound.

Disposition: fixed. Production Transport now counts all response bytes against an 8 MiB per-transport capture ceiling before parsing/caching. The focused test `aggregate_metadata_is_bounded_across_individually_valid_responses` executes multiple valid HTTP responses below the individual response ceiling and verifies the aggregate fail-closed diagnostic. It passed in both candidate and installed-binary proof runs. No original finding has been removed or rewritten as a pass.

## Local validation corrections

The first test run exposed inherited nonblocking accepted sockets in the macOS fixture. The fixture explicitly restores blocking sockets; all cases then passed. Clippy found a test assertion using len()>0; corrected to !is_empty(), and focused clippy passed. These are retained execution corrections, not independent approval.

## Independent source review and R2 publication-card P2

Reviewer `review_836` approved source at `edc4d0a7df7b3c55c20f65d6a08c9a53affaa502` with no actionable production findings.

Original R2 finding: rendered SOR retained summary, execution, PVF and verification placeholders; SRP review result and source references were unfilled. The summary aliases previously edited were not the actual template semantic fields.

Disposition: native semantic fields and matching inline fields now carry concrete execution/validation/review truth, repository-relative references, explicit unknown metrics and pending publication/CI. No rendered Markdown was hand-edited. All six cards must pass native validation and literal-placeholder inspection. Record-only acknowledgement remains pending; source proof is unchanged and has not been rerun.
