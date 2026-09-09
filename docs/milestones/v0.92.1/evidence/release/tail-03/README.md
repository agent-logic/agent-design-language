# TAIL-03 publication candidate preparation

Issue #519 is in progress under explicit early-start authorization while #518 / PR #753 is open. This packet is provisional and supplies no release approval. No merge, tag, release or issue closure is performed.

`candidate.json` fixes an immutable PR head, the independently reviewed source commit, the source review-record digest, and hashes of every TAIL-02 artifact. The source handoff covers 791 documents. Read source artifacts with `git show <source_head>:<path>`; the live #518 checkout is not an input to validation.

The source PR closes #518. The eventual #519 implementation PR must use `Closes #519`; it must not close #518 or another release-tail issue. No #519 PR exists yet.

Run `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --preparation` for provisional linkage, hashes and redaction checks. `--self-test` checks rejection of altered hashes, wrong issue linkage and an attempted premature final claim. Final lane flags (`--linkage`, `--exact-head`, `--redaction`, `--all`) deliberately fail while the packet is preparation-only.

After #753 merges, refresh the source and immutable review linkage against its actual merged revision. Preserve the historical quality assessment and the current accounting dispositions from #517's completed closeout. Accounting resolution does not itself supply product proof or release approval. Run the final validator and independent review before publication. The final validator requires a merged PR observation, canonical main ancestry and byte-for-byte equality between reviewed source artifacts and the merged candidate. Refresh the live PR observation before final validation; the saved observation is evidence of that read, not a continuous monitor.

Validation classification: deterministic local documentation contracts, small CPU/file/Git reads, issue-level preparation proof. No network, runtime, cloud or release acceptance is claimed. Redaction screening covers this publication packet, not every historical source artifact it references. Source review scope and residual risks remain in the hash-bound #518 review record.
