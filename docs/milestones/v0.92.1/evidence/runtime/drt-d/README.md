# DRT-D GCP portability qualification

Issue #509 proves that the distributed Runtime qualification can run on GCP with
the same six resident agents and Ollama model identities used by the AWS DRT
line.

The executable path is:

```bash
ADL_ISSUE509_ARTIFACT_BUCKET="<approved-gcs-bucket>" \
  adl/tools/run_issue509_gcp_drt_d_qualification.sh prepare-artifacts --execute

ADL_ISSUE509_ARTIFACT_BUCKET="<approved-gcs-bucket>" \
ADL_ISSUE509_ARTIFACT_MANIFEST_OBJECT="<manifest-object-from-prepare-output>" \
ADL_ISSUE509_ARTIFACT_MANIFEST_SHA256="<manifest-sha-from-prepare-output>" \
adl/tools/run_issue509_gcp_drt_d_qualification.sh preflight

ADL_ISSUE509_ARTIFACT_BUCKET="<approved-gcs-bucket>" \
ADL_ISSUE509_ARTIFACT_MANIFEST_OBJECT="<manifest-object-from-prepare-output>" \
ADL_ISSUE509_ARTIFACT_MANIFEST_SHA256="<manifest-sha-from-prepare-output>" \
adl/tools/run_issue509_gcp_drt_d_qualification.sh run --execute
```

The live run creates two disposable private GCP VMs:

- Runtime/CSM node: downloads the prepared Runtime bundle from GCS object
  storage, verifies SHA-256, and runs the six-resident UTS workload against the
  private Ollama endpoint.
- Ollama/GPU node: downloads the pinned Ollama runtime/model-store bundle from
  GCS object storage, verifies SHA-256, and serves `llama3.1:8b`, `qwen3:8b`,
  and `phi4-mini:latest` over private GCP networking only.

The runner destroys both VMs before writing the retained qualification receipt
and records cleanup as part of the proof. It requires a noninteractive
operator-approved GCP credential; the credential path is intentionally not
stored in the retained receipt.

Because the company project denies external VM IP addresses, the Terraform root
also creates a disposable regional Cloud Router/NAT for outbound bootstrap when
`create_cloud_nat=true`. The NAT and router share the issue/run labels and are
destroyed by the same Terraform cleanup path. The `cleanup --run-id` path also
removes any matching per-run Router/NAT names, and retained cleanup proof
requires absent readback for instances, Router, and NAT.

The VM startup scripts must not call `ollama pull`, `git clone`, `rustup`, or
`cargo build`, and they must not perform normal-start package installation.
Runtime/model acquisition happens before launch by placing the portable bundle
in GCS and passing:

- `artifact_bucket`
- `artifact_manifest_object`
- `artifact_manifest_sha256`

The manifest uses the same object-store posture as the AWS warm-start work:
immutable object paths plus SHA-256 identity. It must include one
`runtime_bundle`, one `ollama_runtime`, and the exact Ollama model store for the
three configured resident model identities. Startup verifies every archive
before extraction and fails closed if the prepared image lacks its expected
launch tools.

Runtime binaries are cached the same way as model artifacts. The relay builder
is copied to the persistent relay VM and creates the Linux `adl`/`csm` runtime
bundle there once, uploads it to GCS, and the qualification manifest points
disposable VMs at that object. The source-controlled builder is
`adl/tools/build_issue509_runtime_bundle_on_relay.sh`; it defaults to one Cargo
job so small relay instances do not OOM while compiling large SDK crates. Set
`ADL_RELAY_CARGO_BUILD_JOBS` only when the relay has enough memory.

The artifact bucket is private. Credential-free HTTPS probes may return `403`
even for a valid object; live object/readiness checks require an authenticated
GCP control-plane profile, while retained receipts must not store credential
paths or contents.
