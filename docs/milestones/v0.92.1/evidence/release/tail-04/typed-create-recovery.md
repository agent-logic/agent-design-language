# Typed issue-creation recovery

The first typed creation attempt for `R520-001` persisted operation intent
`8fb0d2c47fe73be0ea198166ced8c3fcb4f95eb62532f212e424a96270de6cad`
but did not receive the required GitHub token-file mapping. The operational
mutation and authenticated reconciliation therefore did not complete.

After mapping the repository-approved token file, the same native route
performed authenticated marker readback and returned
`github_mutation_not_reconciled`. A separate read-only title and marker query
repeated at `2026-09-09T04:17:28Z` returned zero exact-title matches and zero
operation-marker matches. These observations confirm that no matching remote
issue existed at readback time and support exactly one typed retry.

The single retry changes the operation digest by adding an explicit recovery
note to the reviewed issue body. All later creates must provide the approved
token-file mapping on their first invocation. No raw GitHub mutation is
authorized or used.

The missing native transition from an authenticated, confirmed-absent intent
to a safe retry is a C-SDLC tooling finding and must be routed for repair.
