# CodeFriend HTML Export

`adl codefriend export html` creates a deterministic, script-free local HTML
report from an explicitly approved CodeFriend publication input. It does not
publish remotely and does not claim PDF rendering.

The input contract and flags match the Markdown exporter. The approved
publication must bind renderer `html` at version `v1`, and the output path must
be the approved target beneath an existing destination root.

```sh
adl codefriend export html \
  --review-record review-record.json \
  --publication publication.json \
  --approval-store approval-store \
  --artifact-root approved-inputs \
  --synthesis synthesis/synthesis.json \
  --remediation-plan remediation/remediation-plan.json \
  --test-plan tests/test-plan.json \
  --destination-root local-output \
  --out local-output/approved-report
```

On success the new target contains `report.html` and `manifest.json`. The
report preserves the approved claims, findings, citations, uncertainty,
disagreements, remediation actions, and test plans. Contents, finding, and
evidence links are same-document anchors. It contains no scripts, remote
assets, remote links, forms, or automatic actions.

All repository-controlled text is HTML-escaped. Export stops before committing
the target when approval is absent or withheld, provenance changed, an
approved artifact was modified, renderer or target identity drifted, redaction
fails, or the output target already exists. The committed report and manifest
are re-read and digest-checked before success is returned.

The deterministic local proof lane is:

```sh
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_html
```

Its PVF classification is recorded in
`adl/tests/fixtures/codefriend/html/PVF.json`. Browser observation is retained
separately from the deterministic contract lane.
