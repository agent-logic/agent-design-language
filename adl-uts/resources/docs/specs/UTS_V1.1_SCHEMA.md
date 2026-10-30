# UTS v1.1 Schema

## Status

Normative candidate specification for Universal Tool Schema v1.1.

## Compatibility

UTS v1.1 is additive. It preserves the v1.0 field vocabulary and safety
invariants while adding richer metadata for categories, side effects,
observability, planning, and invocation posture.

## Additions

UTS v1.1 adds:

- `compatible_versions`
- `lifecycle`
- `categories`
- `side_effects`
- `observability`
- `planning`
- `tool_invocation.v1.1` companion metadata

## Safety Invariants

- If `authentication.required` is true, `authentication.mode` must not be
  `none`.
- If `side_effect_class` is `exfiltration`, `exfiltration_risk` must be `high`.
- If `exfiltration_risk` is `high`, `side_effect_class` must be
  `exfiltration`.
- If `side_effects` contains `none`, it must contain no other side-effect
  value.

## Deliberate Pattern Relaxations

UTS v1.1 deliberately relaxes three v1.0 string patterns while preserving the
v1.0 field names and safety invariants:

| Field | v1.0 constraint | v1.1 constraint | Rationale |
| --- | --- | --- | --- |
| `version` | Semantic-version-like pattern | non-empty string | Allows provider, registry, or protocol versions that are not SemVer. |
| `resources[].resource_type` | lowercase token pattern | non-empty string | Allows external resource taxonomy names without forcing UTS-specific slugging. |
| `errors[].code` | lowercase token pattern | non-empty string | Allows native provider/runtime error codes to be represented without lossy rewriting. |

These relaxations do not remove any v1.0 fields and do not grant execution
authority. A runtime that needs stricter local registry hygiene may layer a
profile-specific policy on top of the normative UTS v1.1 schema.

## Categories

`categories` are non-authority tags. They help registries and reviewers cluster
tools, but they do not replace `side_effect_class`, `side_effects`,
`exfiltration_risk`, or external governance.

| Category | Meaning |
| --- | --- |
| `read_only` | The tool is intended to observe or retrieve information without mutating state. |
| `computational` | The tool primarily computes over supplied inputs. |
| `state_mutating` | The tool may change local, remote, or durable state. |
| `external_network` | The tool communicates with a networked service or endpoint. |
| `human_visible` | The tool may create output visible to a person outside the runtime internals. |
| `governance_sensitive` | The tool should be reviewed by a governance or policy companion before use. |
| `identity_sensitive` | The tool touches identity, account, subject, or actor-linked data. |
| `continuity_sensitive` | The tool touches state that may affect durable history, memory, or continuity. |
| `observability_sensitive` | The tool touches monitoring, trace, audit, or telemetry surfaces. |

`identity_sensitive` and `continuity_sensitive` originated in ADL work, but they
are intentionally generic: non-ADL runtimes may interpret them as account,
subject, session, memory, or durable-state sensitivity markers.

## Lifecycle

`lifecycle` is an optional registry and catalog hint with one of:

- `draft`
- `experimental`
- `stable`
- `deprecated`
- `sunset`

It does not change schema validity or authority. Runtimes may use it to warn,
hide, phase out, or route tools through stricter review.

## Invocation Metadata

`tool_invocation.v1.1` records invocation-time intent without becoming the
argument payload or runtime authority surface. It captures caller, tool,
purpose, replay posture, side-effect expectations, observability posture, retry
posture, and extension metadata.

`caller` is a structured object with a required `kind` value of `model`,
`agent`, `human`, or `service`, plus an optional caller `id`. This preserves the
minimum attribution needed for review without requiring a universal identity
system.

`intent` is a non-empty string rather than a closed enum. The suggested
vocabulary below is non-normative; runtimes MAY use it directly, extend it, or
map local intent labels into it:

| Suggested intent | Example |
| --- | --- |
| `read` | Read a weather forecast, document section, or fixture record. |
| `compute` | Convert units, calculate totals, or format data. |
| `search` | Search contacts, files, messages, or registry entries. |
| `mutation` | Update inventory, append a log, or create a draft record. |
| `review` | Ask a tool to produce a bounded review result or checklist. |
| `observability_action` | Query logs, traces, metrics, or provider-status records. |
| `planning` | Generate a plan, dry-run sequence, or proposed execution packet. |
| `simulation` | Run a fixture, model, or dry-run without committing side effects. |
| `governance_action` | Request approval, denial, audit, or policy classification from a companion. |

`side_effect_expectations` uses the same vocabulary as the tool definition's
`side_effects` array: `none`, `local_state`, `external_state`, `irreversible`,
`human_visible`, and `governance_relevant`. An invocation MAY express a subset
of the declared tool side effects, but it must not invent a weaker posture for
the tool it is invoking.

## Non-Authority

UTS v1.1 validity does not imply authority, approval, execution permission,
policy compliance, or safe runtime behavior.

The machine-readable schemas live at:

- `../../schemas/uts/v1.1/universal_tool_schema.v1_1.schema.json`
- `../../schemas/uts/v1.1/tool_invocation.v1_1.schema.json`

## Consumer reconciliation

See [the consumer reconciliation matrix](CONSUMER_RECONCILIATION.md) for the
authoritative schema decisions, legacy invocation mapping requirements, shared
conformance corpus and separation of Runtime policy from UTS validity.
