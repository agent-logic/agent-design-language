# v0.92.2 Sprint Plan

Status: planned; sequence expresses dependencies, not calendar deadlines.

## First sprint — C-SDLC v3 simplification alongside Runtime

SIM-UMBRELLA opens coordination; SIM-01 → SIM-02 → SIM-03 → SIM-04 → SIM-05 → SIM-06 → SIM-07 → SIM-08 → SIM-09 is one coherent sprint. It starts through its own readiness and dedicated issue launch, in parallel with Runtime, without waiting for WP-01 or unrelated prior closeout. The umbrella completes after SIM-09 and converges at TAIL-01. Resolve C-SDLC/Runtime shared-path and installed-binary ownership before overlapping writes. An eventual C-SDLC writer pause requires separate explicit authorization; it does not pause Runtime/provider services. See the [complete plan](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md).

## CodeFriend implementation waves

CF-ADAPTER opens local ingestion; CF-ADAPTER-GITHUB and CF-ADAPTER-CI follow it. CF-EVIDENCE admits local packets. CF-REVIEW follows CF-EVIDENCE and RT-PROVIDER; CF-SHELL then proves a real review journey. CF-SYNTHESIS follows the lane runner; CF-REMEDIATE and CF-TESTPLAN follow synthesis. CF-COG provides structure, followed by CF-COG-IMPACT and CF-COG-RATIONALE; CF-COG-DRIFT additionally consumes CF-MEMORY. CF-GOV-CI follows the working local CF-GOV runner. CF-UX owns approval; CF-RENDER-MD consumes approval, synthesis and both action plans; CF-RENDER-HTML and CF-RENDER-PDF follow Markdown for parity. RT-COST precedes PLAT-PROVIDER (also consuming merged #622), then RT-PROVIDER/#855. PLAT-MLX and PLAT-PAIR consume PLAT-PROVIDER. PLAT-MEMORY consumes CF-EVIDENCE and CF-MEMORY. CF-INTEGRATE connects the complete product consumers, all ingestion routes, RT-PROVIDER and PLAT-MEMORY first. CF-PROOF then independently qualifies that installed integrated product on ADL and the pinned external repository. TAIL-01 consumes both completed results.

WP-01 creates only separately authorized rows. All seven planning tasks remain required. Supporting tasks retain the exact dependencies in the issue wave. QUAL-EVIDENCE consumes the four independently completed Runtime repair/proof tasks. CSDLC-REMOTE follows local decomposition and CSDLC-MERGE; shared-path ownership with SIM is reconciled before execution.

## Milestone closeout — Canonical Release Tail

Run TAIL-01 through TAIL-10 in exact order. Individual issue closeout is asynchronous; downstream work depends on merged product authority and the stated release gate, not on bookkeeping receipts.

## Scope Control

Deferred connectors, autonomous mutation, public customer-scale or multi-tenant deployment, ATE, OCI model packaging, optional modernization, and Runtime v4 require separate admission. The bounded static Observatory sidecar does not authorize those broader deployment programs.

## Existing issues outside new-wave startup

Closed merged #717 and #718 are v0.92.1 predecessor inputs. #720 uses its own independent authority; #848, #849, #852, #854, #855, #861, and #862 use their mapped existing authority after WP-01. #848 is a decision row rather than split implementation. This schedules existing v0.92.2 work; #523 does not implement it.
