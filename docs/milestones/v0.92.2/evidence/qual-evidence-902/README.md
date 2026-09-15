# QUAL-EVIDENCE issue 902

This packet is the public, sanitized input and output surface for the v0.92.2
five-row Runtime qualification consumer. Run it from the repository root:

```sh
python3 adl/tools/validate_v0922_runtime_qualification.py \
  --protected-root "$(git rev-parse --git-common-dir)/csdlc-v3/local"
python3 -m unittest adl/tools/test_validate_v0922_runtime_qualification.py
```

The validator binds the exact text, digest, and accepted execution profile of `RUST-01-ac-4`,
`DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2`, and `DRT-C-ac-3` to the accepted
#899, #900, #901, and #852 producer revisions. It checks producer outcomes,
exact artifact bytes, #899's two complete execution inventories, #901's
timeout request/result and observed 150 ms deadline, #852's six raw execution
logs, allowlisted protected archive members, and typed independent-review
receipts. It opens archives in place and never extracts them.

`qualification-manifest.json` intentionally contains repository-relative
logical paths and digests only. The actual retained #900 and #901 archives and
all four typed review receipts remain under Git-local C-SDLC evidence. The
#852 execution archive is retained under issue #902's worktree-local
`.csdlc/evidence/902/retained/` for terminal C-SDLC retention. If any
underlying execution input is unavailable or differs by one byte, the affected
row is not admitted and aggregate completion fails.

The five admitted rows do not rewrite the original 19 findings. The five
cloud-control gaps and two execution-proof gaps remain separate categories,
and historical consumers #522 and #833 remain closed historical records. This
packet grants no release approval.
