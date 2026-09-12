# #879 review record

Independent exact-head review pending. Source candidate: `b91ace4be5c67e00455df62dd6ea4f3b5afb5787`.

## Preserved early root finding R1 — cumulative metadata memory bound

Original observation: the tree cache could retain 4096 responses of up to 2 MiB each despite a 1 MiB source budget; per-response bounds and deadlines alone were not a practical memory bound.

Disposition: fixed. Production Transport now counts all response bytes against an 8 MiB per-transport capture ceiling before parsing/caching. The focused test `aggregate_metadata_is_bounded_across_individually_valid_responses` executes multiple valid HTTP responses below the individual response ceiling and verifies the aggregate fail-closed diagnostic. It passed in both candidate and installed-binary proof runs. No original finding has been removed or rewritten as a pass.

## Local validation corrections

The first test run exposed inherited nonblocking accepted sockets in the macOS fixture. The fixture explicitly restores blocking sockets; all cases then passed. Clippy found a test assertion using len()>0; corrected to !is_empty(), and focused clippy passed. These are retained execution corrections, not independent approval.
