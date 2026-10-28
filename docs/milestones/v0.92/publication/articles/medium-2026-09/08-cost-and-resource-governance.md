# Cost and Resource Governance

### When limits are implicit, agents hit timeouts and throttling with no model of why — and operators see the cost after the fact. That's not a pricing problem. It's a design defect.

---

Every team that puts agents into production eventually has the same meeting.

Someone brings a bill. The bill is larger than expected, and more importantly it's *unattributable*. Which workflow spent it? Which agent? Was any of it useful? Was a single run responsible for forty percent of the month, and if so, what was that run doing after the first twenty minutes?

Nobody can say, because the system had no representation of resources as anything other than something it consumed until it stopped.

An agent that lives for one request can borrow from an application budget and disappear. A persistent population of agents cannot. Once work outlives the request — and once several of them run concurrently — allocation stops being an infrastructure detail and becomes a governance question.

These are economic questions even when no currency is involved.

## Four scarce goods

ADL identifies four resource classes, and the useful move is treating all four as first-class rather than obsessing over the one with the obvious price tag.

**Compute** — model inference and execution cycles. The most visible cost and the one everyone watches.

**Memory** — durable storage and retrieval bandwidth. Long-lived agents create pressure not just to store more, but to decide what stays available and at what fidelity.

**Attention** — scheduling priority and access to shared services. In a multi-agent system, attention determines which goals advance and which quietly wait forever.

**Bandwidth** — communication between agents and access to tools or external APIs. It governs both cost and the rate at which an agent can affect its environment.

Making these explicit changes the system's design. A runtime can record allocation, refusal, consumption, and remaining capacity. An agent can reason about alternatives when its preferred route is too expensive. A reviewer can see whether resource policy shaped an outcome — which is a question you currently cannot ask at all.

Note that only the first is denominated in dollars, and the other three are how the first one gets wasted. An agent starved of attention still burns compute waiting. An agent with poor memory economics re-derives things it already knew, at full inference price, forever.

## A budget is a capability envelope

Budgets get described as restrictions. That framing is exactly backwards, and it's why they usually get bolted on last.

An agent with a clear resource grant can choose among strategies inside known limits. It can weigh whether a large model call, an extended search, an additional experiment, or a memory retrieval is justified. It can stop, or ask for approval, before exhausting shared capacity.

Compare that to the alternative — which is what most systems ship today:

> When limits are implicit, agents encounter timeouts, throttling, or arbitrary termination without a model of why. Operators see costs after the fact. Other agents experience starvation with no visible policy.

Read that as three separate failures, because it is. The agent can't reason about a constraint it can't see. The operator can't intervene in a spend they learn about in arrears. And the *other* agents are being quietly deprived by a policy nobody wrote down and nobody can inspect.

Explicit budgets don't make allocation fair. They make it **inspectable**, which is the precondition for making it fair.

## This is where stop conditions come from

Resource governance connects directly to the runaway-loop problem. An adaptive execution layer can notice a run has stalled — but "stalled" only becomes actionable if something defines what further work would cost and what remains available.

So the budget participates in the convergence contract rather than sitting beside it. The runtime's termination states are a closed set — `converged`, `stalled`, `bounded_out`, `policy_stop`, `handoff` — and two of those are resource verdicts. `bounded_out` means the run hit a ceiling. `policy_stop` means a rule halted it. Neither is the same event as `converged`, and none of them is the same as "it finished."

The governing rule is that another step must be justified by progress *plus* policy and budget allowance. Budget isn't a kill switch bolted on at the edge; it's one of the two things that has to agree before the system is permitted to continue.

That distinction is worth money. A run that stopped because it exhausted budget, a run that hit a step ceiling, and a run that was stopped by policy call for three different responses — raise the ceiling, fix the decomposition, or revisit the rule. Collapse them into "the run ended" and you've thrown away the diagnosis along with the receipt.

Attribution follows from the same structure. If allocation, consumption, and refusal are recorded against actors and episodes, "which workflow spent the money" stops being an archaeology project.

## Memory has its own economy

Persistent memory makes allocation unusually awkward, because the cheap decision and the correct decision diverge.

Storage is finite. Retrieval costs. Compression loses detail. Forgetting can improve performance in one context while damaging continuity in another. The system needs criteria for retaining evidence, summaries, commitments, relationships, and obsolete state.

The failure modes sit on both sides. Allocate memory purely by immediate utility and an agent loses the history required to explain itself — the records with the lowest day-to-day utility are frequently the ones an auditor asks for. Retain everything forever and privacy, cost, and relevance all degrade together.

An evidence-aware memory economy would distinguish raw artifacts, derived summaries, identity-critical records, private material, and disposable working state — and treat deletion and compression as *governed transitions* rather than silent housekeeping. A summary that quietly replaced its sources is a fact you needed to know about.

ADL has architecture for evidence and continuity. The long-run economics of memory remain genuinely unsolved, here and everywhere else.

## Price is a signal, not the sovereign

There's an obvious next step, and it's worth explaining why we haven't taken it.

Auctions, bidding, and internal markets are a natural mechanism for allocating scarce compute or attention among participants with different priorities. ADL's historical design explores all of them. They remain proposals rather than implementation, and the restraint is deliberate.

A market isn't a neutral arbiter. The initial allocation determines who can bid at all. A well-funded or highly-rewarded agent can monopolize capacity. Short-term prices systematically undervalue maintenance, safety, memory integrity, and the needs of participants with weak bargaining positions — all of which are exactly the things you most want protected in a system running unattended.

So policy, risk, and fairness are allowed to override "highest bid wins." Economic signals feed arbitration without becoming the only expression of value.

This matters most for delegated budgets, which is nearly all of them. An agent's spending power originates in human or institutional authority. **A bid cannot legitimize an action that policy forbids** — otherwise you've built a system where sufficient budget purchases permission, which is a vulnerability with a business model.

Inter-polis exchange — one governed world buying capacity from another — raises further questions about authority, dispute, provenance, and transfer. ADL requires explicit traces and governance on both sides of any such exchange, and defers payment and market implementation entirely. There is no token economy, payment network, or payments-protocol integration, current or imminent.

Introducing money before identity, authority, dispute, security, and evidence boundaries are mature multiplies ambiguity instead of creating an economy.

## Economics as pressure on cognition

Resource limits shape what an agent attempts. A costly strategy gets deferred. Scarcity raises the value of a compact experiment. A safety-critical task can receive priority despite poor economic return. A fair scheduler preserves capacity for actors that would lose every auction.

So economics sits *inside* cognition and governance rather than beside them. It supplies pressure, signals, and constraints. Policy supplies values that price alone can't express. Evidence lets the system explain how the tradeoff was made.

Nobody knows the final mechanism — not us, not the field. But persistent agents make the question unavoidable, because intelligence consumes shared resources, and the rules for allocating them become part of whatever we're building.

In the meantime, the practical claim is narrower and entirely available today: make the limits explicit, record the allocation, and you can answer the question the person holding the invoice is actually asking.

## Where this goes

Next: the largest recoverable line item in most agent budgets — routing work to the cheapest lane that can demonstrably do it, and what happens when that lane can't.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [ADL cost model](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.88/features/ADL_COST_MODEL.md)
- [Economic and resource model](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.90.1/ideas/source_runtime_v2/ECONOMIC_AND_RESOURCE_MODEL.md) — retained historical design
- [Payment and inter-polis deferral](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.90.4/ideas/PAYMENT_AND_INTERPOLIS_DEFERRAL.md)
- [AEE explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/AEE.md) — stop conditions and bounded execution
