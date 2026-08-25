# Planned Issue Catalog — v0.92.1

This is the complete issue-creation plan. After the planning PR merges, the milestone operator creates the number-free WP-01 opening conductor. That new conductor creates the remaining unnumbered entries. This document does not create issues and must not preallocate GitHub numbers.

## Existing issues

| Ref | Purpose | Disposition |
|---|---|---|
| #432 | Remove tracked local-path authority | Opening prerequisite |
| #431 | Historical authoring of this planning package | Closed planning provenance; never future conductor authority |
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

## Milestone-opening conductor

| Planned ID | Title | Created by | Depends on |
|---|---|---|---|
| WP-01 | Open v0.92.1 and create the execution wave | Milestone operator after the reviewed planning package merges | #432 and the merged planning package |

WP-01 is deliberately number-free until milestone opening. It is not #431, and #431's closed state cannot authorize future issue creation.

## Issues WP-01 will create

| Planned ID | Title | Depends on | Retained predecessor scope |
|---|---|---|---|
| CORP-A | Inventory, provenance, licensing, trademark, assignment, and acceptance | WP-01/#432 | #153-#155 |
| CORP-B | Corporate account custody, billing, recovery, MFA, and vault controls | CORP-A | #156 |
| CORP-C | Repository, domain, brand, vendor, AWS, Terraform, CI, and runbook transfer | CORP-A, CORP-B | #157-#159 |
| CORP-D | Chain-of-title, diligence, counsel review, exceptions, and corporate acceptance | CORP-A-C | #160 |
| V3-A | Product contract, Rust construction slice, and platform decision | WP-01/#432 | #161-#163 |
| V3-B | Binary foundation, services, repository context, state, and projections | V3-A | #164-#167 |
| V3-C | Lifecycle kernel, transactions, recovery, and typed adapters | V3-B | #168-#170 |
| V3-D | Local issue/bind/card/doctor commands and PVF planning | V3-C | #171-#173 |
| V3-E | PVF execution, review, publication, GitHub/PR, finish, and cleanup | V3-D | #174-#178 |
| V3-F | Parity, canary migration, authority cutover, observation, and retirement decision | V3-E | #179/#180 |
| DRT-A | Qualification contract, ACIP authority, and replay conformance | WP-01/#432 | #181/#182 |
| DRT-B | Multi-agent UTS work and hybrid Spot continuity | DRT-A | #183/#184 |
| DRT-C | Identity/provider failure, Observatory evidence, soak, cleanup, and synthesis | DRT-B | #185-#187 |
| DRT-D | GCP six-resident portability qualification | DRT-C | New sidecar; does not execute #269 |
| HOT-01 | Validated atomic Axum configuration hot reload | WP-01/#432 | New v0.92.1 work |
| OBS-A | Observatory information architecture, interaction, and accessibility redesign | WP-01/#432 | Existing Observatory evidence |
| OBS-B | Authentic Runtime projections, implementation, and state/failure proof | OBS-A and stable Runtime authority | Existing Observatory evidence |
| DEC-01 | Separate Runtime v2/v3 authority and source ownership | WP-01/#432 | New decoupling work; Runtime v4 excluded |
| PROV-A | Shared provider inference-profile contract and Ollama materialization | WP-01/#432 | #457 historical provenance only |
| PROV-B | Local-model shadow execution and comparison evidence | PROV-A | New non-authoritative comparison work |
| INT-01 | Cross-lane convergence and release-tail admission | Every root named in the issue wave | #188 convergence intent |
| TAIL-01 | Quality gate | INT-01 | #188 quality-admission intent |
| TAIL-02 | Docs and release-truth pass | TAIL-01 | Standard release tail |
| TAIL-03 | Publication finalization | TAIL-02 | Standard release tail |
| TAIL-04 | Internal milestone review | TAIL-03 | Standard release tail |
| TAIL-05 | External or third-party review | TAIL-04 | Standard release tail |
| TAIL-06 | Accepted-findings remediation or explicit deferral | TAIL-05 | Standard release tail |
| TAIL-07 | v0.92.2 CodeFriend Beta 1 planning handoff | TAIL-06 | #190 successor-planning intent |
| TAIL-08 | Next-milestone closeout planning | TAIL-07 | Standard release tail |
| TAIL-09 | Next-milestone planning review | TAIL-08 | Standard release tail |
| TAIL-10 | Final validation, notes, tag, cleanup, and release ceremony | TAIL-09 | #189 ceremony intent |

## Retired inputs, not issues to recreate verbatim

- #149-#152 were premature umbrellas.
- #153-#190 are retained requirement packets mapped above; do not reopen them.
- #433-#438 were prematurely created placeholders and are closed; do not use them as execution authority.
- #439 is a redundant successor-handoff placeholder and remains closed.
- #457 is retained as historical provenance for provider-profile planning; it is not reopened and is not an execution dependency.

## Existing issues promoted into v0.92.1

#251, #122, #84, and #345 are part of the active milestone denominator. WP-01 reconciles their labels and dependency links without creating replacements. #251, #122, and #345 may execute in parallel; #84 preparation may overlap them, but its final proving lane waits for #251 and #122.

## Integration provenance

#188 supports INT-01 convergence and TAIL-01 quality admission. #190 supports only the TAIL-07 v0.92.2 handoff. #189 supports only TAIL-10 final ceremony. These closed packets are historical requirement inputs, not active issues and not interchangeable dependencies.
