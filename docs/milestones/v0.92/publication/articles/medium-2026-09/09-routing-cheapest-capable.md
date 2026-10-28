# The Cheapest Capable Agent

### You're paying frontier prices for work a smaller model could do. You keep paying because the only alternative is guessing, and guessing wrong is expensive in a way the invoice doesn't show.

---

Look at where your inference budget actually goes.

Some of it buys genuinely hard reasoning — the synthesis step, the architectural call, the review that catches something subtle. Most of it doesn't. Most of it is classification, extraction, formatting, summarising, routine validation, and the hundred small steps between the interesting ones. Work a model a tenth the price would do correctly.

Everyone knows this. Almost nobody acts on it, and the reason isn't laziness.

The reason is that **routing on price requires knowing what's capable**, and capability is exactly what nobody measures. So the safe default is to send everything to the best model you have, absorb the cost, and treat it as the price of not being wrong.

It's a rational response to missing information. It's also the largest recoverable line item in most agent budgets.

## Why naive cheap-routing fails

The obvious fix — send easy things to the cheap model — breaks on first contact, for three reasons.

**Vendor claims aren't evidence.** Every provider publishes a capability list. That list describes what a lane *claims* to support, under conditions that suit the vendor. It tells you nothing about whether a particular model handles *your* review task, on *your* codebase, with *your* tool surface.

**"Capable" isn't a global property.** A model that's excellent at synthesis can be unreliable at structured validation. One that writes good code can be a poor reviewer of it. Treating capability as a single axis — better model, worse model — throws away the only distinction that would let you route intelligently.

**Failure is silent.** When the cheap lane fails, it frequently fails by producing something plausible. Nobody gets an error. The bad output flows downstream, and you discover the routing decision three steps later, or never.

Which means the question isn't "which model is cheaper." It's **which lane has demonstrated it can do this specific job, and what happens when it can't.**

## Three things that get conflated

ADL separates them, in an architecture decision accepted at v0.91.6:

**Availability** proves only that a provider path can be reached under approved credentials and setup. Nothing more. Reachability is not readiness, and mistaking the two is how a workflow ends up pointed at a model nobody evaluated.

**Capability profiles** describe what a provider or model lane *claims* to support. Useful for filtering. Not evidence.

**Role suitability evidence** describes whether a model behaves reliably enough for a *specific ADL role* — review, planning, coding, synthesis, validation, local multi-agent work.

That third surface is the one that makes routing possible, and it's the one the industry is missing. Suitability is measured per role, against retained evidence, rather than asserted globally. ADL distinguishes **capability** — what an actor is declared or observed to be able to do — from **aptitude**, the measured tendency across capability tests, which is explicitly evidence-bearing and varies by model, prompt, tool and domain.

So the routing question becomes answerable: *for this role, which lanes have evidence, what does that evidence say, and which is cheapest among those that qualify?*

Cheapest capable. Not cheapest, and not most capable.

## Escalation is a first-class outcome, not a fallback

The other half of the problem is what happens when the cheap lane isn't enough — and this is where most cost-routing schemes quietly fall apart, because their failure path is a retry loop.

ADL already models this. **Handoff** is one of the adaptive execution engine's five terminal convergence states, alongside `converged`, `stalled`, `bounded_out` and `policy_stop`. A run that needs a stronger lane doesn't fail and doesn't spin. It reaches a named state that means *this needs someone else*.

That matters for three reasons.

It's **distinguishable**. Handoff isn't a timeout or an exception. It's a verdict, recorded as such, and you can count it. A workload where thirty percent of runs escalate is telling you something specific about your routing thresholds.

It's **justified**. The convergence record carries the progress signal, the stop condition, the iteration count, whether a strategy change was triggered, and the evidence explaining the final control action. An escalation arrives with the reason it happened attached.

And it **carries context**. ACIP — the Agent Communication and Invocation Protocol — treats consultation, delegation, review, handoff and negotiation as distinct interaction types rather than one undifferentiated "call another agent." Escalating upstream isn't re-prompting a bigger model from scratch. It's a handoff with the work already done, the approach already tried, and the reason the cheap lane stopped.

That last point is where the economics actually land. Naive cascading pays twice: once for the cheap attempt that failed, once for the expensive attempt that starts over. A handoff that transfers state and rationale pays for the cheap attempt and a *shorter* expensive one.

## Routing recommends. It does not decide.

Here's the constraint that separates this from a load balancer, and it's stated as a hard boundary rather than a guideline.

Advisory provider-role logic **may recommend a lane, but must not overrule credential policy, cost policy, security and adversarial-verification boundaries, or issue-specific validation requirements.**

Read that as a list of things a cost optimiser is not allowed to win against.

It cannot route around a credential boundary because the cheap lane is cheaper. It cannot select a model that a security review excluded. It cannot skip a validation requirement because a faster lane would satisfy the deadline. The glossary makes the same point in the other direction: routing preference must not be confused with capability evidence or authorization.

This is the same separation the whole series is built on, applied to spend. A recommendation is not a permit. The routing layer proposes the cheapest qualifying lane; policy and the governance layer decide whether that proposal is admissible. Sufficient budget doesn't purchase permission, and neither does insufficient budget purchase an exemption.

Without that boundary, cost optimisation becomes a privilege-escalation path with a business case — the cheapest lane is also, frequently, the least constrained one.

## The receipts

A routing decision that saves money and can't be audited is a decision you'll eventually have to defend without evidence.

ADL's economic accounting design holds one invariant above the rest:

> **No economic state change without a ledger entry.**

No hidden balance mutation, no implicit payout, no invisible reservation change, no silent settlement. The ledger is the economic analogue of the trace — the durable record of what changed.

Entries are deterministic, so replaying the same inputs reconstructs the same ledger state. They're explicit about *why* each entry exists. And every one is trace-linked: a reviewer can move from a trace event to its ledger entry and back without ambiguity.

Applied to routing, that means each decision leaves a record you can interrogate afterward: which lane was selected, against which suitability evidence, what it cost, whether it escalated, and what the escalation cost on top. Reservations are visible rather than implied — an account can't present funds as available when they're already committed to work in flight.

Aggregate those and the questions that were unanswerable in the last article become arithmetic. What fraction of spend went to the frontier lane, and how much of that escalated from a cheaper one? Which roles never justify the expensive model? Which role's escalation rate is climbing, and is that a model regression or a change in the work?

Status, stated plainly: ADR 0041 and the provider suitability surfaces are **accepted architecture** at v0.91.6. The ledger entry families and economic accounting schema are **draft designs at v0.1**, targeted at a later milestone. The cognitive scheduler that would automate lane selection is **planned**, not shipped. What exists today is the evidence layer and the boundary; the automation sits on top of it.

## Why this gets harder, not easier

Two trends point the same way.

Model prices keep falling and model variety keeps growing, which sounds like it makes routing less important and does the opposite. More lanes means more decisions, and a wider spread between the cheapest qualifying option and the default. The cost of *not* routing rises as the menu expands.

And agent workloads are getting longer. A single-shot request routed poorly wastes one call. A multi-step workflow routed poorly wastes every step, compounds errors across them, and escalates late — after the cheap lane has already spent your budget failing quietly.

Which is the argument for putting suitability evidence, named escalation states and ledger entries underneath routing rather than bolting a price table onto the front of it. The decision is easy. Knowing whether the decision was right, and proving it afterward, is the part that needs architecture.

## Where this goes

Cheapest capable, escalate on evidence, never override policy, record every economic consequence.

None of that requires an agent economy, a market, or a payment rail. It requires knowing what your lanes can actually do, having a named state for *this needs a better one*, and writing down what happened.

Next: how a system proposes changes to its own behavior without being trusted to grade its own homework — Gödel agents, the Gödel–Hadamard–Bayes loop, and governed experimentation.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [ADR 0041 — provider/model suitability boundary](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0041-provider-model-suitability-boundary-v2.md) — availability, capability, suitability as distinct surfaces
- [AEE explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/AEE.md) — convergence states including handoff
- [ACIP explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/ACIP.md) — consultation, delegation, handoff, negotiation
- [Provider capability and transport architecture](https://github.com/agent-logic/agent-design-language/blob/main/docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md)
- [Glossary](https://github.com/agent-logic/agent-design-language/blob/main/docs/GLOSSARY.md) — capability, aptitude, routing preference
