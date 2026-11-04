# GHB v1 packet schema

`packet.schema.json` is the sole editable JSON Schema for the versioned GHB
packet proposed in [ADL #1414](../../../../../docs/milestones/v0.93.2/ghb/NORMATIVE_SPEC_1414.md).
It uses Draft 7 and local `#/definitions/…` references only. Its stable public
raw ID identifies the schema; readers must use authenticated installed bytes,
not fetch that URL during validation. The frozen v0.8 distribution is unchanged.

A packet has `schema`, `contract_version`, `profile_id`, `status`,
`external_refs`, `cycle` and `artifacts`. Each artifact has `artifact_id`,
`artifact_type`, `owner`, `schema_version` and a typed `payload`. References
contain `artifact_id`, `owner`, `schema_version` and lowercase `sha256`.
The digest covers the **entire artifact record**, encoded as sorted-key compact
UTF-8 JSON without a trailing newline. The data domain permits integers only,
ASCII object keys, no duplicate keys and no floating-point values; decoders must
reject ambiguity before computing a digest. JSON escaping and canonicalization
must follow the public validator, not platform-dependent pretty printing.

`external_refs` declares referenced identities that are not embedded artifacts.
It does not authenticate a receipt, admit an operation or establish currentness.
The consumer must receive independently selected expected context for all cycle
reference fields and compare exact owner/version/digest bindings. Schema success
alone cannot reject a stale but internally consistent parent or prove an external
owner result. Unresolved or unknown authority must remain an explicit gap.

Budget `ceilings`, `consumed` and transition `remaining_budget` use the ten
canonical fields from the profile. `monetary_cost` is an integer amount in
microUSD; the other units are counts or elapsed seconds. Ceilings, remaining
amounts, serial candidate restrictions and deadline arithmetic require semantic
checks. A configured amount does not itself reserve resources or authorize use.

The first-profile candidate mutation can contain only typed
`task_policy_parameters` and `reasoning_guidance`. The proposed lending parameter
vocabulary is an outage-extension ceiling, required registry confirmation, and a
suspended-borrower policy that denies or defers to the existing owner. These data
cannot grant tool access or override protected controls. Any new profile or data
vocabulary needs explicit versioned review. This packet does not prescribe the
later WP10 improvement-component profile.

The schema separates task effects from body activation and limits replay
transition effects to none or artifact retention. Quantitative belief requires
model, prior, likelihood, units, dependence treatment, calibration assumptions
and an integer-ratio posterior. Qualitative/unmodeled assessment excludes that
quantitative payload. It does not mathematically validate a Bayesian model or
turn verbal confidence into a posterior. Operation outcomes retain explicit
reason, operation identity, partial effects and recovery-owner reference.

The public semantic validator checks immutable reference resolution and canonical
artifact digests, separately supplied current context (including incarnation),
completed transition state order, loop/disposition compatibility, consumed and
remaining ceilings, recovery blocking and posterior range. It refuses remote
schema references. It does not authenticate owner receipts, mathematically verify
Bayes, reserve budget or perform deadline admission. Identity continuity,
proposal/disposition causal binding, authentic admission before dispatch, and
current deadline/authority decisions remain native-owner responsibilities. A
successful validation result explicitly reports `authority_authenticated=false`
and `execution_performed=false`; declared references cannot supply these facts.

The two [example packets](../../../examples/ghb/v1/) are synthetic,
`proposed_unexecuted` replay material. They have no transition execution and no
provider calls. The task result is **deferred**, with zero invocations and explicit
synthetic absence declarations in the required admission/result reference fields;
these are not owner receipts. The improvement example has no frozen evaluator,
corpus/split/rubric or threshold and sets `execution_admissible=false`. Neither
example demonstrates a resident task success, installed consumption, activation,
quality gain, Gate A or Gate B.

PVF: deterministic offline schema/data contract validation, small local CPU and
filesystem, no credentials, services or providers. Actual installed Runtime
consumer parity is a separate owner-package proof.
