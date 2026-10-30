# Cognitive SpaceTime Manifold

CSM means Cognitive SpaceTime Manifold. It is ADL's model for a governed
runtime world where agents, state, time, causality, trace, memory, and identity
continuity are first-class.

The short version:

> CSM is the world model for long-lived agents.

Runtime v2 remains the default runtime and the architecture line that turns
that model into reviewable packets, contracts, demos, and proof surfaces.
Runtime v3 provides governed execution as explicit opt-in as of v0.91.7; a
default switch remains gated on the cutover proof.

## The Principle Of Cognitive Relativity

CSM is derived from one principle.

> There is no privileged cognitive frame. The meaning of an event, a claim, a
> commitment, or an instruction is relative to the frame in which it is
> evaluated. Participants may disagree about measurements — what is now, what
> is recent, what is settled — and still cooperate, but only where they share
> the laws of the world they are reasoning about: its causality, its authority,
> its evidence, and its continuity.

The name is taken seriously rather than decoratively. Einstein's principle of
relativity is not the claim that everything is subjective; it is the claim that
the laws of physics are identical in every inertial frame. Observers disagree
about measurements, including simultaneity, while agreeing about the physics.
That shared structure is what makes their disagreement reconcilable: a
transformation carries one frame's measurements into the other's.

The same structure is what a human and a model lack by default. They do not
occupy different frames of a shared world; they occupy no shared world at all.
Different clocks, different histories, no agreed ordering, and no transformation
between them. Two participants in that position do not disagree in any useful
sense. They produce confident, mutually incompatible answers with no procedure
for reconciliation.

The operational consequence:

> A language model can only work effectively when it shares the manifold with
> humans.

This reframes ordinary model-human misalignment. When a model does an unhelpful
thing, the cause is frequently neither capability nor values but a missing
invariant: the two participants are computing over different worlds, and
neither can see the discrepancy, because seeing it would require the shared
structure whose absence is the problem.

Context engineering is the manual workaround. Pasting history into a window
transmits content but not position, and a frame reassembled from text each
session is a frame that silently differs each session.

### What CSM Makes Invariant

CSM exists to supply the shared laws rather than shared measurements:

- **causality** — trace is execution truth; artifacts carry the payload truth
  that trace references
- **authority** — standing, grants, delegation, and refusal are runtime state
  rather than prose
- **evidence** — claims connect to witnessed artifacts, and claim strength is
  declared
- **continuity** — identity is the persistence of an agent across time under
  continuity-preserving transformations

Measurements remain frame-relative by design. Chronosense supplies each
participant a position and an explicit translation between the internal frame
(UTC, monotonic, lifetime) and the external frame (human-local, contextual).
"Last Thursday, before the incident" and a monotonic order value are the same
event in different frames; neither is privileged, and the mapping between them
is what must not be lost.

### Boundary

Cognitive relativity is a design principle, not an implemented subsystem. It
explains why chronosense, ObsMem, trace, continuity semantics, and Observatory
projections belong to one architecture rather than a pile of features. It does
not by itself claim that the fully inhabited manifold exists, and "manifold"
here describes a world with coordinates, worldlines, located events, and
frame transformations rather than a differential-geometric structure.

### Sources

The principle is Einstein's; the geometry the name borrows is Minkowski's.

- Einstein, A. (1905). "On the Electrodynamics of Moving Bodies"
  (*Zur Elektrodynamik bewegter Körper*). *Annalen der Physik*, 322(10),
  891-921. `doi:10.1002/andp.19053221004`. The relativity principle and the
  light postulate are both stated before section 1. Older citations give the
  same paper as AdP 17, 891.
- Minkowski, H. (1909). "Space and Time" (*Raum und Zeit*).
  *Physikalische Zeitschrift*, 10, 104-111; also *Jahresbericht der Deutschen
  Mathematiker-Vereinigung*, 18, 75-88. Delivered 21 September 1908 in Cologne.
  Space and time become one four-dimensional object here, not in the 1905
  paper.

## What CSM Provides

CSM gives ADL a way to talk about agents as inhabitants of a persistent
manifold rather than temporary prompt executions.

It includes:

- time and ordering through chronosense
- state and memory through ObsMem and runtime packets
- causality through traces and replay surfaces
- identity continuity through snapshots, rehydration, wake, and worldlines
- governance through polis, policy, Freedom Gate, and security boundaries
- operator visibility through Observatory-style projections

## CSM And Polis

CSM is the governed runtime world. The polis is the social, security, policy,
and economic layer that civilizes that world.

The repo's Runtime v2 planning puts it simply:

> The manifold is the world. The polis is the society that inhabits it.

That means agents are not just tasks. They can become citizens with standing,
duties, rights, commitments, memory, and continuity evidence.

## Why It Matters

Without CSM, agent systems remain mostly job-scoped. They can perform tasks,
but they do not have a durable world in which identity, memory, policy,
security, and causality can accumulate.

With CSM, ADL can ask stronger questions:

- What state did the agent inherit?
- What episode happened?
- What policy applied?
- What changed?
- What can be replayed?
- Can this agent sleep, wake, recover, or continue?
- What should the operator see?

## Current Boundary

Runtime v2 and CSM proof surfaces exist, but the fully inhabited runtime is
still being completed across later milestones. CSM should be read as ADL's
runtime-world architecture, not as a claim that every planned citizen,
identity, migration, or birthday surface is already finished.

## Deeper References

- [Runtime v2 foundation demo](../milestones/v0.90.1/features/RUNTIME_V2_FOUNDATION_PROTOTYPE.md)
- [Runtime v2 bounded CSM run ADR](../adr/0012-runtime-v2-bounded-csm-run.md)
- [Runtime v2 and birthday boundary](../planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md)
- [Glossary: CSM](../GLOSSARY.md)
