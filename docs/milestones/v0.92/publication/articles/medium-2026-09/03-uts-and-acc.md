# Governed Tools: UTS and ACC

### Tool calling is a solved problem. Your agent can now reach ten thousand servers and invoke any of them. Nothing in that stack establishes whether it should have.

---

Two years ago, connecting a model to a tool meant picking a framework and accepting its opinions. Every stack shipped its own calling convention. Integration was the bottleneck.

That bottleneck is gone. MCP became the de facto standard for how models discover and call external tools, it's supported across Claude Code, Cursor, GitHub Copilot and OpenAI's tooling, and community registries now list over ten thousand public servers. Whatever you think of the specifics, the interoperability problem has largely been won.

Which is precisely why the next problem is now urgent rather than theoretical.

When calling a tool was hard, the friction was doing a lot of unearned safety work. Every integration was bespoke, so every integration got looked at. Now an agent can reach an enormous surface of capabilities through a uniform interface, and the number of humans who examined any particular one on the way in is frequently zero.

A tool definition tells a model three things: what the tool is called, what it does, and what arguments it takes. That's enough to generate a call. It is not close to enough to govern one.

Because none of these are answered:

Will this change external state? Can the operation be safely replayed? Is it idempotent? What data can leave the boundary? Which credentials does it need? Who authorized *this actor* to run it *right now*? What evidence has to survive? Who's allowed to see the result?

ADL splits those into two connected contracts. **Universal Tool Schema** answers "what is this tool?" **ADL Capability Contract** answers "who may use it, under what authority, with what visibility, and with what evidence?"

## UTS: describing the capability honestly

UTS is a portable, JSON-compatible tool description, and its v1.0 baseline goes well past parameters and return types. It carries side-effect class, determinism, replay safety, idempotence, resource boundaries, authentication, data sensitivity, exfiltration risk, execution environment, an error model, and constrained extension points.

Here's a release publisher, with input and output schemas elided for length:

```json
{
  "schema_version": "uts.v1",
  "name": "release.publish",
  "version": "2.1.0",
  "description": "Publish a tagged release for a single repository.",
  "side_effect_class": "external_write",
  "determinism": "nondeterministic",
  "replay_safety": "not_replay_safe",
  "idempotence": "not_idempotent",
  "resources": [
    { "resource_type": "repository", "scope": "single-repo" }
  ],
  "authentication": { "mode": "user_delegated", "required": true },
  "data_sensitivity": "internal",
  "exfiltration_risk": "low",
  "execution_environment": {
    "kind": "external_service",
    "isolation": "external release API, no local mutation"
  },
  "errors": [
    { "code": "tag_exists", "message": "Release tag already present.", "retryable": false }
  ]
}
```

Every one of those enums is closed. `side_effect_class` admits eight values — `read`, `local_write`, `external_read`, `external_write`, `process`, `network`, `destructive`, `exfiltration`. `replay_safety` admits three: `replay_safe`, `replay_requires_approval`, `not_replay_safe`. That's not decorative typing; it's what lets a runtime reason *before* execution rather than parsing free text at the worst possible moment.

A read-only deterministic lookup and a command that deletes production data should not arrive at the executor wearing the same clothes. And `not_replay_safe` is doing real work: it tells an adaptive execution layer that a retry is a second real-world event, not a second attempt at the first one. Get that wrong and your recovery logic publishes the release twice.

Two details worth noticing. `resources` requires **at least one** resource boundary — a tool with no declared scope fails validation rather than defaulting to unlimited. And extension keys must be `x-` prefixed, must be lowercase tokens, and are explicitly forbidden from smuggling authority or approval semantics into the schema layer. The schema is not allowed to grow a permission system by accident.

Note how much of that is invisible in a conventional tool definition. Most tool schemas in circulation describe the *interface*. UTS describes the interface plus the **hazard profile**. That difference is what lets a runtime make an automated decision that isn't just "the JSON parsed."

UTS is also transport-independent, which is the procurement argument. The same semantic description sits behind different providers and different invocation protocols, so your governance posture isn't trapped inside one vendor's function-call metadata. When a provider renames a model, changes its calling convention, or you move workloads for cost reasons, your tool hazard model doesn't get rebuilt from scratch.

And then the deliberate omission: **UTS grants no authority whatsoever.**

## ACC: the governed relationship

Authority lives in a separate contract, and it's a substantially bigger object — sixteen top-level members covering the actor, the grant, role standing, the delegation chain, the capability, policy checks, confirmation, Freedom Gate posture, execution, trace and replay, privacy and redaction, failure policy, and the decision itself.

Take the release publisher. UTS described its inputs, outputs, authentication, side effects, and replay danger. ACC describes the *relationship* — abridged here to the load-bearing parts:

```json
{
  "schema_version": "acc.v1",
  "contract_id": "acc.release.publish.4417",
  "tool": {
    "tool_name": "release.publish",
    "tool_version": "2.1.0",
    "adapter_id": "github.public"
  },
  "actor": {
    "actor_id": "runtime.release_agent",
    "actor_kind": "agent",
    "authenticated": true,
    "authority_evidence": [
      { "evidence_id": "grant.release.operator.primary",
        "kind": "operator_grant",
        "issuer": "operator.primary" }
    ]
  },
  "authority_grant": {
    "grant_id": "grant.release.operator.primary",
    "grantor_actor_id": "operator.primary",
    "grantee_actor_id": "runtime.release_agent",
    "capability_id": "release.publish",
    "scope": "repo:releases.write",
    "status": "active",
    "revocation_reason": null
  },
  "role_standing": { "role": "release_agent", "standing": "service_actor" },
  "delegation_chain": [],
  "capability": {
    "capability_id": "release.publish",
    "side_effect_class": "external_write",
    "resource_type": "repository",
    "resource_scope": "repo"
  },
  "policy_checks": [
    { "policy_id": "policy.release.publication",
      "decision": "allowed",
      "evidence_ref": "grant.release.operator.primary" }
  ],
  "confirmation": { "required": true, "confirmed_by_actor_id": null, "confirmation_id": null },
  "freedom_gate": { "required": true, "decision": "pending", "event_id": null },
  "execution": { "environment": "production", "dry_run": false, "approved_for_execution": false }
}
```

Read the last three members together, because they're the whole argument. `confirmation.required` is true and `confirmed_by_actor_id` is null. `freedom_gate.decision` is pending. And therefore `execution.approved_for_execution` is **false**.

The tool call is well-formed. The actor is authenticated. The grant is active and in scope. And the contract still says no, because a human confirmation hasn't happened and the gate hasn't ruled. Those are separate facts held in separate fields, and none of them can be satisfied by the others being fine.

Note also `authority_grant.status` and `revocation_reason` sitting right next to each other. Revocation isn't a deletion — it's a recorded state with a reason attached, which is what lets a reviewer later distinguish "never had permission" from "had it and lost it, here's why."

So a call can be **perfectly valid under UTS and still fail under ACC**. Wrong repository. Expired delegation. Missing confirmation. Policy forbids the target.

That failure is the product.

## Three deliberately negative claims

The design rests on three statements about what schema validation does *not* buy you:

- UTS validity is not authority.
- UTS validity is not execution permission.
- UTS validity is not replay permission.

Schema validation establishes that a proposal is well-formed. It cannot establish that the proposal is legitimate in the current context, and those are different kinds of fact.

This matters more as interoperability improves, not less. A common schema makes it easier for more models to call more tools — that's the point, and it's good. But the easier invocation becomes, the more dangerous it is to let permission ride along inside the fact that an invocation parses. Ubiquity and permissiveness are not the same property, and the industry has a habit of conflating them.

## How this relates to MCP

The honest answer: **they solve different problems, and they compose.**

MCP has an authorization framework, and it's a real one. The 2025-03-26 revision introduced OAuth 2.1-based authorization; a later revision separated auth concerns from server logic using Protected Resource Metadata; the 2026-07-28 specification hardened it further and aligned it more closely with OAuth and OpenID Connect deployments. Anyone claiming MCP is unauthenticated hasn't read it recently.

But notice what OAuth-style authorization answers. It answers: *is this client permitted to reach this server, and with what scopes?* That's connection-level access control, and it's the right shape for the problem it solves.

ACC answers a different question: *is this particular actor, acting under authority delegated by this particular grantor, within this scope, permitted to perform this specific action right now — and what evidence survives the decision?*

Those questions come apart precisely where it matters most. An agent can hold an entirely valid OAuth token for a server, with the right scope, and still be the wrong actor to publish that release — because the delegation it's operating under covered a different repository, or expired, or required a human confirmation that never happened, or the action needed a depth-limited grant it doesn't hold. The token says the connection is legitimate. It says nothing about whether *this act, by this agent, on behalf of this person, at this moment* is.

So the composition we'd argue for is straightforward:

- **MCP** carries discovery and transport, and authorizes the connection.
- **UTS** describes the tool's hazard profile in a transport-independent way — a tool exposed over MCP can carry a UTS description.
- **ACC** governs actor-level capability authority, delegation, confirmation, visibility, and evidence.
- **The Freedom Gate** admits or refuses the specific request.

None of that requires replacing MCP, and we're not proposing to. It requires noticing that "the client is authorized to call this server" and "this agent may take this action" are two sentences, and that shipping only the first one leaves an enormous amount of the risk unaddressed.

## The division of labour

The full chain, with no layer asked to solve the whole safety problem:

- The model **proposes**.
- UTS **describes**.
- ACC **constrains** authority and visibility.
- The Freedom Gate **admits or refuses**.
- The actuation boundary **executes**.
- Audit records **preserve what happened**.

A model stays an untrusted generator, which is the correct security posture for a component whose output distribution you cannot fully characterize. The governing components are ordinary software with explicit validation and fail-closed behavior — the kind of thing a security reviewer can actually read.

That last point is worth making plainly, because it's the practical reason to care. When someone in your organization is asked to sign off on letting agents touch production systems, they need artifacts: what the tool can do, who authorized it, how delegation attenuates, what happens on failure, what evidence persists. "We wrote a careful system prompt" is not a control. A typed capability contract with a signed authority chain is.

## Versions, stated precisely

UTS v1 — formally versioned by its v1.0 schema — and ACC v1.0 are the implemented baselines. Both have normative documents and machine-readable schema or runtime surfaces. They shipped in the v0.90.5 Governed Tools milestone.

UTS v1.1 and ACC v1.1 are tracked *additive proposals*. They refine invocation metadata, observability, version negotiation, authority, and visibility. They are not current wire behavior.

That distinction is more than release hygiene. Agents and runtimes need to know which contract they're interpreting, and quietly treating a proposed field as enforced policy would be exactly the class of hidden assumption these contracts exist to eliminate.

## What contracts can't do

They can improve interoperability, inspectability, replay posture, and governance. They can make refusal deterministic and evidence richer. They can surface a dangerous mismatch before a call ever reaches a tool.

They cannot guarantee that a tool is bug-free, that a model's intent is correct, that a policy is wise, or that an authorized action will turn out well. They don't remove the need for security review or human judgment.

Their value is narrower and entirely practical: they give agent platforms a precise place to represent facts that minimal function calling leaves implicit. In systems where models touch files, infrastructure, money, messages, and identities, making those facts explicit is a substantial and unglamorous safety improvement.

## Where this goes

Tool calling got standardized. Tool *governance* hasn't, and the gap between those two facts is where most of the operational risk in agent systems currently lives.

Next: the Freedom Gate — the runtime component that turns all of this into an admit-or-refuse decision, with signed one-shot permits, attenuating delegation, and refusal as first-class evidence.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai).**

---

### Go deeper

- [UTS + ACC explainer](https://github.com/agent-logic/agent-design-language/blob/main/docs/explainers/UTS_AND_ACC.md)
- [UTS v1.0 schema](https://github.com/agent-logic/agent-design-language/blob/main/docs/specs/uts/UTS_V1.0_SCHEMA.md) — the normative field set
- [ACC v1.0 spec](https://github.com/agent-logic/agent-design-language/blob/main/docs/specs/acc/ACC_V1.0_SPEC.md) — the authority contract
- [ADR 0020](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0020-universal-tool-schema-portable-tool-description-standard.md) — why UTS is a portable standard
- [Governed Tools v1.0 release evidence](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.90.5/RELEASE_EVIDENCE_v0.90.5.md)
