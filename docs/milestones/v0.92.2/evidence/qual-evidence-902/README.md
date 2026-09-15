# QUAL-EVIDENCE issue 902

This packet is the public, sanitized input and output surface for the v0.92.2
five-row Runtime qualification consumer. Run it from the repository root:

```sh
python3 adl/tools/validate_v0922_runtime_qualification.py \
  --protected-root "$(git rev-parse --git-common-dir)/csdlc-v3/local"
python3 -m unittest adl/tools/test_validate_v0922_runtime_qualification.py
```

The validator binds the exact text and digest of `RUST-01-ac-4`,
`DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2`, and `DRT-C-ac-3` to the accepted
#899, #900, #901, and #852 producer revisions. It checks producer outcomes,
exact artifact bytes, allowlisted protected archive members, and typed
independent-review receipts. It opens archives in place and never extracts
them.

`qualification-manifest.json` intentionally contains repository-relative
logical paths and digests only. The actual retained #900 and #901 archives and
all four typed review receipts remain under Git-local C-SDLC evidence. If those
protected inputs are unavailable or differ by one byte, the affected row is
not admitted and aggregate completion fails.

The five admitted rows do not rewrite the original 19 findings. The five
cloud-control gaps and two execution-proof gaps remain separate categories,
and historical consumers #522 and #833 remain closed historical records. This
packet grants no release approval.
