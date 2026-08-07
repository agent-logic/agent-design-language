# WP-06 Portable Remote Validation Runbook

## Boundary

Use one validated `adl.remote_validation.request.v1` request for local, Nessus,
or AWS execution. The selected adapter must preserve the request revision and
command-profile digest in its result. Provider speed, cache state, and
provisioning success are operational observations, not validation proof.

Use only the Agent Logic business AWS profile for an authorized AWS lane.
Verify the profile before use and retain only a one-way account digest. Never
export credentials into a request, result, log, or artifact.

## Local Qualification

1. Validate the request with `adl-remote-validation validate-request`.
2. Run it with `adl-remote-validation run-local` from the repository root.
3. Validate the result against the same request.
4. Confirm the exact revision, command-profile digest, artifact digest,
   redaction status, and cleanup status before accepting the receipt.

## Remote Qualification

1. Obtain explicit operator approval for any paid or provider-backed lane.
2. Reverify the approved provider identity and requested revision.
3. Pass the validated request to exactly one adapter.
4. Preserve stdout as machine output and stderr as `adl_event` diagnostics.
5. Reject stale revisions, malformed results, path leakage, incomplete cleanup,
   and ambiguous adapter selection.

Linux requires a native live receipt. Windows may use a deterministic fixture
only when it is explicitly marked non-native. A blocked platform row remains a
blocker and must not be converted into a pass.

## Fallback

Local fallback is allowed only for provider unavailability, authentication, or
capacity failures and only when the local command-profile digest exactly
matches the remote request. Local fallback never counts as remote proof.

## Rollback

Disable the Nessus and AWS adapters, cancel remote work, and clean every
temporary provider resource. Preserve request/result receipts and any cleanup
failure. Restore the unchanged local command profile as the sole path, then
rerun the local contract and negative adapter tests before re-enabling remote
execution.
