# The Freedom Gate

### The most consequential word in an agent system isn't "intelligence." It's "may."

---

Somebody in your organization is going to be asked to approve letting agents touch production. Maybe they already have been.

That person needs to answer a specific set of questions, and none of them are about model quality. What exactly can this thing do? Who authorized it? Can that authority be passed along, and if so how far? What happens when it's revoked? What happens on restart? When it refuses something, is there a record, and can a human overrule it through an accountable path?

"We wrote a careful system prompt" is not an answer to any of those. Neither is "the model is well-aligned." Both are statements about *disposition*, and the question being asked is about *control*.

ADL's answer is a component called the **Freedom Gate**: the runtime boundary between a proposed action and an authorized effect.

## A valid proposal is not a valid permit

Most agent architectures treat tool use as a continuation of text generation. The model emits a function name and arguments; the application executes them. Safety then rests on prompt wording, scattered tool-specific checks, and whatever context the application remembered to thread through.

The Freedom Gate starts somewhere stricter:

> A valid proposal is not a valid permit.

The proposal says what an actor wants to do. Authority says why that actor is allowed to. Policy supplies the constraints. Resource state determines whether it can be admitted *now*. All four have to agree before anything executes.

That separation buys a clean trust boundary, and the trust boundary is the point. Models stay untrusted generators — which is the correct posture for any component whose output distribution you can't fully characterize. The gate is ordinary software with explicit validation and fail-closed behavior. It's reviewable in the way a security team means when they say reviewable.

## What the implemented gate actually checks

Runtime v3 implements a focused Freedom Gate contract in Rust. A request binds to a principal, an action, a resource, a unit ceiling, a policy identity, and an expiry. Commitments and authority grants are signed with trusted Ed25519 keys.

Four properties are worth understanding, because they're the ones that come up in review.

**Delegation attenuates.** A child grant references its signed parent and cannot silently acquire more units or greater delegation depth. Authority can be passed down; it cannot grow on the way. If you've ever watched a service account accumulate permissions through three layers of "it needed access to do its job," you know precisely which failure this closes.

**Stale and forged authority is refused.** Revoked, expired, forged, escalated, or replayed authority doesn't pass. Not "gets flagged" — refused.

**Reservation and consumption are atomic.** The gate checks trusted time and resource availability, then reserves the required units and consumes the request identity *in the same critical section* that emits the permit. Two concurrent attempts cannot both spend the same authority or the same reservation. That's a concurrency property, and it's the difference between a permission system and a permission system that works under load.

**The permit is one-shot.** What the gate issues is not a general credential. It authorizes one bounded actuation. The execution layer verifies its signature and consumes it once. Authorization isn't a mode the system enters and remains in — it's a token attached to a single act.

## The constitutional layer: trajectories, not just actions

Everything above operates on individual requests, and individual requests are not where the hardest problem lives.

Consider a sequence in which every single step is permissible. Read a config file. Read a second one. Summarize them. Write the summary to a location the agent is allowed to write to. Nothing in that list trips an action-level check, and the sequence exfiltrated your credentials.

This is the structural weakness of per-action authorization, and it isn't exotic — it's the shape of most real agent security incidents. The dangerous thing is rarely one obviously-bad call. It's a path assembled from individually-defensible steps, where the harm is a property of the *composition* rather than of any element.

So the Freedom Gate is positioned as ADL's **constitutional layer**: it validates whole trajectories, not just individual actions.

The distinction is the same one that separates a bouncer from a legal system. A bouncer evaluates each person at the door. A constitution constrains what the institution may do over time, including sequences nobody anticipated when the rules were written. You need both, but only one of them can reason about a pattern.

Concretely, that's what makes the audit chain load-bearing rather than decorative. A gate that retains refusals, appeals, consumed request identities, and prior audit relationships has the material to evaluate a path. A gate that only sees the current request has no way to know it's the eleventh step of something.

## Refusal is a first-class result

Most systems treat denial as an error string. The Freedom Gate treats refusal as governance evidence.

A refusal retains its reason, the request and policy identities, its relationship to prior audit records, and a canonical evidence hash. An operator decision can be signed independently. An appeal preserves both the original refusal and the later decision, rather than rewriting history so the final answer looks like it was inevitable.

This is valuable even when the gate is right — arguably *especially* then. A reviewer needs to know whether a request failed because authority was missing, policy was stale, a resource was exhausted, a commitment was revoked, or a replay was detected. Those are five operationally different events with five different responses, and a system that collapses them into `403` has thrown away the only information that would have told you which one you're looking at.

Refusal evidence also supports correction. Policy can change through an accountable path. A grant can be reissued. A resource can free up. The gate can admit a later request without pretending the earlier one was valid all along.

## The actuation boundary

Once a permit is issued, the Adaptive Execution Engine invokes an injected actuation shell — which might wrap a provider, a tool, or an external service. The runtime kernel deliberately doesn't reimplement every SDK.

The engine verifies the permit before calling the shell. Result bytes are bounded and hashed. Successful outcomes become canonical audit events. Tool errors and oversized results are **quarantined** rather than released as though they were normal trusted output.

That last detail matters more than it sounds. An unexpected payload from a tool is exactly the vector you'd use to attack an agent downstream of a call, and "the tool returned something weird so we passed it along" is how that attack succeeds.

The separation here is also worth stating: the gate decides whether an action may be *attempted*. It makes no promise that the tool is correct, the service is available, or the result is desirable. Authorization and outcome are different facts, and the audit trail keeps both.

## Continuity and replay

Governance state cannot evaporate when the process restarts. Otherwise a restart restores revoked authority, forgets consumed permits, or breaks the audit chain — and nothing in the logs says it happened.

So the Freedom Gate and the actuation engine participate in checkpoint and restore. Snapshots retain resource balances, revocations, consumed request identities, refusal and appeal evidence, permits, results, and audit continuity. Restore validates schema and evidence integrity before execution continues.

This is where runtime continuity and security turn out to be the same problem. Persistent identity without persistent authority state is unsafe. Persistent authority state without integrity checks is untrustworthy. You need both or neither is worth much.

## Why "freedom"?

The name sounds backwards. A gate constrains action; how is that freedom?

Because unconstrained output isn't agency, it's noise with consequences. Meaningful agency requires being able to form proposals, hold delegated authority, act within a known envelope, receive a reasoned refusal, and participate in accountable review. Constraints are what make the scope of action legible — to the agent, and to everyone it affects.

The alternative to a gate isn't freedom. It's opaque power, distributed across prompts, application code, credentials, and accidents, with no one able to say who holds what.

## The honest boundary

The current implementation proves a focused kernel contract, with tests covering allowed mediation, forged and missing authority, stale policy, attenuating delegation, resource exhaustion, revocation, replay, appeal, quarantine, and checkpoint recovery.

It does not prove a universal policy language, formally verified governance, distributed authority, production provider integration, or a complete operator interface. And cryptographic signatures establish integrity and provenance within a trust model — they do not make a bad policy into a good one. A perfectly signed authorization to do something stupid remains an authorization to do something stupid.

That boundary is why the architecture is useful rather than despite it. The Freedom Gate identifies the exact place where intelligence becomes power, and gives engineers concrete contracts for deciding whether that transition should occur.

## Where this goes

Next: what happens when you point this entire apparatus at your own software delivery — the Cognitive SDLC, and what changes when coding agents can produce more work than a maintainer can safely absorb.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [Freedom Gate feature doc](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.86/features/FREEDOM_GATE.md)
- [Runtime v3 governed execution architecture](https://github.com/agent-logic/agent-design-language/blob/main/docs/architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md)
- [ADR 0015](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0015-governed-tools-execution-authority-architecture.md) — execution authority architecture
- [ADR 0021](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0021-adl-capability-contract-runtime-authority-boundary.md) — the ACC runtime authority boundary
- [`adl-runtime-kernel/src/governance.rs`](https://github.com/agent-logic/agent-design-language/blob/main/adl-runtime-kernel/src/governance.rs) — the implementation
