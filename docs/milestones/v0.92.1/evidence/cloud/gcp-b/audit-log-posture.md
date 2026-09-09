# GCP-B audit and log posture proof

Issue: #772

Candidate with open proof gap: `c24f8fa65ce445b03ce6cd69007307291d78b60c`

Owned row: `GCP-B:GCP-B-ac-1` / `State is versioned private recoverable and auditable`

## Assertions

`GCP-B-ac-1` is complete only when the accepted GCP-B project can be read back as private, versioned, recoverable, and auditable. Corrective issue #740 proved private posture, versioning, and Terraform backend recovery. This #772 packet adds the missing audit/log side of that row.

The audit/log posture proof uses temporary public identifiers in the retained packet. The accepted project is represented as `gcp-b-project`, and the bootstrap provider identity is represented as `gcp-b-bootstrap-service-account`; exact binding is retained through SHA-256 digests rather than raw provider identifiers.

The audit/log posture proof requires:

- intended project alias: `gcp-b-project`;
- intended bootstrap identity alias: `gcp-b-bootstrap-service-account`;
- enabled services for Logging, Cloud Resource Manager, IAM, and Service Usage;
- IAM audit configuration readback for project-level audit settings;
- Cloud Logging bucket/readback posture including the required audit bucket;
- logging sinks/settings readback when available through the read-only API;
- representative Cloud Audit log readback from the accepted project within the proof window;
- retained evidence that omits raw log payloads, credential material, local key paths, token values, and unredacted principals.

## Proof route

The issue-owned runner is `.csdlc/prepared/issues/772/run-gcp-b-audit-log-posture.sh`.

The issue-owned validator is `.csdlc/prepared/issues/772/validate-gcp-b-audit-log-posture.sh`.

The retained redacted packet is `.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json`.

The retained auth-readiness receipt is `.csdlc/evidence/772/gcp-b-audit-log-auth-readiness.redacted.json`. It is diagnostic evidence only and does not satisfy `GCP-B-ac-1` by itself.

## Live proof result

After operator auth refresh, the issue-owned runner generated `.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json` for candidate `c24f8fa65ce445b03ce6cd69007307291d78b60c` using proof runner head `4d9dd072ecf4c46795cb28a9530b7ac66d6b4348`.

The retained packet records a read-only GCP posture check: the project readback is active, the bootstrap provider identity exists and is not disabled, the active operator digest matches the approved actor digest, IAM audit configuration is readable, required Logging/Cloud Resource Manager/IAM/Service Usage APIs are present, `_Required` and `_Default` logging buckets are readable, sinks are readable, and representative Cloud Audit log entries were observed within the 30-day proof window.

The proof artifact SHA-256 is `0f15781020d340e87b99e2fe505ac87c20816755ad805817f712493836ce95ce`; its Git blob is `66201a6a476b379a988975989866446a61d7a01a`.

The runner requires the raw project and service-account identifiers as process-local environment inputs: `GCP_B_PROJECT_ID` and `GCP_B_BOOTSTRAP_SERVICE_ACCOUNT`. Those values are used only for live readback and digest computation. They are not written to the retained packet.

For short-lived credential execution, pass the runner's environment-only credential input plus the approved actor digest input. In that mode the runner calls the Cloud Resource Manager, IAM, Service Usage, and Cloud Logging REST APIs directly with an authorization header sourced from process environment. The short-lived credential is never placed in argv, copied into a file, or written to retained evidence.

When the short-lived credential input is not supplied, the runner uses a disposable issue-local Cloud SDK config under the repository git-common directory, performs read-only `gcloud` project, service-account, service, logging bucket, sink, settings, and audit-log readback commands, writes only a sanitized summary packet, validates that packet, and removes the disposable Cloud SDK config before exit. When the normal user Cloud SDK cache cannot refresh non-interactively, the runner may receive an approved bootstrap service-account key file through its key-file environment input; that path and key material are consumed only by `gcloud auth activate-service-account` inside the disposable config and are not retained in the evidence packet.

The validator binds the public aliases to expected SHA-256 digests for the accepted GCP-B project and bootstrap identity. A packet generated for a different project or provider identity cannot pass by merely reusing the aliases. The retained packet also binds the active readback actor to an approved actor digest. By default the runner treats the active authenticated actor as the approved actor for the one command invocation; callers that need an externally predeclared actor can pass `GCP_B_APPROVED_ACTOR_SHA256`.

No cloud mutation is part of this proof route.

## Redaction boundary

The retained packet stores provider/environment identity, candidate binding, service/configuration booleans, log bucket summaries, sink destination classes, and representative audit-log classes. It intentionally does not retain raw log entries, `protoPayload.request`, `protoPayload.response`, unredacted principal email values from log payloads, token material, key contents, local credential paths, or Cloud SDK credential databases.

Issue #769 remains the broader publication redaction gate. This #772 packet includes equivalent issue-local redaction checks so the `GCP-B-ac-1` proof can be reviewed without adopting unsafe retained evidence.
