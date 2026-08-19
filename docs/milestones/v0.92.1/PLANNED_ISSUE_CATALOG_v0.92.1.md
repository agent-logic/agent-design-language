# Planned Issue Catalog — v0.92.1

This is the complete issue-creation plan. WP-01 creates the unnumbered entries after the planning PR merges. This document does not create issues and must not preallocate GitHub numbers.

## Existing issues

| Ref | Purpose | Disposition |
|---|---|---|
| #432 | Remove tracked local-path authority | Opening prerequisite |
| #431 | Publish this planning package and create the later wave | WP-01 conductor |
| #51 | Podcast coordination | Existing execution root |
| #261 | Identity, artwork, rights, metadata, mailbox | Existing podcast child |
| #262 | Hosting, RSS, enclosures, playback | Existing podcast child |
| #263 | Directory-submission runbooks | Existing podcast child |
| #264 | Authorized directory submissions | Existing podcast child |
| #342 | Podcast Studio first ten episodes | Existing podcast child |
| #251 | TLS 1.2 support | Active Observatory prerequisite; may run in parallel with #122 and #345 |
| #122 | Route53/ACM public exposure | Active Observatory prerequisite; may run in parallel with #251 and #345 |
| #84 | Unity Observatory readiness | Active lane; preparation may run in parallel, final proof consumes #251 and #122 |
| #345 | AWS GPU Shepherd hardening | Active distributed Runtime input; may run in parallel with Observatory prerequisites |

## Issues WP-01 will create

| Planned ID | Title | Depends on | Retained predecessor scope |
|---|---|---|---|
| CORP-A | Inventory, provenance, licensing, trademark, assignment, and acceptance | #431/#432 | #153-#155 |
| CORP-B | Corporate account custody, billing, recovery, MFA, and vault controls | CORP-A | #156 |
| CORP-C | Repository, domain, brand, vendor, AWS, Terraform, CI, and runbook transfer | CORP-A, CORP-B | #157-#159 |
| CORP-D | Chain-of-title, diligence, counsel review, exceptions, and corporate acceptance | CORP-A-C | #160 |
| V3-A | Product contract, Rust construction slice, and platform decision | #431/#432 | #161-#163 |
| V3-B | Binary foundation, services, repository context, state, and projections | V3-A | #164-#167 |
| V3-C | Lifecycle kernel, transactions, recovery, and typed adapters | V3-B | #168-#170 |
| V3-D | Local issue/bind/card/doctor commands and PVF planning | V3-C | #171-#173 |
| V3-E | PVF execution, review, publication, GitHub/PR, finish, and cleanup | V3-D | #174-#178 |
| V3-F | Parity, canary migration, authority cutover, observation, and retirement decision | V3-E | #179/#180 |
| DRT-A | Qualification contract, ACIP authority, and replay conformance | #431/#432 | #181/#182 |
| DRT-B | Multi-agent UTS work and hybrid Spot continuity | DRT-A | #183/#184 |
| DRT-C | Identity/provider failure, Observatory evidence, soak, cleanup, and synthesis | DRT-B | #185-#187 |
| HOT-01 | Validated atomic Axum configuration hot reload | #431/#432 | New v0.92.1 work |
| OBS-A | Observatory information architecture, interaction, and accessibility redesign | #431/#432 | Existing Observatory evidence |
| OBS-B | Authentic Runtime projections, implementation, and state/failure proof | OBS-A and stable Runtime authority | Existing Observatory evidence |
| INT-01 | Cross-lane convergence and release-tail admission | All six lane roots | #188-#190 |
| TAIL-01 | Quality gate | INT-01 | Standard release tail |
| TAIL-02 | Docs and release-truth pass | TAIL-01 | Standard release tail |
| TAIL-03 | Publication finalization | TAIL-02 | Standard release tail |
| TAIL-04 | Internal milestone review | TAIL-03 | Standard release tail |
| TAIL-05 | External or third-party review | TAIL-04 | Standard release tail |
| TAIL-06 | Accepted-findings remediation or explicit deferral | TAIL-05 | Standard release tail |
| TAIL-07 | v0.92.2 CodeFriend Beta 1 planning handoff | TAIL-06 | #190 intent |
| TAIL-08 | Next-milestone closeout planning | TAIL-07 | Standard release tail |
| TAIL-09 | Next-milestone planning review | TAIL-08 | Standard release tail |
| TAIL-10 | Final validation, notes, tag, cleanup, and release ceremony | TAIL-09 | #189 intent |

## Retired inputs, not issues to recreate verbatim

- #149-#152 were premature umbrellas.
- #153-#190 are retained requirement packets mapped above; do not reopen them.
- #433-#438 were prematurely created placeholders and are closed; do not use them as execution authority.
- #439 is a redundant successor-handoff placeholder and remains closed.

## Existing issues promoted into v0.92.1

#251, #122, #84, and #345 are part of the active milestone denominator. WP-01 reconciles their labels and dependency links without creating replacements. #251, #122, and #345 may execute in parallel; #84 preparation may overlap them, but its final proving lane waits for #251 and #122.
