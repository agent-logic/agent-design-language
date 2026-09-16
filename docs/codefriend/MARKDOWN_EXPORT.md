# CodeFriend Markdown Export

`adl codefriend export markdown` creates a deterministic local Markdown report
from an explicitly approved CodeFriend publication input. It does not publish
the report remotely and does not claim HTML or PDF rendering.

## Required inputs

The command requires:

- the completed `review-record.json` used by the publication decision;
- the exact `publication.json` and its current approval store;
- one approved artifact root containing canonical synthesis, remediation-plan,
  and test-plan bundles;
- an existing local destination root; and
- a new output path equal to the approved publication target beneath that
  destination root.

Every selected JSON file and every companion manifest/source file read from its
bundle must be present with its current digest in the publication artifact
manifest. The approved publication must bind the Markdown renderer as `v1`.

## Command

```sh
adl codefriend export markdown \
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

The three semantic-input flags are paths relative to `--artifact-root`. The
output directory must not already exist. On success, stdout contains one JSON
result and the new target contains:

- `report.md` — the readable canonical report; and
- `manifest.json` — source, approval, renderer, finding, claim, plan, and
  output-digest bindings.

## Fail-closed behavior

Export stops before creating the target when the current decision is absent,
withheld, or invalidated; an approved artifact changed; source identities or
finding partitions disagree; required provenance is missing; the renderer,
destination, or target differs from the approved identity; redaction fails; or
the target already exists.

Repository evidence is rendered as non-clickable code-span citations. Other
untrusted text is Markdown-escaped, so a repository-supplied link or embedded
HTML fragment is not emitted as active content. The complete rendered report
and manifest are re-read and digest-checked before success is returned.

## Validation

The required local lane is:

```sh
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md
```

Its classification is tracked in
`adl/tests/fixtures/codefriend/markdown/PVF.json`. Provider execution, network,
browser inspection, HTML/PDF parity, and external publication are outside this
lane.
