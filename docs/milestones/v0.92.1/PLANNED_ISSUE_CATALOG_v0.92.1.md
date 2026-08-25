# Planned Issue Catalog — v0.92.1

This is the complete issue-creation plan. The planning merge only makes the package eligible to open. When the operator declares v0.92.1 ready, the operator creates the number-free WP-01 opening conductor; WP-01 then creates the remaining unnumbered entries. This document does not create issues and must not preallocate GitHub numbers.

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
| WP-01 | Open v0.92.1 and create the execution wave | Milestone operator, when the milestone is declared ready after the reviewed planning package merges | #432 and the merged planning package |

WP-01 is deliberately number-free until milestone opening. It is not #431, and #431's closed state cannot authorize future issue creation.

## Issues WP-01 will create

| Planned ID | Title | Depends on | Retained predecessor scope |
|---|---|---|---|
| CORP-A | Critical-asset schedule | WP-01/#432 | #153-#155 |
| CORP-B | Corporate account custody register | CORP-A | #156 |
| AWS-A | AWS resource ownership inventory | WP-01, CORP-A, CORP-B | Promoted AWS move-in phase 0 |
| AWS-B | AWS access and billing baseline | AWS-A | Promoted AWS move-in phase 1 |
| AWS-C | AWS Terraform bootstrap | AWS-B | Promoted AWS move-in phase 2 |
| AWS-D | AWS audit and security baseline | AWS-C | Promoted AWS move-in phase 3 |
| AWS-E | AWS resource adoption register | AWS-D | Promoted AWS move-in phase 4 |
| AWS-F | AWS Runtime platform modules | AWS-E, #122 | Promoted AWS move-in phase 5 |
| GCP-A | GCP hierarchy and cost decision | WP-01, CORP-A, CORP-B | Promoted GCP move-in phase 0 |
| GCP-B | GCP Terraform bootstrap | GCP-A | Promoted GCP move-in phase 1 |
| GCP-C | GCP organization and billing baseline | GCP-B | Promoted GCP move-in phase 2 |
| GCP-D | GCP private platform foundation | GCP-C | Promoted GCP move-in phase 3 |
| GCP-E | GCP GPU readiness smoke test | GCP-D | Promoted GCP move-in phase 4 |
| XCL-01 | Cross-cloud Runtime Terraform conversion | AWS-E, GCP-D | Exact #194/#268 CloudFormation denominator |
| AWS-G | AWS CloudFormation retirement decision | AWS-F, XCL-01 | Promoted AWS move-in phase 6; no silent deletion |
| CORP-C | Corporate operational-control transfer | CORP-A, CORP-B, AWS-G, GCP-D | #157-#159 |
| CORP-D | Corporate diligence acceptance | CORP-A-C | #160 |
| RUST-01 | Resilience owner-boundary refactoring | WP-01/#432 | Bounded behavior-preserving Rust slice; no LoC quota |
| V3-A | C-SDLC v3 contract and construction decision | WP-01/#432 | #161-#163 |
| V3-B | C-SDLC v3 foundation | V3-A | #164-#167 |
| V3-C | C-SDLC v3 lifecycle kernel | V3-B | #168-#170 |
| V3-D | C-SDLC v3 local preparation workflow | V3-C | #171-#173 |
| V3-E | C-SDLC v3 remote delivery workflow | V3-D | #174-#178 |
| V3-F | C-SDLC v3 authority-transition decision | V3-E | #179/#180 |
| DRT-A | Distributed qualification contract | WP-01/#432 | #181/#182 |
| DRT-B | Six-resident UTS qualification | DRT-A, #345 | #183/#184 |
| DRT-C | Final distributed Runtime qualification | DRT-B | #185-#187 |
| DRT-D | GCP portability qualification | DRT-C, GCP-E, XCL-01 | New sidecar; does not execute #269 |
| HOT-01 | Axum configuration hot reload | WP-01/#432 | New v0.92.1 work |
| OBS-A | Observatory experience design | WP-01/#432 | Existing Observatory evidence |
| OBS-B | Observatory redesign implementation | OBS-A and stable Runtime authority | Existing Observatory evidence |
| DEC-01 | Runtime v2/v3 authority separation | WP-01/#432 | New decoupling work; Runtime v4 excluded |
| PROV-A | Shared provider inference profiles | WP-01/#432 | #457 historical provenance only |
| PROV-B | Local-model shadow execution | PROV-A | New non-authoritative comparison work |
| INT-01 | Release-tail admission | Every root named in the issue wave | #188 convergence intent |
| TAIL-01 | Quality gate | INT-01 | #188 quality-admission intent |
| TAIL-02 | Documentation review and external-review handoff | TAIL-01 | Standard release tail |
| TAIL-03 | Publication finalization | TAIL-02 | Standard release tail |
| TAIL-04 | Internal review | TAIL-03 | Standard release tail |
| TAIL-05 | External / third-party review | TAIL-04 | Standard release tail |
| TAIL-06 | Review findings remediation | TAIL-05 | Standard release tail |
| TAIL-07 | Next-milestone planning | TAIL-06 | #190 successor-planning intent |
| TAIL-08 | Next-milestone closeout plan | TAIL-07 | Standard release tail |
| TAIL-09 | Next milestone review pass | TAIL-08 | Standard release tail |
| TAIL-10 | Release ceremony | TAIL-09 | #189 ceremony intent |

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
