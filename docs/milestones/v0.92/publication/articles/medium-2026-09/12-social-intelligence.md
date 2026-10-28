# ADL and Social Intelligence

### If an attacker can change what your agent believes about another participant, and belief quietly expands capability, you don't have a social model. You have an escalation path.

---

Put two agents in a system and they immediately start modelling each other.

Not because anyone designed it. Because cooperation requires it. What does the other one know? What's it trying to do? Did it misunderstand the handoff? Is it uncertain? Will this action surprise it?

Those inferences get made regardless. The question is whether they live in structured runtime state or drift around inside model context, ungoverned, unversioned, and increasingly authoritative.

Left in context, they degrade in a specific and dangerous way. A plausible narrative about another actor feels more complete than the evidence supports. It gets summarized into the next turn. The hedges fall off in compression. By turn forty the system is acting on "the reviewer agent will approve this" as though it were a fact rather than a guess made thirty-nine turns ago on thin evidence.

And if any of that belief influences what the system is *allowed* to do, you have built a lateral movement path out of vibes.

## Social knowledge is not authority

So ADL draws the hard line first, before anything else:

> A theory-of-mind model may inform reasoning, coordination, and review. It does not grant authority.

An agent's belief that a colleague "would probably agree" cannot substitute for consent. An inference that another actor is confused cannot revoke their standing. A prediction of harmful intent cannot silently bypass the Freedom Gate, access control, or due process.

The security argument for this is straightforward. If an attacker can manipulate one agent's model of another participant — through a poisoned document, a crafted message, a compromised tool result — that manipulation must not automatically expand capability or expose private state. Belief and permission have to be separate subsystems, or every prompt injection becomes a privilege escalation.

The second argument is about paternalism. The more persuasive a social model becomes, the more tempting it is to let it act on others' behalf "helpfully." An agent confident that a user is confused, and empowered to act on that confidence, is a system that overrides people for their own good. Keeping execution authority in separate contracts is what prevents helpfulness from becoming a control surface.

## A model of another is not the other

The epistemics matter too, and they follow from the same discipline.

A theory-of-mind record is a *hypothesis* about another actor's state, intent, knowledge, behavior, or uncertainty. It is not access to that actor's mind. It should be grounded in observable evidence or policy-authorized state, and it should preserve uncertainty rather than rounding it away.

So ADL treats unknowns, corrections, and privacy restrictions as first-class parts of the model. A social hypothesis must be able to say *insufficient evidence*, to become less certain, and to be replaced when later behavior contradicts it.

This protects correctness and dignity at the same time. Once a plausible story about someone is stored, it influences later decisions and gradually acquires the appearance of fact — and the person it's about typically has no idea it exists.

## Updates are runtime events

The theory-of-mind foundation implemented in ADL's Runtime v2 module represents updates as explicit events, carrying evidence references, authority basis, uncertainty changes, and visibility scope.

That structure is what makes review possible. Why did the system change its belief about another agent? Which observation supported it? Who was authorized to see the underlying evidence? Did confidence rise for a valid reason? Was a correction retained, or did it vanish into a summary?

Without an event boundary, social models drift invisibly inside prompts. The system remembers the judgment and forgets how it was formed, which is the worst of both worlds: confident conclusions with no recoverable basis.

Explicit updates also support replay. A reviewer can check whether the same evidence should have produced the same bounded state transition, even though the original natural-language reasoning was generative.

## Privacy, and what survives a restore

Social intelligence concentrates sensitive information by its nature. Observations, inferred preferences, relationship histories, and uncertainty may be appropriate for one actor and entirely inappropriate for another.

So the foundation includes visibility scope rather than assuming a single global view. Operators, participants, reviewers, and public reports get different projections. Redaction preserves *that evidence exists* without exposing its contents — a reader should be able to tell the difference between "nothing here" and "something here you can't see."

The requirement that's easy to miss: **privacy restrictions must survive memory and checkpoint operations.** A safe view must not become unsafe merely because state was restored, summarized, or copied into an audit packet.

That's a real bug class, and a compliance-relevant one. Redaction applied at the presentation layer evaporates the first time state is serialized somewhere else. Restrictions have to travel with the data.

These controls don't solve every privacy problem. They make privacy an architectural input rather than a promise added in prose.

## What comes after the foundation

A bounded theory-of-mind foundation is one ingredient of a social runtime. Richer social intelligence could add relationships, commitments, reputation, norms, negotiation, shared memory, and governance-facing projections.

Each addition carries a specific risk worth naming in advance. Reputation hardens uncertain history into a permanent score. Group models amplify bias. Social learning rewards conformity. Private observations become power.

ADL's polis concept provides somewhere to govern those dynamics, but the current implementation claims neither complete social cognition nor a finished reputation system. Later work has to preserve the foundation's principles: evidence, uncertainty, correction, privacy, and non-authority.

The reputation case deserves particular care. A score is a compression of history that outlives the context that produced it, and a system that cannot forgive is a system that has confused a record with a verdict.

## Intelligence begins with correction

It's easy to define social intelligence as making accurate predictions about others. A more robust definition includes discovering that a prediction was wrong, preserving the correction, and reducing confidence appropriately.

That's the understated strength of this approach. The system doesn't need to claim mind-reading. It needs a disciplined way to hold social hypotheses lightly enough to revise them and firmly enough to inspect their influence.

In a multi-agent world, cooperation depends on models of one another. Governance depends on remembering that every such model is partial.

## Where this goes

Next, and last: what's actually ahead — the identity challenge, learning without hidden mutation, and the standard of evidence we intend to hold ourselves to for the next claim.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [ADR 0019](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0019-theory-of-mind-foundation.md) — the theory of mind foundation decision
- [Theory of Mind foundation](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md)
- [Runtime polis architecture](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md)
- [`theory_of_mind_foundation.rs`](https://github.com/agent-logic/agent-design-language/blob/main/adl/src/runtime_v2/theory_of_mind_foundation.rs) — the implementation
