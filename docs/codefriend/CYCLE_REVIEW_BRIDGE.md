# Cycle review continuation

A completed update cycle containing a successful review may continue through the
native Journey and publication owners. Review-less or failed cycles remain
unavailable. Honest partial source coverage follows the review owner's existing
`successful_execution` rule; it is not a claim of complete source analysis.

Hosted owners select and validate the retained nested review and reuse the
original server review directory. The producer finalizes the retained run,
review record and lane results with the observed provider identity before
publishing its cycle. Every hosted Journey access checks the selected review
against that retained run. They do not replace the aggregate result or
invoke a model again.

Installed agents fetch `GET /v1/operations/:gateway_operation/review-evidence`
with the existing paired model credential. The gateway admits only a live,
complete, owned operation at its current candidate. The versioned
`codefriend.cycle_review_evidence.v1` capsule binds operation, request, candidate,
cycle and admission digests and a retention deadline. It carries exact JSON
bytes for `run.json`, `review-record.json` and the four lane input/result pairs.
Provider logs and raw provider responses are excluded. Files are limited to
1 MiB each and the serialized capsule to 4 MiB; excess data is rejected, never
truncated. This endpoint adds no model execution.

The capsule digest is an integrity check, not a signature or an independent
provider attestation. Every file is also validated against the already accepted
nested review and its deterministic lane input contracts. The agent stores the
import under a distinct `imported-cycle` owner with an explicit receipt. It
preserves the website run ID, gateway run ID, original local admission and
gateway admission; it does not modify the original report or forwarding receipt.
Imported evidence is not labelled original local producer output. Importing
retains a separate import cleanup deadline, taking the earlier of the run deadline, the
report deadline and the capsule deadline. This makes ordinary expiry cleanup
remove imported source even when a later request rejects an expired report
before reaching the import owner.

Each continuation and publication boundary performs fresh authenticated gateway
GETs before and after importing/checking evidence, then rechecks local consent,
pairing, cancellation, report and original acquisition. Missing or revoked
remote authority fails closed and scrubs the imported payload. The imported store
has the original gateway admission and cannot extend its deadline. Run payload
cleanup also removes imported data. An offline capsule is not live ownership.

The website model proxy must forward the new GET through its existing paired
model credential policy. Website cycle routing and user controls belong to
#1132. Native component tests do not establish installed-agent, deployed website,
paid-provider or #915 twelve-journey qualification.
