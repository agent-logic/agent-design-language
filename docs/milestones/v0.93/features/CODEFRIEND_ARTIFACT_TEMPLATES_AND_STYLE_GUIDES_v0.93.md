# CodeFriend versioned artifact templates and style guides

## Status

Reconciled draft under #922. Operator-confirmed v0.93 scope; no implementation, milestone opening or release acceptance claimed.

## Purpose

Every supported document, diagram, review and test has a versioned template and style guide, prepared through human-friendly named fields and rendered as branded output.

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #922.

## Template Rules

Planning structure validation does not establish implemented behavior.

## Context

Follow accepted v0.92.2 closure, explicit v0.93 opening and RD-11 repository-split acceptance. Early #922 drafting does not satisfy final #921 handoff.

## Coverage / Ownership

Owner: codefriend. Results: CT-01 through CT-10 in dependency order. Named people and resource limits are assigned before execution.

## Overview

- **CT-01:** Every enabled CodeFriend producer and exporter maps to a stable kind, field contract, owner, formats and common/per-kind style requirements; accepted blank and filled examples cover the catalog.
- **CT-02:** One immutable common template package provides shared field/schema conventions, manifest/digest rules, REV 01 style tokens and approved font/logo assets for kind-specific template authors.
- **CT-09:** Every supported test-plan, generated-test and test-report kind has a versioned template and guide preserving language/framework rules and distinct generated-versus-executed truth.
- **CT-08:** Every supported diagram kind has a versioned template and guide with semantic shapes/edges and accessible alternatives; the full 4+1 package preserves shared IDs and cross-view consistency.
- **CT-07:** Every enabled review kind has a versioned template and guide preserving scope, evidence, findings, severity/confidence and limitations with representative fixtures.
- **CT-06:** Every enabled document kind has a versioned fill-in-the-blank template, per-kind guide, field contract and representative blank/filled fixture inheriting the common package.
- **CT-03:** An installed user selects a kind and fills or edits named fields and repeatable blocks into a validated version-pinned content artifact with explicit unknowns and unfinished-field state.
- **CT-10:** Installed renderers consume the validated content artifact and pinned template package to produce human-readable preview and supported branded exports without exposing authoring hints.
- **CT-04:** An installed project pins a template release, preserves historical output and explicitly upgrades or rolls back supported schema/render combinations without changing evidence or finding identities.
- **CT-05:** Independent installed qualification covers every enabled kind and supported format with readable representative output, selectable text and accessible diagrams; supported generated tests parse and a bounded behavioral regression fails before its fix and passes afterward.

## Design

Use one coordinated immutable template-set release, initially 1.0.0 independent of product v0.93. Catalog generation entrypoints and exporters exhaustively; each kind has named required/optional fields, examples, repeatable sections, source rules and explicit unknowns. Hide authoring hints and empty optional blocks; block finalized output with missing required fields. Preserve legacy versions honestly and use existing artifact manifests for provenance, not another receipt/checkpoint process.

Include architecture logical, process, development and physical views plus scenarios (full 4+1), linked through shared component IDs. Reviews use clear executive summary, scope, findings, evidence, severity/confidence, recommendations and limitations; scores require a stated rubric. Tests preserve target-language/framework conventions and meaningful behavior assertions.

Apply operator-supplied Agent Logic Visual Brand Style Guide REV 01: Black #151515, White #F4F6FC, Blue #1C5BE9, Red #A63D2F and success Green #168A55. Akzidenz-Grotesk serves body/headings; Courier Prime serves technical IDs and metadata. Resolve font distribution/embedding rights and approved fallbacks explicitly. Use official logo assets, generous margins, grids, thin rules, restrained footers and simple evidence-grounded diagrams; verify contrast and non-color-only meanings. The suggested 70/20/10 composition is guidance, not a pixel gate. No new logo or brand is invented.

Human preview/edit/export consumes the same versioned fields and assets. Pin digests and renderer versions; upgrade deliberately and retain rollback. Qualification covers the supported catalog and representative rendered outputs through existing product checks, without a new review gate for each document.

## Execution Flow

- CT-01 follows CF-01.
- CT-02 follows CT-01.
- CT-09 follows CT-02.
- CT-08 follows CT-02.
- CT-07 follows CT-02.
- CT-06 follows CT-02.
- CT-03 follows CT-06, CT-07, CT-08, CT-09, CF-02.
- CT-10 follows CT-03.
- CT-04 follows CT-10.
- CT-05 follows CT-04.

## Determinism and Constraints

Pin source, contracts, installed artifacts and fixtures. Persist explicit retry/recovery state. Preserve identity, authority and evidence; no undocumented fallback or implied publication.

## Integration Points

Consume versioned Runtime and product contracts after RD-11. CT-05 is required by CF-05 before launch.

## Validation

- CT-01: Unmapped producer; Catch-all hides missing kind; Required field lacks source or unknown-state rule.
- CT-02: Unversioned asset; Unavailable or unlicensed font silently substituted; Manifest digest mismatch.
- CT-09: Unsupported framework; Generated test reported as executed; Empty suite reported as pass.
- CT-08: Unsupported edge; Cross-view identity mismatch; Missing scenario view.
- CT-07: Unsupported grade without rubric; Missing evidence or uncertainty.
- CT-06: Unmapped document producer; Missing required field rule.
- CT-03: Unknown kind accepted; Missing required field hidden; Input loses source evidence.
- CT-10: Required unfinished field finalized; Authoring hints leak; Renderer bypasses pinned version; Unsupported export silently accepted.
- CT-04: Historical output falsely restamped; Digest mismatch; Incompatible schema accepted; Rollback loses evidence.
- CT-05: Missing kind fixture; Clipped text or missing glyphs; Color-only meaning; Unsupported diagram claim; Generated test does not detect regression.

Record actual installed success, refusal and recovery evidence. Design rows have planning-contract proof only; downstream implementation proof remains required.

## Acceptance Criteria

Every result above and its declared negatives passes at its consumer; no enabled kind or admitted identity transition is silently omitted.

## Non-goals

No autonomous public publication, whole-polis reproduction platform, automatic private-state cloning, unrelated subsystem rewrite or inferred legal personhood.

## Risks

Inconsistent versions, unsupported scope and weak recovery proofs can create false completion. Resolve actionable findings before acceptance.

## Future Work

Only explicitly excluded scope may be deferred; the admitted v0.93 results remain release requirements.

## Notes

EXECUTION_PLAN_v0.93.json is the dependency authority; FEATURE_COVERAGE_v0.93.md records feature mappings.

## Current Candidate Mapping

CT-01, CT-02, CT-09, CT-08, CT-07, CT-06, CT-03, CT-10, CT-04, CT-05
