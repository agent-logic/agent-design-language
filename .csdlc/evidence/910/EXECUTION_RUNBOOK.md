# #910 reviewed execution sequence

This is the reviewed pre-execution procedure, retained for traceability. Its initial apply and four uploads have now executed under explicit operator authorization; see `DEPLOYMENT_PLAN.md` and the execution receipts. Browser public-read/WSS and unauthenticated write-gating proof now pass; see `browser-proof.json` and the operator-local permission boundary in `DEPLOYMENT_PLAN.md`. The final exact-invalidation receipt is retained in `upload-invalidation.json` and independently verified. The commands below are not instructions to repeat completed mutations. Do not run this document as a script. Commands assume execution from the bound issue worktree. `OBS_TF` is the stable private Git-common `csdlc-v3/private/910/terraform` directory already created, `OBS_ASSETS` is `.adl/runs/910/assets`, and `OBS_PRIVATE` is its stable private parent; resolve these to absolute paths before use. No shell history or output may contain credentials. Reconfirm business STS in memory against approved baseline, target DNS/global-name absence and Origin acceptance immediately before approved execution; stop if they changed.

## Apply exact saved plan

Verify deployment.tfplan SHA256 equals `0aa4a1d558cafe4de27eee3acc7e1c00249b2ed2219e9f109410b00037af9fec`, each configuration/lock hash equals custody-manifest, and each asset hash equals upload-manifest. Do not silently regenerate a changed plan.

```sh
terraform -chdir="$OBS_TF" init -backend=false -input=false -lockfile=readonly
OBS_APPLY_STATUS=0
terraform -chdir="$OBS_TF" apply -input=false -no-color deployment.tfplan > "$OBS_PRIVATE/apply.log" 2>&1 || OBS_APPLY_STATUS=$?
```

After any apply return, including failure, preserve existing terraform.tfstate and terraform.tfstate.backup into a new private timestamped backup directory, mode 0700/0600, with SHA256 manifest. Do not continue on failure. **Perform and verify the state backup before evaluating the next block**, including a failed or partial apply. Output capture is conditional on success:

```sh
if [ "$OBS_APPLY_STATUS" -ne 0 ]; then
  exit "$OBS_APPLY_STATUS"
fi
terraform -chdir="$OBS_TF" output -json > "$OBS_PRIVATE/outputs.json"
```

Copy outputs only after success. Derive `OBS_SITE`, `OBS_LOGS` and `OBS_CF` from the corresponding site_bucket/access_log_bucket/distribution_id outputs; confirm exact FQDN output and expected bucket prefixes. Derive `OBS_ZONE` from the previously verified exact public csm.agent-logic.ai zone. Never substitute existing Wuji resources.

## Upload four exact assets

Use the read-only hash check first. Execute each put separately and stop on any nonzero status, missing or empty VersionId receipt, or object verification failure. Before proceeding to the next put, read back that exact object version privately and compare its SHA256 to the upload manifest. All three prerequisite asset/config uploads must have successful receipts and matching readback hashes before index publication. Do not run invalidation unless all four puts and their hash/version checks succeeded. Each receipt is retained privately for rollback. Preserve initial absence versus prior version truth. Put index last.

```sh
aws --profile agent-logic-admin s3api put-object --bucket "$OBS_SITE" --key app.327288d52ff025b5.js --body "$OBS_ASSETS/app.327288d52ff025b5.js" --content-type application/javascript --cache-control 'public,max-age=31536000,immutable' > "$OBS_PRIVATE/put-app.json" || exit $?
aws --profile agent-logic-admin s3api put-object --bucket "$OBS_SITE" --key styles.fb5a4cb483c419b7.css --body "$OBS_ASSETS/styles.fb5a4cb483c419b7.css" --content-type text/css --cache-control 'public,max-age=31536000,immutable' > "$OBS_PRIVATE/put-css.json" || exit $?
aws --profile agent-logic-admin s3api put-object --bucket "$OBS_SITE" --key runtime-v3.config.json --body "$OBS_ASSETS/runtime-v3.config.json" --content-type application/json --cache-control 'no-cache,no-store,must-revalidate' > "$OBS_PRIVATE/put-config.json" || exit $?
aws --profile agent-logic-admin s3api put-object --bucket "$OBS_SITE" --key index.html --body "$OBS_ASSETS/index.html" --content-type text/html --cache-control 'no-cache,no-store,must-revalidate' > "$OBS_PRIVATE/put-index.json" || exit $?
aws --profile agent-logic-admin cloudfront create-invalidation --distribution-id "$OBS_CF" --paths / /index.html /runtime-v3.config.json > "$OBS_PRIVATE/invalidation.json"
```

Wait for the exact invalidation to complete before content verification. Derive its ID from the private creation receipt, then retain a safe completion projection:

```sh
OBS_INVALIDATION=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["Invalidation"]["Id"])' "$OBS_PRIVATE/invalidation.json")
aws --profile agent-logic-admin cloudfront wait invalidation-completed --distribution-id "$OBS_CF" --id "$OBS_INVALIDATION"
aws --profile agent-logic-admin cloudfront get-invalidation --distribution-id "$OBS_CF" --id "$OBS_INVALIDATION" --query 'Invalidation.{status:Status,created_at:CreateTime,paths:InvalidationBatch.Paths}' --output json > "$OBS_PRIVATE/invalidation-completed.json"
```

Require `status == Completed`; a waiter timeout is pending evidence, never completion. Do not repeat create-invalidation merely because the waiter times out.

## Readback and live acceptance

```sh
AWS_PROFILE=agent-logic-admin bash infra/aws/observatory/readback.sh --execute "$OBS_CF" "$OBS_SITE" "$OBS_LOGS" "$OBS_ZONE" observatory.csm.agent-logic.ai > "$OBS_PRIVATE/posture-readback.json-stream"
```

The script emits a stream of JSON values, not one JSON document. Normalize fixed allowed fields into public evidence. Supplement privately with get-distribution-config, get-origin-access-control and get-response-headers-policy to verify OAC binding/signing, exact CSP/origins and security headers; publish only fixed safe projections. Check S3 get-bucket-policy-status IsPublic false and anonymous direct-object access denied. Read every published HTTPS asset and compare SHA256/cache/content headers to upload-manifest. Verify ACM ISSUED, CloudFront Deployed, exact A/AAAA and 90-day logs. Browser acceptance must exercise actual target HTTPS, health/readiness/live Observatory feed and WSS using existing operator auth without retaining tokens/screenshots with secrets. No stubbed network counts as deployed proof.

## Rollback dry-run

For first deployment no prior version exists: report that explicitly and enumerate the four newly created object version receipts as future recovery anchors. A non-mutating simulation maps known index/config version IDs back to their verified object hashes and planned copy-object source `<site>/<key>?versionId=<recorded-version>`, preserving content type/cache metadata, then the same three-path invalidation. Do not execute copy or invalidation for a dry-run. A future actual content rollback requires exact prior-version selection and approval; restore prior index/config as new versions, keep immutable assets. Infrastructure recovery requires a separately reviewed saved Terraform plan using preserved authoritative state, never an automatic destroy. Retain all partial state if any step fails.
