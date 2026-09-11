# v0.92.2 Supporting Platform and Publication Tracks

Status: new work remains planned until WP-01 opens the wave. Existing #720, #848, #849, #852, #854, #855, #861, and #862 retain mapped authority; #848 is the bounded repository-split decision row. Closed merged #717/#718 are v0.92.1 predecessor inputs.

These tracks support CodeFriend Beta 1 without being folded into one oversized product issue.

| Track | Concrete result | Acceptance boundary |
|---|---|---|
| PLAT-PROVIDER | Validated editable provider definitions consumed by production | Registration/reload and invalid-definition behavior, no credentials in data files; requires RT-COST and merged #622 |
| RT-PROVIDER (#855) | Dynamic agent lifecycle through registered providers | Real attach, detach and continuity through production Runtime; follows PLAT-PROVIDER |
| PLAT-MLX | One bounded MLX/Apple Metal adapter | Canonical provider definitions consumed; supported-platform and failure proof |
| PLAT-PAIR | One bounded NVIDIA PAIR experiment and retained decision | Runs after PLAT-PROVIDER; reproducible comparison and resource/cost evidence; no production-provider claim |
| PLAT-UTS | One versioned installable UTS package used by Runtime ACC/UTS dispatch | Production tool dispatch and compatibility/rejection proof; schema-only delivery fails |
| PLAT-RUST | One fully extracted or simplified production Rust responsibility | Select the exact responsibility and invariant before creation; focused regressions and recursive source accounting prove the completed change |
| OPS-AWS | Current sanitized SCR, S3, model, and staleness delta from #484 | Agent Logic business account verified; read-only operation; #484 baseline not repeated |
| OPS-GCP | One apply-ready company GCP move-in execution packet | Current-state, Terraform, apply-order, rollback, ownership/billing, and residual sections are parts of the single packet; no cloud mutation or six-resident requalification |
| PUB-MEDIUM | One finished selected Medium article | Complete source-checked prose ready for editorial decision; an outline is insufficient; no publication |
| PUB-CSDLC | One completed named C-SDLC manuscript revision | Freeze and finish the revision checklist with source/citation checks; no submission |
| PLAT-MEMORY | Memory Palace retrieval in the production second-review comparison path | Compatible prior-run retrieval, privacy and deletion enforcement through the real caller |
| SPEC-RETEST | Keep, repair, or retire decision | Current reproducible correctness and performance evidence |
| OBS-S3 | One deployed static Observatory edge sidecar | Apply existing #679 Terraform with `agent-logic-admin` after OBS-LIVE; verify business account, private S3/OAC, CloudFront/ACM/Route53, logging, security headers, invalidation, browser HTTPS, Runtime WSS, cost, and rollback; never gate CF-INTEGRATE or TAIL-01 |
| ARCH-ADR | One reconciled milestone ADR set | Inventory required decisions, draft source-grounded ADRs, retain explicit status/ownership, and verify supersession without implementing or silently accepting decisions |

Each unassigned row becomes a separate bounded issue through WP-01. OCI packaging, ATE, Runtime v4, customer-scale or multi-tenant deployment, OpenRewrite services, and other deferred programs remain outside v0.92.2. `OBS-S3` is only the bounded static sidecar already designed by #679/PR #685.

## Existing admitted tracks

Reuse the existing issues listed in the reconciliation rather than creating replacements. Consume the reviewed merged #717 capability-orientation and #718 canonical-name A2A results from v0.92.1. #720 and the declared release-gating existing lanes converge at TAIL-01; #848 decides split routing and is not itself split implementation.

## Separate existing repairs and qualifications

QUAL-RUNTIME/#852 delivers correlated Runtime dispatch failure events. QUAL-RESIDENT executes resident workload and signed-restore qualification; QUAL-PROVIDER executes real provider failure/recovery; QUAL-INVENTORY measures the before/after validation inventory; QUAL-EVIDENCE validates their criterion-bound results. No aggregate packet can substitute for execution or absorb an unfinished repair.

CSDLC-DECOMPOSE/#862 completes local command-owner decomposition. CSDLC-REMOTE separately completes remote decomposition after local decomposition and CSDLC-MERGE. Existing issue scopes must be reconciled before execution; this plan does not change GitHub bodies or create successors.
