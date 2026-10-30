# Reviewer Output Provenance

This document describes the retained historical `card_review_output.v1` validator for deterministic review output artifacts. It does not create current C-SDLC review authority.

## Purpose

WP-08 needs a concrete evidence/provenance-linked output surface in the repo.

This slice focuses on deterministic review outputs because they already:

- carry explicit evidence pointers
- use constrained enums and ordering rules
- represent a serious claim about what was reviewed and why

## Verification Entry Point

Select the read-only companion from an authenticated C-SDLC package, then run:

```bash
export CSDLC_HISTORICAL_REVIEW_PROVENANCE_BIN="$CSDLC_PACKAGE_ROOT/resources/tools/historical-review-provenance/bin/csdlc-historical-review-provenance"
bash adl/tools/verify_review_output_provenance.sh \
  --review docs/tooling/examples/reviewer-provenance/good_review_output_661.yaml
```

`CSDLC_PACKAGE_ROOT` must name the delivered package for this platform. The wrapper requires the explicit executable and never builds ADL or falls back to a retired command. `ADL_TOOLING_MANIFEST_ROOT`, when supplied, selects the historical input repository root; it does not require a Cargo manifest.

## What It Verifies

- `review_target.input_card_path` and `review_target.output_card_path` are repo-relative and exist
- finding `evidence` pointers use supported pointer classes
- evidence pointers are ordered canonically
- path/artifact pointers are repo-relative
- validation command entries do not leak absolute host paths
- `absolute_host_paths_present: false` is not contradicted by the artifact text

## Deterministic Fixture

Use:

```bash
bash adl/tools/test_review_output_provenance.sh
```

The fixture compares:

- a valid provenance-linked review output artifact
- an intentionally bad artifact that should be refused

## Scope Notes

This does not solve the full long-term provenance system for ADL.

It is the first bounded verification path proving that at least one evidence-linked output surface can be checked deterministically inside the repo.
