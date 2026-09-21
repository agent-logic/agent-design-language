# The Cognitive SpaceTime Manifold

### The Principle of Cognitive Relativity: a language model can only work effectively when it shares a frame of reference with you. Almost everything we currently call "context engineering" is frame synchronization, performed by hand, forever.

---

You have been on a project for six months. You know why the retry logic looks wrong but isn't, which service the compliance team made you carve out, what the last incident cost, and which of last quarter's decisions are load-bearing.

You ask a model for help. It answers fluently and wrongly.

The reflex is to reach for capability — better model, better prompt, more parameters. But look at what actually happened. Its *now* and your *now* are different objects. Yours sits at a position in six months of accumulated state, causality, and commitment. Its now is a flat window of text somebody assembled in the last few seconds. It isn't reasoning badly about your situation; it's reasoning correctly about a different one.

And notice what every experienced user does about it. You paste the context back in. You restate the constraints. You re-explain the decision you already explained on Tuesday. You keep a file of project background you shovel into the window at the start of each session.

That's frame synchronization, done manually, by a human, in every session, permanently. We've dressed it up with the name "context engineering," and it works about as well as manual memory management did.

## The Principle of Cognitive Relativity

Almost everyone misremembers what Einstein did.

The popular version is "everything is relative" — a slogan about subjectivity, about how nobody's viewpoint is better than anyone else's. That is very nearly the opposite of the actual theory.

Special relativity begins with a postulate about what is *shared*: **the laws of physics are the same in all inertial reference frames.** It's stated in the opening pages of "On the Electrodynamics of Moving Bodies," before the first numbered section — Einstein raises it to a postulate, pairs it with the constancy of the speed of light, and derives the rest from those two. Observers in relative motion will disagree about measurements — how long something took, how far apart two events were, even whether two events happened at the same time. Simultaneity itself turns out to be frame-dependent. But they do not disagree about the physics. The laws are invariant. So is the speed of light. So is the spacetime interval between two events.

That invariance is what makes the disagreement *productive*. Two observers who share the laws can compare notes: there's a transformation that carries one frame's measurements into the other's, and after applying it they agree completely about what happened. Relativity isn't a theory about how observers are trapped in their own perspectives. It's a theory about the structure that lets them communicate across perspectives without either one having to be privileged.

Take that structure seriously for cognition and you get the principle this runtime is derived from:

> **The Principle of Cognitive Relativity.** There is no privileged cognitive frame. The meaning of an event, a claim, a commitment, or an instruction is relative to the frame in which it is evaluated. Participants can therefore disagree about measurements — what is now, what is recent, what is settled — while still cooperating, but only where they share the laws of the world they are reasoning about: its causality, its authority, its evidence, and its continuity.
>
> Shared physics first. Shared measurements never.

And the operational consequence, which is the sentence the whole architecture hangs on:

**A language model can only work effectively when it shares the manifold with humans.**

Look at what a human and a model actually share by default: nothing. Different clocks — one has continuous time, the other has no time at all between invocations. Different histories. Different notions of what's simultaneous, what's settled, what's still open. Two byte-identical requests, issued an hour apart, can mean different things, and nothing in either request says which.

That isn't two observers in different frames. It's two observers with **no shared physics at all** — and the difference matters enormously. Relativistic observers disagree and can reconcile, because a transformation exists. Participants with no common world don't disagree in any useful sense; they produce confident, mutually incompatible answers and no procedure exists for reconciling them. What looks like a conversation is two systems computing over different worlds while using the same words.

This relocates the problem. Model-human misalignment, in the ordinary daily sense of a model doing an unhelpful thing, is usually neither a capability failure nor a values failure. It's a frame failure — and specifically a *missing invariants* failure. Neither participant can see the discrepancy, because seeing it would require the shared structure whose absence is the problem.

You cannot fix that with a better prompt, for the same reason you cannot fix a units mismatch with a faster processor. Mars Climate Orbiter didn't need a better navigation computer.

So the fix has to be structural: establish the invariants, build the frame that carries them, make it durable, and put both participants inside it. The rest of this article is what that costs.

## Chronosense: how you get a position

Here's the part that took us longest to see clearly.

A frame is not a bag of facts. A frame is a **position** — and a position is what makes facts locatable.

That distinction is exactly what context-stuffing cannot reach, and it explains why the technique fails in a way more tokens will never fix. When you paste six months of project background into a window, you transmit *content*. You do not transmit *position*. The model receives a pile of true statements with no coordinate to evaluate them from — no answer to the only question that would make them mean anything: **when am I, and where in this world am I standing?**

Facts without a position don't compose into a frame. They compose into a very long prompt.

**Chronosense** is ADL's mechanism for binding a participant to a coordinate on the manifold. The repository defines it as an agent's intrinsic capacity to perceive, structure, and reason over its own temporal continuity — linking past executions, present state, and future intent into a coherent identity.

It decomposes into four capacities:

- **Now-sense** — locating the current moment within the agent's execution and cognitive trajectory
- **Sequence-sense** — ordering events causally, distinguishing before from after
- **Duration-sense** — representing relative duration, spacing, and temporal density between events
- **Lifetime-sense** — situating the present within the agent's entire existence, from origin to current state

Together those constitute *temporal identity*.

### The coordinate is not a timestamp

This is the part that makes it a position rather than a clock reading, and it's where the architecture gets specific.

Chronosense isn't one notion of time. It's a **stack of coordinated clocks**:

- **UTC / wall clock** — shared external reference
- **Monotonic** — strictly increasing execution order
- **Lifetime** — time since the agent's origin
- **Trace / event** — causal ordering of events
- **Narrative** — structured sequences used in reasoning and memory

The position is the whole tuple, not any one reading. And here is the definition that does the most work in the entire system:

> Continuity is the preservation of a coherent mapping across the clock stack.

Not uptime. Not uninterrupted execution. The *mapping*. An agent can be down for a week and remain continuous; an agent can run without interruption and have lost continuity, if the relationship between its clocks silently broke.

Every event, memory, and trace artifact is temporally anchored, carrying at minimum its lifetime-relative age, monotonic order, delta from the prior event, a reference to trace position, and — importantly — confidence or uncertainty where applicable:

```yaml
temporal_anchor:
  agent_age: 00:14:22:07     # lifetime time since ephemeris
  monotonic_order: 88412
  prior_event_delta: 340ms
  trace_position: trace://ep-2291/evt-118
  confidence: inferred        # known | inferred | relative | unknown
```

That last field is a policy ADL calls **temporal honesty**: an agent must distinguish known time from inferred time from relative time from unknown time, and must not perform certainty it doesn't have. Fluent systems are extremely good at stating an uncertain "when" in the same register as a certain one. Making the distinction a required field means the uncertainty survives into the record instead of being smoothed away by good prose.

### Two frames, one translation

Chronosense operates across an **internal frame** — UTC plus monotonic plus lifetime time — and an **external frame**, which is human-local and contextual. The requirement is that agents translate between them without losing coherence.

That requirement is the Principle of Cognitive Relativity implemented rather than asserted, and it's doing the same structural job a coordinate transformation does in physics. Two frames, both legitimate, neither privileged, plus an explicit rule for carrying a statement from one into the other. Your "last Thursday, before the incident" and `monotonic_order: 88412` are the same event in different frames. Neither is the real one. What matters is that the mapping exists and is lossless enough to reason across.

Without that transformation you don't have two perspectives on a shared world — you have two worlds, and the conversation between them only appears to be working.

### The birthday is a coordinate origin

Every agent has what ADL calls a **temporal ephemeris** — a birthday. An immutable origin point, the start of lifetime continuity, the anchor for all lifetime-relative time.

It's worth being clear about why that isn't sentimental. You cannot have lifetime-sense without an origin, for the same reason you cannot have a coordinate without a zero. The birthday marks a concrete architectural transition:

> stateless execution → persistent identity

Article 10 covers what ADL requires before it will certify that transition for a Gödel agent. The relevant point here is that a birthday is a coordinate system's origin, not a candle on a cake.

### Why any of this matters at the prompt

Return to the six-month project. Every ordinary thing you'd say to a colleague — "before that decision," "since the incident," "while that authority was still active," "what did you know at the time" — is a query that requires a position to evaluate.

In a stateless invocation those aren't difficult. They're **undefined**. There's no origin to measure from.

Which sharpens the diagnosis considerably. A model invoked without chronosense isn't in a bad frame; it's in **no frame**. An observer with no worldline — no position, no ordering, no before-and-after. That's precisely why it can be fluent and wrong at the same moment, and precisely why it can't notice: detecting a frame mismatch requires occupying a frame.

The repo puts it more bluntly than we would have dared to in marketing copy:

> An agent without chronosense is not persistent — it is merely re-instantiated.

## Why the name is the argument

This is where the name stops being decoration.

We call ADL's runtime world model the **Cognitive SpaceTime Manifold** — CSM. It's a big name, and if you assumed it was a key-value store wearing cosmology, that was a reasonable prior. But relativity is not an analogy we reached for after the fact. It's the thing the architecture is *for*.

In a request/response system, an event's meaning is self-contained; everything needed to interpret the call arrives with the call. In a long-lived, multi-participant system, an event's meaning depends on where it sits — what preceded it, which state it inherited, which constraints were in force, what it changed, what it makes possible next. That's a coordinate system, and position determines meaning. "Spacetime" is the honest word for that.

Credit where it belongs: that word isn't Einstein's. The 1905 paper gives you the principle — shared laws, frame-relative measurements — but it doesn't treat space and time as one four-dimensional object. That was Minkowski, three years later, opening his Cologne address with the claim that space by itself and time by itself were destined to fade into shadows. Einstein is said to have initially regarded the reformulation as superfluous mathematics, and then built general relativity on it. So this architecture has two ancestors: the principle it's derived from is Einstein's, and the geometry its name borrows is Minkowski's.

And **manifold** rather than "store" or "context" because the thing we need is one continuous structure that every participant occupies a position within — not a repository one of them queries. A database is something you look at from outside. A manifold is something you're *in*. That distinction is the entire difference between retrieving context and sharing a world.

One concession, so the argument stays honest. "Manifold" is doing more aspirational work than mathematical work today — we're not claiming charts, atlases, and differential structure. We're claiming a world with coordinates, worldlines, located events, and transformations between frames. Read it as ambition rather than theorem, and judge the rest on the engineering.

## What a chat session isn't

A chat session has a beginning, a transcript, and an end. It's a fine abstraction for a great many applications, and if it's sufficient for yours, don't build any of this.

It fails when you need an agent to hold a commitment across days, learn from a prior episode, coexist with other agents, recover from interruption, or be *the same agent* tomorrow. Three things break, and all three are frame failures.

**State doesn't survive, so it gets faked.** The patch is to stuff prior context back into the window — the appearance of continuity without any property continuity is supposed to provide. Nothing inspectable, nothing durable, nothing replayable. A frame reconstructed from text each time is a frame that silently differs each time.

**Memory becomes mythology.** An agent's confident account of its own past is not a memory record. It's a generated narrative about a transcript, with no provenance. It'll tell you what it did last Tuesday with total fluency and there's no artifact behind the claim. Nothing about the format distinguishes recollection from invention.

**Restarts silently resurrect authority.** This is the one that should alarm you. A process dies and returns. A snapshot loads, the name matches, and the system treats it as the same agent — including the authority that agent held. If a permission was revoked between checkpoint and restore, naive rehydration hands it back, and nothing anywhere records that it happened.

None of these are model failures. They're missing-world failures.

## Six things a shared world needs

CSM's answer to what the missing structure consists of:

- **Time and ordering** (ADL calls it *chronosense*), so episodes, commitments, and deadlines have a sequence rather than a vibe — and so both participants agree on what happened before what.
- **State and memory** through observational memory and runtime packets, so later behavior rests on witnessed artifacts.
- **Causality and trace**, so an operator can connect an outcome to the events and policies that produced it.
- **Identity continuity**, so waking a restored agent is distinguishable from starting a fresh process that reuses a label.
- **Governance**, so standing, authority, policy, and external effects stay explicit.
- **Visibility**, so each participant sees the projection appropriate to them — which is where shared frames get genuinely subtle, and where ADL has the most concrete machinery.

## The manifold and the polis

ADL separates the world from the society that governs it. The **manifold** is the world: state, time, memory, causal relationships, execution, continuity. The **polis** is the social, security, policy, and economic order inside it.

The split lets the runtime ask questions a task queue structurally cannot. Which actor has standing? What authority was delegated, by whom, and how deep does the chain go? Citizen or guest? What obligations survive this episode?

One rule worth stating plainly, because it's enforcement rather than metaphor: ADL prohibits the **naked actor** — anything that can observe, communicate, or affect shared world state without declared standing, bounded authority, and traceable identity. If it can touch the world, it has a name, a scope, and a trail. An unnamed participant is a frame you can't reconcile with.

Humans enter the polis as **guests** by default. A human is not directly a citizen; the mediated CSM identity is. That sounds like philosophy right up until you're working out who is accountable for something an operator did through a console at 3 a.m.

None of this makes an agent a legal person, and we don't claim it does. It makes explicit the relationships that otherwise hide in prompts, application code, and operator intuition.

## The Observatory: one world, many frames

Here's where relativity becomes an API rather than a thesis.

If meaning is frame-relative, then "just show everyone the state" is not a coherent goal. The operator, the reviewer, the agent itself, and the public each need a different view — and the differences aren't only about secrecy. They're about which projection is *meaningful* from that position.

So ADL treats it as a **projection** problem. There is one authoritative **private state**, explicitly not ordinary debug data. Everything anyone sees is derived:

- an **operator projection** — status, risk, required action, without raw private-state access unless explicitly authorized
- a **reviewer** view — enough evidence to audit a decision
- a **citizen-facing projection** — what the agent is told about its own transitions
- a **public projection** — redacted, safe for external reporting, must not leak private state

The **Observatory** is the visibility surface serving these: operator and reviewer inspection of packets, reports, status, refusals, wake continuity, and boundary evidence. Its structured artifact is the **visibility packet**, exposing enough status and evidence for review while respecting private-state and projection boundaries. An **operator report** summarizes status, risks, decisions, refusals, continuity evidence, and recommended action in a form a human can act on immediately.

The schema is `adl.csm_visibility_packet.v1`, and its design goal is that any consumer can answer three questions: **what is alive, what is changing, and what requires judgment?** It has fourteen required top-level sections — schema, packet_id, generated_at, source, manifold, kernel, citizens, episodes, freedom_gate, invariants, resources, trace, operator_actions, review.

The section that deserves your attention is `source`:

```json
{
  "schema": "adl.csm_visibility_packet.v1",
  "packet_id": "pkt.observatory.0447",
  "generated_at": "...",
  "source": {
    "mode": "fixture",
    "evidence_level": "fixture_backed"
  },
  "manifold": {
    "manifold_id": "csm.local.01",
    "state": "running",
    "current_tick": 88412,
    "snapshot_status": "clean",
    "health": "nominal",
    "evidence_refs": ["..."]
  },
  "freedom_gate": { "...": "docket of pending and decided requests" },
  "invariants": { "...": "checked, violated, deferred" },
  "operator_actions": { "...": "what a human may do from here" }
}
```

`source.mode` takes one of three values — `fixture`, `captured_artifacts`, or `live_runtime` — and the contract states the rule explicitly: *a fixture packet must say it is a fixture and must not present itself as a live runtime capture.* Alongside it, `evidence_level` distinguishes `fixture_backed`, `artifact_backed`, `live_runtime_backed`, `missing`, and `deferred`.

Sit with that for a second, because it's the whole philosophy compiled into two enum fields. **The packet is required to tell you how much to believe it.** Every major section carries enough evidence references or caveats for a reviewer to work out whether it's backed by real runtime artifacts, fixture data, or planned work that doesn't exist yet.

Most dashboards render a number identically whether it came from production telemetry, a cached estimate, or a default. This one cannot. A view that doesn't declare its own epistemic status is a view that will eventually mislead someone at 3 a.m. — and in frame terms, you're being told exactly what transformation produced the picture you're looking at.

The projection model is also what makes refusal useful. A gate that says no and produces nothing is a dead end. A gate whose docket names the exceeded scope, the evidence reference, and the available operator actions is a workflow. *(Article 4 covers Freedom Gate mechanics in full; the packet above is abridged — the full contract specifies required fields for every section.)*

The mental model is a control room: a manifold header, a citizen constellation, a kernel pulse, a Freedom Gate docket, a trace ribbon, and an operator action rail. To be accurate about status — the packet is the *contract* for that console, and the first fixture implementing it runs in fixture mode. It's the reviewable bridge between runtime artifacts and a console that doesn't exist yet, and the repo is careful to say so: the Observatory "must not scrape arbitrary runtime files or invent live state from a UI."

Underneath sit a handful of commitments that are easy to skim past and are actually the load-bearing part:

- **Private state is not ordinary debug data.** It doesn't get dumped to a log because someone raised the verbosity.
- **A projection is a review surface, not authority.** You cannot act on the world by editing a view of it. Read that twice if you've ever watched an ops tool quietly become a write path.
- **Lineage is append-only and auditable.** History accumulates; it doesn't get rewritten.
- **Ambiguous continuity preserves evidence rather than optimizing through doubt.** When the system isn't sure whether continuity held, it stops and keeps the evidence instead of pressing on and hoping.

That last one is a genuine design value and it costs throughput. Most systems, faced with an ambiguous restore, proceed — because proceeding is what systems are rewarded for. Choosing to halt and preserve is a decision you have to make on purpose.

## Persistence is not identity

The convenient shortcut is to call any restored state "the same agent." Process restarted, snapshot loaded, name reappeared, ship it. ADL refuses that inference deliberately.

Start with the distinction the runtime actually encodes. **Interruption** is execution halting with state preserved and continuity restorable — a crash, a pause, resource exhaustion. **Termination** is continuity irrecoverably broken: state loss beyond recovery, or explicit destruction. Those are different events with different consequences, and most systems have no way to tell them apart because they never represented continuity in the first place.

A resumption is valid only if lifetime time is not reset, monotonic order is preserved, trace linkage is restored, and experiential position remains coherent. The failure modes have names: silent reset, identity substitution, causal discontinuity. Wake and migration require continuity witnesses rather than a successful load.

The implemented sequence is **snapshot → rehydration → wake**, and wake is a gated step, not a status change: a rehydrated state activates only after required continuity and invariant checks pass. Governance and actuation state participate in checkpoint and restore specifically so a restart cannot silently revive revoked authority — the third failure above, handled as an architectural property rather than a code-review checklist item.

There's a runnable proof hook for this, which is the part I'd point a skeptic at:

```
adl identity continuity --out .adl/state/continuity_semantics_v1.json
```

That artifact is deliberately contract-shaped. It makes continuity and identity semantics inspectable as runtime behavior without pretending the rest of the temporal programme is finished.

And the definition the whole section turns on:

> Identity is the persistence of an agent across time under continuity-preserving transformations.

That is an invariance claim, and it is exactly the move relativity makes about physical law. Einstein didn't ask which observer's measurement of an interval was correct; he asked what quantity every observer computes the same way. Identity here is defined the same way — not as a label that travels along, but as whatever survives the transformations.

Which is why "the process came back with the same name" doesn't qualify. A name is not an invariant. It's a measurement taken in one frame, and it tells you nothing about whether the thing it labels is continuous with what came before.

This is also where the Principle earns its keep architecturally rather than rhetorically. If meaning is frame-relative, then identity cannot be a frame-relative property — otherwise every participant gets a different answer to "is this the same agent," and no reconciliation exists. The invariant has to be specified, and specifying it is what the continuity semantics contract does.

One check in this family is worth calling out as evidence the problem was taken seriously: **anti-equivocation**, detecting conflicting continuity claims — two incompatible signed successors for the same lineage position. If you've worked on distributed consensus you recognize the shape instantly. It's equivocation from Byzantine agreement, applied to identity rather than to blocks. Nobody writes that check without having thought about an adversary forking an agent's history.

The stronger surfaces are still ahead. **Worldlines** (continuity paths through time for an identity), **continuity witnesses** (artifacts explaining and supporting a major identity transition), and **receipts** (explanations of what transition occurred and why it was accepted) are planned, not shipped. The v0.92 identity-continuity plan sets the bar for a first Gödel-agent birthday deliberately higher than "the process came back": named identity architecture, continuity records, witnessed evidence. Article 10 states that acceptance boundary in full. It's active work and it is not done.

We hold that line because identity claims contaminate everything downstream. If continuity is ambiguous, commitments, responsibility, learning, and relationships are all ambiguous too — invisibly.

## Memory must be evidence, not mythology

The same discipline governs memory, which is why ADL's concept is **observational memory** — ObsMem — rather than just memory: evidence-adjusted retrieval and indexing over prior run evidence, scores, traces, and provenance. The underlying rule is that trace is execution truth, and artifacts carry the payload truth that trace references.

ObsMem and chronosense are complementary halves, and neither works alone. ObsMem provides persistence of *content*; chronosense provides persistence of *position*. ObsMem without chronosense is stored data with no temporal grounding — a pile of true things with no answer to "when was this true, and relative to what." Chronosense without memory is temporal form with nothing in it. Identity needs both.

The rule: a claim about the past must connect to something witnessed. Not model confidence — an artifact, event, observation, or trace. Compression and summarization are expected; what must survive compression is the *link* between summary and sources.

Three corollaries:

- Unknowns stay unknown. A gap in the record is a gap, not something a fluent narrator smooths over.
- Corrections don't disappear. The revision and what it revised both persist.
- Continuity comes from governed state transitions with retainable evidence, not from an endlessly expanding context window.

That last point is the practical one, and it follows directly from cognitive relativity. A bigger context window does not build a shared frame. It re-transmits a lossy snapshot of one, faster and more expensively, every single time.

## The world is the safety mechanism

The tempting architecture is to let an agent decide and then filter the output — governance as a bouncer.

CSM argues for something stronger: governance belongs in the structure of the world. Authority, time, resource limits, privacy, causality, and review shouldn't evaluate transitions after the fact. They should determine which transitions are *possible* and what evidence each leaves behind.

A filter can be talked around by a path its author didn't anticipate. A world in which unauthorized transitions have no representation cannot be.

## What's built, and what isn't

Runtime v3 is the governed-execution architecture: typed services for governance ingress, Freedom Gate mediation, bounded actuation against signed one-shot permits, and audit — with governance and actuation state participating in checkpoint and restore. As of v0.91.7 it is explicit opt-in; **Runtime v2 remains the default runtime**, and carries the memory, citizen state, and Theory of Mind surfaces this article and the next one draw on. A default switch is gated on the cutover proof and hasn't been authorized. Bounded CSM runs, runtime packets, continuity contracts, and operator-facing proof surfaces exist and run from a clone.

Not claimed: a complete distributed authority system, production message-bus transport, a universal policy language, a finished operator interface, or a fully inhabited runtime. Worldlines, continuity witnesses, receipts, and quintessence checkpoints are planned. The birthday is planned and evidence-gated.

That gap isn't a footnote. It's the line between a serious prototype and an unsupported claim, and an article about distinguishing proposals from proven outcomes had better draw it in its own prose.

## Where this goes

The Principle of Cognitive Relativity says a model can only work effectively inside a frame it shares with you — and that sharing a frame means sharing the laws, not the measurements. CSM is the attempt to build that shared physics as infrastructure rather than reassemble it by hand every session.

Today it supplies implemented contracts for governed runtime work, and a vocabulary for asking what a persistent agent would actually need before anyone calls persistence a life in a shared world.

Next: what happens when agents start forming hypotheses about *each other* — theory of mind as evidence-bound runtime state, and why a social model must never become a source of authority.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### The two papers

- Einstein, A. (1905). "On the Electrodynamics of Moving Bodies" (*Zur Elektrodynamik bewegter Körper*). *Annalen der Physik*, 322(10), 891–921. [doi:10.1002/andp.19053221004](https://onlinelibrary.wiley.com/doi/10.1002/andp.19053221004) — the relativity principle and the light postulate, both stated before §1. Older citations give the same paper as AdP 17, 891.
- Minkowski, H. (1909). "Space and Time" (*Raum und Zeit*). *Physikalische Zeitschrift*, 10, 104–111; also *Jahresbericht der Deutschen Mathematiker-Vereinigung*, 18, 75–88. Delivered 21 September 1908 in Cologne — where spacetime becomes one object.

### Go deeper

- [CSM explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/CSM.md) — the runtime world model
- [Chronosense and Identity](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.88/features/CHRONOSENSE_AND_IDENTITY.md) — the clock stack, continuity, and resumption semantics
- [Cognitive SpaceTime Manifold manuscript](https://github.com/agent-logic/agent-design-language/blob/main/demos/v0.90.5/publication/cognitive_spacetime_manifold/cognitive_spacetime_manifold_manuscript_packet.md) — the longer architectural treatment
- [Glossary](https://github.com/agent-logic/agent-design-language/blob/main/docs/GLOSSARY.md) — standing, identity, projection, and continuity terms
- [Runtime v2 bounded CSM run ADR](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0012-runtime-v2-bounded-csm-run.md)
- [Runtime v2 foundation prototype](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.90.1/features/RUNTIME_V2_FOUNDATION_PROTOTYPE.md)
- [Runtime v2 and birthday boundary](https://github.com/agent-logic/agent-design-language/blob/main/docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md)
- [v0.92 milestone](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92/README.md) — active development, including identity continuity
