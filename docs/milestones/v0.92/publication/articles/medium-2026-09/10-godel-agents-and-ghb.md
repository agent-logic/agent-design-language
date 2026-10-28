# Gödel Agents and the Gödel–Hadamard–Bayes Algorithm

### You changed the prompt and things seem better. Prove it. Most agent teams cannot, which means they're also unable to prove when a change made things worse.

---

Here is the state of the art in agent behavior management at most companies right now.

Someone notices the agent handling a case badly. They edit the prompt, or add an example, or reorder the instructions. It looks better on the three cases they tried. It ships. Two weeks later a different failure appears, someone edits again, and nobody can say whether the second problem was caused by the first fix.

Changes ship on vibes and get reverted on vibes. There's no baseline, no control, no record of what was tried and rejected. The institutional memory of what *doesn't* work — the most expensive knowledge a team owns — lives in the heads of whoever was on call.

We'd have found this unacceptable in any other engineering discipline a long time ago.

"An agent that improves itself" sounds exciting until you make the sentence precise. What's changing? Who proposed it? What evidence supports it? What alternatives were considered? Who may adopt it? Can the system reject it, replay the experiment, or recover the prior state?

Without answers, self-improvement is hidden mutation with an optimistic name. And "hidden mutation with an optimistic name" is a fair description of how most prompt engineering currently works.

## From editing to experimenting

ADL implements a bounded Gödel experiment package, and the shift it represents is from *editing behavior* to *running experiments on behavior*.

The experiment system records hypotheses, evaluation plans, bounded mutations, baseline and variant relationships, evidence views, and adoption-or-rejection decisions. Runtime commands expose those artifacts for review, so a promotion decision can be inspected next to the evidence that informed it.

That changes what "the agent learned" is allowed to mean. Instead of inferring learning from a later output that seems better, a reviewer can ask concrete questions: Did an explicit experiment occur? Which behavior changed? How was the variant evaluated? Did a governed decision adopt it?

And the property that makes it genuine rather than theatre:

> Rejection is a valid outcome. So is inconclusive evidence. A system that can only promote changes is not doing disciplined experimentation.

A pipeline that always ships its variant isn't an experiment system, it's a deployment system with extra ceremony. Preserving rejections is what makes the record worth keeping — six months in, the log of what was tried and didn't work is more valuable than the log of what shipped.

## Three activities that shouldn't collapse

Underneath the experiment machinery is a cognitive discipline: the **Gödel–Hadamard–Bayes** loop, GHB. It separates three activities that agent systems routinely smear into a single model turn.

The **Gödel phase** asks: *what is actually true right now?* It builds a structured view of the task, prior evidence, constraints, contradictions, failures, and uncertainty. Not introspection — it's grounded in observable state and externally inspectable records.

The **Hadamard phase** asks: *what could be true instead?* It generates bounded alternatives — solution paths, hypotheses, repairs, reframings, candidate mutations. This is the deliberately creative part. Nondeterminism is welcome; scope and constraints still bound the search.

The **Bayes phase** asks: *what should we believe, given the evidence and constraints?* It compares candidates against objectives, prior results, policy, risk, and available evidence. The output may be a selection, a ranking, a rejection, or a finding that the evidence is insufficient.

The names matter less than the separation, and they're ADL phase labels rather than claims about the underlying mathematics. The substance is this: describing what's true is not the same activity as inventing alternatives; inventing an alternative doesn't establish that it's good; and believing a proposal is promising does not grant permission to execute it.

Each of those elisions is a real failure mode you can watch happen in production.

## The speculation-to-execution collapse

The most dangerous of them deserves naming on its own.

A model raises a possibility while exploring. The surrounding system treats the suggestion as a decision. A tool runs before authority, side effects, and evidence were ever evaluated.

That's not a hypothetical — it's the default behavior of any architecture where model output flows directly into a function call. The model was brainstorming. The system executed the brainstorm.

GHB keeps the boundary visible: Hadamard generation may be permissive, Bayes evaluation must be comparative and evidence-aware, and external action still requires admission by the governance layer.

Which means constraint lives in the substrate rather than in the assumed temperament of a model. **A carefully-worded prompt is not a security architecture.** Explicit contracts, authority checks, resource ceilings, replay controls, and audit records are.

## Improvement gets harder when identity is involved

A long-lived agent has to connect experiments to identity and memory, and the questions get sharp fast.

Which version ran the baseline? Which state did the variant inherit? Does an adopted change survive sleep and wake? Can the agent explain the evidence without inventing a cleaner history than it actually has?

Those questions tie experiments to the runtime world model. Experiments need causal placement. Adoption needs authority. Outcomes need durable memory. Recovery needs checkpoints. Review needs an evidence chain.

The result is deliberately slower than silent prompt mutation. It's also the difference between a system you can operate and a system you can only hope about — and the speed advantage of silent mutation disappears the first time you spend a week bisecting a regression you have no record of introducing.

## Controlled cognition, not controlled imagination

The most interesting property of GHB is that it doesn't try to eliminate creativity. It gives creativity somewhere to live.

The system can generate surprising hypotheses, compare them, experiment, learn, and retain results. But every transition carries a different epistemic and governance status:

- A possibility is not a belief.
- A belief is not a permit.
- An experiment is not an adoption.
- An adoption is not authority for arbitrary action.

That's ADL's working definition of controlled cognition. Not a mind with no freedom to explore — an architecture where exploration doesn't erase accountability.

## What the birthday would mean

One further note, because ADL's v0.92 milestone is framed around what it calls the first true Gödel-agent birthday, and the phrase deserves precision rather than atmosphere.

Starting a process is not a birthday. Restoring a snapshot is not a birthday. Naming a test citizen is not a birthday.

The planned event requires identity architecture, continuity evidence, grounded memory, a capability envelope, inherited governance context, witnesses, and a reviewable receipt. v0.92 is active development, so the birthday is not presented here as accomplished — and won't be until there's an evidence record to point at.

Nor would such an event prove consciousness, legal personhood, or unrestricted autonomy. It would prove something narrower and more checkable: that an identity-bearing agent crossed a declared, witnessed, evidence-backed runtime boundary.

We're setting the bar that high on purpose. A lower bar would be easy to clear and would mean nothing.

## Where this goes

The near-term payoff doesn't require any of the ambitious material. It's that behavior changes become experiments with baselines, evidence, and recorded outcomes — including the ones that didn't work.

Next: the runtime world these experiments live in — why a model can only work effectively inside a frame it shares with you, and what it costs to build one.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [Gödel agents explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/GODEL_AGENTS.md)
- [Gödel–Hadamard–Bayes algorithm](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md)
- [Gödel experiment system](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.89/features/GODEL_EXPERIMENT_SYSTEM.md)
- [ADR 0008](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0008-godel-stage-loop-v08.md) — the stage loop decision
