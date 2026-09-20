# What's Next for ADL

### Thirteen articles, one architecture, and a list of things we haven't earned the right to claim yet.

---

This series has been built out of separations.

A proposal is not authority. A valid tool schema is not permission. A hypothesis is not evidence. A plan is not an outcome. Persistence is not identity. A security exercise is not certification. A social model is not another mind. A bid is not legitimacy.

Individually each sounds like a caveat. Together they're the architecture. Every one of them marks a place where agent systems routinely collapse two different facts into one, and every collapse is where the accountability goes missing.

ADL's next phase is connecting more of them inside one witnessed runtime without relaxing the evidence bar. Here's what that means concretely, and what it would take for us to claim it.

## What exists now

Implemented and runnable today: deterministic plan materialization, fork/join execution, bounded concurrency with canonical ready ordering, retries and failure controls, stable run artifacts and replay-friendly state, signing and verification and provenance, CLI inspection and trace tooling, provider abstraction and bounded remote execution.

On top of that: governed tools through UTS v1.0 and ACC v1.0, completed in the v0.90.5 milestone. Runtime v3 governed execution with Freedom Gate mediation, signed one-shot permits, and audit — available as explicit opt-in since v0.91.7, with Runtime v2 still the default until the cutover proof authorizes a switch. Bounded Gödel experiments preserving hypotheses, variants, evaluation, and adoption decisions. Runtime foundations for memory, continuity, citizen state, and theory of mind. Red/blue security work framing adversarial testing as a governed loop. A typed C-SDLC that we run our own development on.

That is not a completed artificial society. It's the engineering substrate from which stronger claims can eventually be tested — which is a less exciting sentence and a more accurate one.

## The identity challenge

The v0.92 milestone line is framed around the first true Gödel-agent birthday, and the acceptance boundary is deliberately harder than it needs to be.

Earlier work can already create test citizens, snapshots, wake records, and persistent processes. Those are prerequisites, not the event. The planned boundary turns on identity architecture, continuity across cycles, witnessed memory, and a citizen-facing receipt.

That work is live and unfinished. Release readiness is not claimed, work packages are dependency-gated and issue-owned, and the birthday belongs in a future evidence record rather than in anticipatory copy. When it happens there will be an artifact to inspect, and until there is, treat any claim otherwise — including from us — as unsupported.

## Learning without hidden mutation

The GHB loop and bounded experiment system give us a basis for learning. Fuller adaptive learning needs more.

The roadmap includes an Adaptive Learning DAG: a governed path from evaluation feedback to stateful adaptation and policy-controlled change in the reasoning graph. The hard part is preserving provenance and rollback as learning becomes structural rather than parametric.

Which evidence caused a graph change? Which identity adopted it? Can a reviewer compare prior and resulting behavior? Can policy prohibit a whole class of mutation? Can the system recover without losing the history of *why* it changed?

The goal isn't unbounded recursive self-improvement. It's an agent that improves through explicit, inspectable transitions — which is slower, and which is the only version that can be operated by anyone other than its author.

## Tools, communication, and the wider world

UTS and ACC are a current foundation for governed tools. Future work extends portability, invocation semantics, provider integration, and transport while holding the rule that description and authority stay separate.

There's also **ACIP**, the Agent Communication and Invocation Protocol, with schema and transport-readiness work including an optional WebSocket carrier proof. We built it because nothing existing covered what we needed: not merely messages between agents, but *invocation* — consultation, delegation, review, handoff and negotiation as distinct, traceable interaction types rather than one undifferentiated "call the other agent."

The principle underneath is worth stating because it's easy to get wrong: **transport is not social legitimacy.** An authenticated message still needs identity, policy, visibility, and runtime admission. A signed message proves something about provenance; it settles nothing about trust, standing, or interpretation.

### Where A2A fits

The obvious question, and the answer is that we've supported it from the beginning.

A2A was in scope alongside ACIP from the first agent-communication milestone. ADR 0017 — *Secure Local Agent Comms and A2A Boundary* — was accepted in May 2026 at v0.91.0; the A2A external-agent adapter boundary shipped as an implemented baseline at v0.91.1; and the runtime kernel carries A2A as a first-class protocol adapter kind today. ACIP was never an alternative to A2A. It's the layer A2A plugs into.

Agent2Agent is winning the interoperability layer and deserves to. It reached v1.0 in January, it's governed by the Linux Foundation rather than any single vendor, and it has north of 150 organizations behind it with integration across the major clouds. If you need agents built on different frameworks by different vendors to discover each other and collaborate, A2A is the answer and we're not proposing a competitor.

What it solves is cross-vendor task lifecycle — discovery, delegation, execution, status. What it doesn't express is governance, and we're not the only ones who've noticed; there's now published work specifically on the governance gaps that MCP, A2A and ACP leave open.

That gap is the same shape as the one in article 3, and the adapter is built around a single thesis: **A2A by itself does not grant execution.** An Agent Card is ingested as a *claim*, not authority. Advertised capabilities are translated into bounded ADL capability contracts rather than trusted as stated. External identity maps into ADL's trust classification. And every external-agent invocation goes through the explicit `agent.invoke(...)` boundary, so trace, replay, redaction and audit discipline survive the vendor crossing.

Which is the composition, same as with MCP: A2A carries interoperability, ADL types the invocation and binds it to runtime admission. Cross-vendor reach and governed invocation are different problems, and the industry is currently solving the first one well and the second one hardly at all.

As agents communicate across processes and eventually across governed worlds, distributed authority and continuity get substantially harder. We'd rather say that now than discover it in a press release.

## Security that remembers

Continuous adversarial verification points toward security systems that retain attack and defense evidence as durable knowledge. The next challenge is integration: bounded targets, safe fixtures, governed tool access, replayable traces, and a route for mitigations through ordinary review. Learned defenses need provenance and regression tests. Negative results need careful wording.

The intended result is not an autonomous attacker roaming infrastructure. It's a runtime that examines declared surfaces continuously while remaining subject to the same governance it's testing.

## The product surface

CodeFriend is the practical route for these ideas to reach engineering teams, and its beta is the first real test of whether the substrate holds up outside our own repository.

Its success should be judged by exactly one question: can a real operator run it against a bounded repository and understand both the findings *and their evidence*? Everything beyond that — enterprise pilots, portfolio views, the wider product family — waits on that answer.

## What remains genuinely open

Long-lived agents will consume compute, memory, attention, and bandwidth. They'll form beliefs about one another, inherit norms, negotiate, and encounter conflict.

We have foundations and proposals here, and a lot of honest uncertainty. Which allocation mechanisms are fair? How should identity-critical memory compete with immediate utility? How can reputation stay correctable? What rights or duties, if any, should attach to persistent artificial actors?

Engineering can make these questions explicit and give them somewhere to be argued about. It cannot answer them alone, and we're not going to pretend the architecture settles what it doesn't.

## The standard for the next claim

The roadmap is broad. The commitment underneath it is methodological, and it's the one thing we'd most want to be held to:

**Stronger claims require stronger evidence.**

A release should be tied to closed work, passing validation, reviewed artifacts, and witnessed runtime behavior. A birthday claim needs continuity evidence. A learning claim should identify the adopted change. A safety claim should state its threat model and its limits. A product claim should describe what a user can actually run today.

That standard makes this project easier to attack, which is the point. Inspect the repository. Question a boundary. Reproduce a focused proof. Find a place where our language outruns our evidence — and if you find one, we want to hear about it more than we want the sentence.

## Three things you can do this week

If any of this landed, here's the concrete path rather than a vague invitation.

**Run CodeFriend on a repository.** [codefriend.ai](https://codefriend.ai) — ideally point it at something you know well enough to catch it being wrong.

**Print a plan.** The repository is public and Apache-2.0, and one command shows you the core claim:

```
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- \
  adl/examples/v0-91-chatgpt-gemini-claude-triad-conversation.adl.yaml --print-plan
```

A multi-agent workflow resolved into an explicit plan object, printed rather than executed. From there, the [feature list](https://github.com/agent-logic/agent-design-language/blob/main/docs/planning/ADL_FEATURE_LIST.md) is the canonical statement of what's implemented versus planned — and the fastest way to check whether this series has been honest with you.

**Tell us where we're wrong.** Open an issue. If a boundary is stated imprecisely, a claim is stronger than its evidence, or an argument in this series doesn't survive contact with your production system, that's the feedback worth the most to us right now.

And if the problems in this series are ones you're currently living with — runs that won't converge, agents your security team won't approve, a merge queue nobody can account for, a codebase whose architecture nobody can explain — we'd like to talk to you.

## The shape of it

What comes next for ADL isn't one grand reveal. It's a sequence of governed transitions toward agents that can persist, learn, cooperate, and act while leaving accountability intact.

We think that's the interesting problem. We're fairly sure it's the necessary one.

**[github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language) · [agent-logic.ai](https://agent-logic.ai) · [codefriend.ai](https://codefriend.ai) — thanks for reading all thirteen.**

---

### Go deeper

- [v0.92 milestone](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92/README.md) — the active development milestone
- [v0.92 issue wave](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml)
- [Identity continuity and birthday plan](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md)
- [Feature list](https://github.com/agent-logic/agent-design-language/blob/main/docs/planning/ADL_FEATURE_LIST.md) — canonical capability and roadmap truth
- [Runtime v2 and birthday boundary](https://github.com/agent-logic/agent-design-language/blob/main/docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md)
- [ADR 0017 — secure local agent comms and the A2A boundary](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0017-secure-local-agent-comms-and-a2a-boundary.md)
- [A2A external agent adapter](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md) — Agent Cards as claims, not authority
