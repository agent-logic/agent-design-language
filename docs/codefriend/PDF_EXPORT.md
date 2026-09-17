# CodeFriend PDF Export

`adl codefriend export pdf` creates a local PDF from the same governed review,
synthesis, remediation, test-plan, and approval contracts used by the Markdown
exporter. It does not publish the report remotely.

## Required inputs

The command requires the completed review record, exact publication manifest,
current approval store, approved semantic artifact root, existing local
destination root, a fresh target directory, and a Unicode TrueType font file.
The publication contract must bind renderer `pdf` to
`v1-printpdf-0.12.8`. The manifest records the renderer/layout identity and the
font digest, so font rotation is explicit and reviewable.

```sh
adl codefriend export pdf \
  --review-record review-record.json \
  --publication publication.json \
  --approval-store approval-store \
  --artifact-root approved-inputs \
  --synthesis synthesis/synthesis.json \
  --remediation-plan remediation/remediation-plan.json \
  --test-plan tests/test-plan.json \
  --destination-root local-output \
  --out local-output/approved-report \
  --font /path/to/unicode-font.ttf
```

The semantic-input flags are relative to `--artifact-root`; the font is a
local renderer input rather than review evidence. The selected font must cover
every character in the report. On success the fresh target contains
`report.pdf` and `manifest.json`.

## Safety and layout

The Rust renderer validates current approval, exact source and target identity,
approved artifact digests, finding/action/test-plan parity, and redaction before
rendering. It treats the governed report as inert text, does not fetch or embed
external resources, wraps long unbroken tokens, paginates at fixed bounds, and
fails without a success target on invalid fonts, missing glyphs, renderer
warnings, oversized output, or stale/tampered input.

## Qualification

Run the focused installed lane:

```sh
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf
```

Release qualification additionally extracts the actual PDF with `pdftotext`,
renders every retained page with `pdftoppm`, and visually inspects those page
images for clipping, overlap, missing content, citation damage, and unreadable
Unicode. The lane classification is
`adl/tests/fixtures/codefriend/pdf/PVF.json`.
