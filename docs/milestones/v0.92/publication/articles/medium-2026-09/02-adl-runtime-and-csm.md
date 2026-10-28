# The ADL Runtime

### The worst agent failure isn't a crash. It's the run that neither succeeds nor fails — it just keeps going, spending money, until something looks plausible enough to stop on.

---

A crash is a gift. It has a stack trace, a timestamp, and an obvious owner.

The failure that actually costs you looks like this: a long-running agent workflow that doesn't error and doesn't finish. It retries. It rephrases. It tries a slightly different approach, then a slightly different approach again. Two hours and several hundred dollars later it emits something, and the only available quality signal is whether the output *looks* right to a tired human.

Ask the hard question about that run and there's no answer available. Was it converging, or was it stuck? What justified attempt eleven? What would have stopped it, if anything? Did it change strategy at some point — and if so, on what basis?

Nobody knows. Nothing recorded it. The system had no representation of its own progress, so "keep going" was the only policy it could execute.

This is what we mean when we say ADL is not orchestration. Orchestration sequences steps. A runtime owns what happens when steps meet reality.

## Four things that go wrong

The problems ADL was built against are specific and, if you operate agents, familiar:

**Unpredictable execution.** Nondeterministic chains and implicit orchestration produce inconsistent outcomes across runs that should be identical.

**Hidden orchestration.** Workflows, routing, and state transitions are opaque. The topology is an emergent property of prose, so you discover it in production.

**Weak provenance and review.** Limited evidence, thin traceability, no verifiable history. Nothing to hand an auditor, and nothing to hand yourself in three weeks.

**Fragile long-running workflows.** Failures, retries, and concurrency are handled ad hoc per callsite. It holds at demo scale and comes apart under load.

ADL's answer is a substrate with explicit semantics for all four. Start with the one that costs the most money.

## AEE: adaptation that isn't improvisation

The Adaptive Execution Engine is ADL's execution-adaptation layer — bounded strategy selection, recovery, learning, and policy-aware execution.

The governing idea is one sentence: **a system may change strategy only when the change is visible, bounded, and tied to evidence.**

Real agent work doesn't succeed on the first path. That's not a defect, it's the job. What matters is whether the system has a disciplined way to notice progress, stall, recover, stop, or hand off — instead of collapsing into blind retries.

So AEE makes the questions explicit and answerable:

- Has this run converged, stalled, bounded out, been policy-stopped, or reached handoff?
- What progress signal justified another step?
- What stop condition prevented more work?
- Was a strategy change triggered?
- What evidence explains the final control action?

The convergence state is a closed enum — `converged`, `stalled`, `bounded_out`, `policy_stop`, `handoff` — and that's the load-bearing part. "Stalled" and "converged" look identical from outside a system that doesn't distinguish them, which is exactly how a run burns two hours making apparent progress. Once the runtime carries the distinction, stalling becomes an event you can act on rather than a bill you discover later.

The design doc is unusually direct about what it's rejecting. Its stated purpose is to define convergence *"as a real ADL runtime surface rather than a retry story,"* and its first runtime commitment is that **convergence is judged by explicit progress, not blind retries** — another step must be justified by progress *plus* policy and budget allowance.

The evidence lands as `control_path/convergence.json`, with a human-readable summary beside it. The contract requires these explicit:

```
convergence state          converged | stalled | bounded_out | policy_stop | handoff
progress signal family     what kind of progress was being measured
stop-condition family      what would have halted it
iteration count            how many steps
strategy-change count      how many visible strategy changes
reframing triggered        true | false
next control action        what happens now
final gate decision        the governing verdict
```

Two of those deserve a second look. **Visible strategy-change count** is a number you can chart — a run that changed approach six times wasn't adapting, it was thrashing, and now that's a metric rather than a feeling. And **reframing triggered** records that the system reinterpreted the problem, which is the single most consequential thing an agent can do silently.

What we find most clarifying about AEE is its list of non-goals, which reads like a set of promises about what the system will refuse to do:

- AEE is **not** unconstrained recursive self-improvement.
- AEE is **not** autonomous self-modification.
- AEE is **not** a license to keep retrying until output looks plausible.
- AEE is **not** separate from policy, budget, or Freedom Gate constraints.

That third one is the whole argument compressed. An enormous amount of what currently ships as "agentic" is, mechanically, retrying until output looks plausible. It demos beautifully. It is not something you can put on a budget line or defend in a postmortem.

AEE sits *on top of* deterministic execution. It doesn't replace the runtime and it doesn't turn adaptation into magic. It records the control path.

One property worth stating explicitly, because it's a procurement argument rather than an engineering nicety: AEE provides replay semantics **for any agent on any provider** — deliberately not one vendor's tool. Replay, convergence states, and control records are properties of the runtime, not of whichever model you happened to be using when you built the workflow.

Which matters the next time a provider deprecates a model, changes its pricing, or ships something better. Your execution semantics are yours. Migration becomes a catalog change rather than a rewrite of every behavior you'd tuned to one vendor's quirks — and the run you executed last quarter is still replayable afterward.

## The deterministic floor underneath it

Adaptation is only reviewable if the thing being adapted is stable. ADL's runtime resolves schema-validated agent definitions into deterministic execution plans before anything runs — a plan is an explicit object you can print, diff, and review without executing it.

Working today, with tests and evidence in the repository:

- Deterministic plan materialization and execution
- Sequential and fork/join execution
- Bounded concurrency with canonical ready ordering
- Retries and failure controls
- Stable run artifacts and replay-friendly state
- Signing, verification, and provenance
- CLI inspection, debugging, and trace tooling
- Provider abstraction and bounded remote execution

A few of those deserve unpacking, because the phrasing is doing real work.

**Canonical ready ordering** means parallelism has a defined merge rule. Fork/join without a canonical order isn't concurrency, it's a race with better marketing — and it's the reason a workflow can pass forty times and fail the forty-first with no code change.

**Provider abstraction** separates the model reference your policies and agents use from the provider-native model string at the adapter boundary. When a vendor renames a model or you move workloads between providers, that's a catalog change rather than an archaeology expedition through your prompts.

**Trace is execution truth; artifacts carry the payload truth that trace references.** That split is why replay and diff work at all. Trace tells you what happened and in what order; artifacts hold what was actually produced. Debugging a run means reading evidence, not re-reading a conversation.

**Bounded remote execution** means work can run elsewhere without the boundary becoming vague about authority or evidence.

## Governed execution

ADL's governed-execution architecture — Runtime v3 — exposes typed services for governance ingress, Freedom Gate mediation, bounded actuation, and audit.

A status note, because it matters and it's the kind of thing that usually gets blurred: as of v0.91.7, Runtime v3 is selectable through an explicit CLI compatibility boundary. **Runtime v2 remains the default runtime.** The cutover decision is recorded and deliberate — v3 is explicit opt-in, v2 decommission is not authorized, and a default switch is gated on the aggregate cutover proof. If you clone the repo today and run it without a selector, you get v2.

The mechanism worth knowing here is the **signed one-shot permit**. A proposed action arrives with identity, policy, resource, and authority information. The Freedom Gate admits or refuses it. If admitted, the actuation layer executes against a permit that is signed and good exactly once. Authorization is not a mode the system enters and stays in; it's a token attached to a single act.

Governance and actuation state participate in checkpoint and restore, which closes a failure mode most systems never notice: a restart cannot silently revive revoked authority. If a permission was withdrawn between checkpoint and restore, the restored process does not quietly get it back.

Article 4 covers the Freedom Gate in full. The point for a runtime article is narrower and more practical: authority is a runtime concern with typed services and audit output, not a policy document that lives in a wiki.

## Why this runtime is being built toward a world

Everything above is infrastructure for work that outlives a single request — and once work outlives the request, you need somewhere for it to live.

That's why the runtime bothers with checkpoint semantics, lifetime-relative time, and continuity checks on wake rather than treating a restart as a fresh start. A workflow that runs for three days across two process restarts is not an exotic case; it's Tuesday, and it's the case most agent frameworks handle by losing everything and hoping the transcript is enough.

The full runtime world model gets its own article later in this series. For operational purposes the relevant fact is narrower: state that survives a restart is a design decision made early or not at all.

## What's built, and what's active

Built and runnable from a clone: the eight capabilities above, governed tools through UTS v1.0 and ACC v1.0 (completed in v0.90.5), Runtime v3 governed execution as explicit opt-in, bounded CSM runs, run artifacts, and operator-facing proof surfaces. The repository is public and Apache-2.0.

Under active development in the v0.92.2 line: C-SDLC as default operation, the signed trace substrate and ObsMem handoff, long-lived and Polis runtime hardening, security and continuous adversarial verification and threat-modeling integration, and operational tooling maturity.

Not claimed: a complete distributed authority system, production message-bus transport, a universal policy language, a finished operator interface, or a fully inhabited runtime.

We keep drawing that line explicitly because the alternative is asking you to take engineering claims on vibes, which is the thing this entire architecture exists to stop doing.

## Five minutes, from a clean checkout

The fastest way to see whether any of this is real is to print a plan without running it.

Clone the repository, then:

```
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- \
  adl/examples/v0-91-chatgpt-gemini-claude-triad-conversation.adl.yaml --print-plan
```

That's a three-model multi-agent workflow resolving into an explicit execution plan — printed, not executed. It's the whole argument of this article in one command: the topology is an object you can read, diff against last week's, and review *before* anything touches the world.

If you'd rather see the evidence layer, generate a proof packet:

```
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 cognitive-being-flagship-demo \
  --out artifacts/quickstart/cognitive-being-flagship
```

Then read what lands in the output directory. Run artifacts, traces, and the review surfaces are the actual deliverable — reading one does more to explain the architecture than another thousand words from us.

## Where this goes

Predictable, inspectable, governed, extensible — from agent definition to audited outcome. That's the whole proposition, and the unglamorous middle of it is the part that determines whether an agent system can be operated by people who weren't there when it was written.

Next: governed tools. Tool calling got standardized; tool *governance* didn't, and the gap between those two facts is where most of the operational risk currently lives.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [AEE explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/AEE.md) — bounded strategy selection, recovery, and stop conditions
- [AEE convergence model](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md)
- [UTS + ACC explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/UTS_AND_ACC.md) — governed tool execution
- [CSM explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/CSM.md) — the runtime world model
- [Feature list](https://github.com/agent-logic/agent-design-language/blob/main/docs/planning/ADL_FEATURE_LIST.md) — canonical capability and roadmap truth
- [v0.92.2 milestone](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92.2/README.md) — active development
