# Architecture Cognition

Status: requirements retained; component evidence is indexed in the [review handoff](../evidence/issue-917/HANDOFF.md), with final Beta 1 qualification pending. Owners: CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE.

CodeFriend must explain dependencies, boundaries, layering, coupling and connascence, architecture drift, blast radius, architectural quanta, and available ADR/rationale signals. Findings connect observed evidence to an explicit inference and surface confidence or unknowns.

Acceptance uses known architecture fixtures, reviewer calibration, and sampled false-positive analysis. Opaque scores and automatic architecture rewrites are rejected.

The listed owners deliver separate working tasks, with complete consumer and failure evidence defined in the [atomic task contracts](../ATOMIC_TASK_CONTRACTS_v0.92.2.md). A feature packet or schema alone cannot close an implementation row.

Issue #1109 adds a Beta 1 acceptance requirement: one evidence-grounded 4+1 package
for an admitted repository revision, generated and retrieved through the existing
CodeFriend journey. It must include logical, development, process and declared
deployment views, representative scenarios including failure/recovery when supported,
editable diagram sources, rendered diagrams, evidence references and a scenario-to-view
table in the existing Markdown/HTML/PDF surfaces. Shared identities must remain
consistent across views; conflicts and missing evidence must remain explicit.

A populated sufficient-evidence fixture, truthful partial/conflict fixtures, installed
ADL and bounded external-repository generation, and HTML/PDF readability checks are
required before this outcome is complete. A schema, hand-written sample, source-only
test or empty view does not qualify. Independent Beta 1 qualification remains incomplete: closed/not-planned #915
is succeeded by v0.93.1 #1150 after repairs #1148/#1149. This requirement
does not declare qualification complete.
