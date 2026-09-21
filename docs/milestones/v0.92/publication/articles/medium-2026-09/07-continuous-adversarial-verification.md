# Continuous Adversarial Verification

### Your penetration test was accurate on the day it was written. Your agent system changed eleven times since then.

---

A security review is a photograph. An agent system is weather.

The model version moves. Tool definitions get added by someone who needed one. Policies get amended. Dependencies update themselves. Memory accumulates state that didn't exist during the assessment. The system that was reviewed in March is not the system running in June, and nothing about the March report knows that.

If you own a security budget, this is an uncomfortable economics problem before it's a technical one. Point-in-time assessment is expensive, scheduled annually or quarterly because that's what the budget supports, and its findings begin decaying the moment the engagement ends. You're buying a high-confidence answer about a system that no longer exists — and the gap between assessment and reality is now widening faster than the assessment cycle can close it.

For conventional software that decay was slow enough to live with. For a system where a model update can change behavior across every code path simultaneously, and where a teammate can add a tool integration on a Tuesday afternoon, it isn't.

ADL's continuous adversarial verification direction asks a different question: **how does a governed agent runtime examine its own security repeatedly, retain what it learns, and improve its defenses — without turning "self-attack" into an uncontrolled hazard?**

## Why agents change the threat model

Two things shift at once, and they shift in opposite directions.

First, capable models lower the cost of generating and adapting attack hypotheses. They inspect large surfaces, combine patterns, and iterate fast. The strategic consequence is blunt: **if vulnerability discovery becomes cheap, assume a real weakness is discoverable by someone.** Occasional review was a defensible posture when finding bugs was expensive labour. That assumption no longer holds.

Second, an agent runtime introduces boundaries that classical appsec doesn't have vocabulary for:

- Model output may contain *instructions* rather than data.
- Tool calls cause side effects in the world.
- Memory can preserve poisoned state across sessions.
- Delegation can expand authority in ways no single grant reveals.
- Logs can leak secrets through projections nobody audited.
- Checkpoint restore can revive revoked capability if continuity is wrong.

That last one is a genuinely novel class of bug. "The system restarted and silently got a permission back" has no analogue in a stateless web service, and it won't appear in a scanner's output.

So continuous verification has to examine *runtime governance*, not only application code. Half of the attack surface in an agent system is the authority model.

## Red, blue, and purple are responsibilities, not personalities

ADL models three roles, and it's worth being precise that these are accountable responsibilities with declared authority and visible artifacts — not character prompts.

**Red** develops bounded offensive hypotheses against a declared target: how might this policy, interface, parser, tool boundary, or state transition fail?

**Blue** interprets exploit evidence, proposes mitigations, and tests defensive outcomes.

**Purple** coordinates scope, prioritization, replay, escalation, regression, and durable learning across both.

The constraints are what make them real. Red does not acquire permission to attack arbitrary systems. Blue does not get to mark a mitigation effective without proof. Purple does not erase disagreement to produce a tidy report.

An enormous amount of what passes for AI security work at the moment is a model asked to "act as a penetration tester" and then believed. That's theatre with a good vocabulary. The difference between theatre and verification is whether the output is bounded, authorized, and replayable.

## The loop

The core cycle is compact:

> surface → hypothesis → exploit attempt → defense → replay → learning

Every arrow carries a governance requirement.

The **surface** must be explicitly in scope. A **hypothesis** needs a threat model and a safe execution boundary. An **exploit attempt** must preserve target identity, authorization, resource limits, and evidence. A **defense** must address the observed mechanism rather than only the example input — the difference between fixing a vulnerability and fixing a screenshot. **Replay** must prove the mitigation without reintroducing harm. **Learning** must retain the result in a form later runs can actually use.

This is why "continuous" means something more than running a scanner on a cron. The system needs a durable relationship between attack evidence, defensive change, regression proof, and runtime state. A scanner that finds the same issue every week and produces no accumulating knowledge is a subscription, not a security programme.

## The gate applies to security work too

Security testing is itself a powerful capability. So ADL's Freedom Gate — the runtime admission step that converts a proposed action into scoped, signed authority — applies here completely unchanged.

A red action passes through scoped authority and policy. Dangerous payloads stay inside approved fixtures or isolated targets. Results are redacted by audience. Resource ceilings and stop conditions bound iteration. Refusals and quarantines remain as evidence.

This avoids a contradiction that's easy to walk into: building a safety architecture that disables its own safety controls whenever it does security research. An exception that large tends to become the main entrance.

The same gate protects defensive changes. A proposed mitigation isn't automatically adopted — it needs review, focused validation, and replay against the original evidence. Broader regressions are still possible, so the change enters the normal software lifecycle like any other change. A patch written by a blue agent gets the same scrutiny as a patch written by a person, because the risk profile is the same or worse.

## Security knowledge that accumulates

A report that can't influence the next run has limited value. The longer-range design connects adversarial evidence to runtime memory and governed learning.

A retained record distinguishes the original surface, the hypothesis, exploit status, mitigation, replay result, residual risk, and scope limits. A later agent can retrieve it — but must not inflate a fixture success into a universal claim, which is the most common way security knowledge turns into security folklore.

Negative results need the same discipline. An exploit attempt that failed under one configuration is evidence about *that bounded attempt*. It is not proof that the vulnerability class is absent. The gap between those two statements is where breaches live, and it's precisely the gap that confident summarization erases.

The goal is cumulative security knowledge that preserves its own uncertainty. That's harder than it sounds and more valuable than a clean report.

## What exists, and what doesn't

ADL has documented and exercised bounded red/blue architecture, security bridge surfaces, threat models, source packets, and review evidence. Those artifacts support the architecture and selected integration claims.

They do not constitute external certification, comprehensive production protection, or permission for unrestricted autonomous attack. A broader adversarial-security issue wave is later work, and continuous adversarial verification remains a concrete engineering direction with real integration still ahead of it.

We'd rather say that plainly than imply a security posture we haven't proved. In this domain particularly, overclaiming isn't a marketing risk — it's how people get hurt.

## Where this goes

The central idea is already clear: security should be a governed learning loop whose evidence survives every iteration. In an environment where offensive capability compounds continuously, defensive knowledge has to compound too.

Next: the economics underneath all of this — compute, memory, attention, and bandwidth as governed resources, and why implicit scarcity is how agent systems surprise you on the invoice.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [Red/blue security explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/RED_BLUE_SECURITY.md)
- [Continuous verification and exploit generation](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.89.1/features/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md)
- [CAV threat model](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md)
- [Security bridge and CAV](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md)
