# The Cognitive SDLC

### Coding agents made patches cheap. They made "done" ambiguous. That's a worse trade than it looks.

---

Here's a question that used to be trivial and no longer is: **has this change landed?**

Six months ago, "landed" meant one thing, because one person wrote the change and watched it through. Now there's an issue that may or may not describe what was actually built, a branch that may or may not match the issue, a pull request that may or may not contain the whole change, CI results that may or may not have covered the risky part, and a merge commit that squashed all of it into a single line of history.

Ask five people and a dashboard whether the work is done, and you can get six answers.

This is the coordination crisis that arrives about three months after coding agents start working. Generation stopped being the bottleneck. Nothing else did — and the parts that didn't scale are now the whole problem.

## The maintainer is the constraint

Parallel agents can produce more change than any maintainer can safely absorb. That single sentence reorganizes everything about how a team works.

The scarce resource is no longer typing. It's **convergence**: preventing duplicate effort, detecting drift, preserving review independence, and making "done" mean the same thing across issue, branch, pull request, checks, and repository state.

Three failures show up reliably:

**Intent gets lost between issue and outcome.** The issue asked for one thing. The agent built something adjacent. Both are defensible. Nothing anywhere records the divergence, so it surfaces two sprints later as a surprise.

**Plan and outcome share one mutable narrative.** This is the corrosive one. If the record of what you intended and the record of what happened are the same document, a failed experiment can be quietly rewritten to look like the original plan. Not through malice — through ordinary tidying. And the learning is destroyed.

**Review independence erodes.** When the same system implements and reviews, review becomes a formality with good grammar.

Git and pull requests are excellent substrates, and none of this is an argument against them. They preserve changes, branches, conversations, checks, and merge history. They were simply never asked to carry semantic state about intent, plan, validation, and outcome as *separate, non-rewritable* facts — because until recently a human head held all of that, and there was one head per change.

## C-SDLC: one change as a governed transition

The Cognitive Software Development Lifecycle represents a single bounded change as a cognitive state transition, recorded in a canonical sequence of typed lifecycle records — cards.

The sequence runs `SIP → STP → SPP → SRP → SOR`:

- **SIP**, Structured Issue Prompt — the problem, context, scope, acceptance boundary, dependencies.
- **STP**, Structured Task Prompt — the chosen transformation, changed surfaces, invariants, expected behavior, proof shape, and explicit non-goals.
- **SPP**, Structured Plan Prompt — sequencing, dependencies, validation, review handoff, stop conditions, risks, worktree constraints.
- **SRP**, Structured Review Prompt — review instructions *and* review results: findings, dispositions, residual risks, recommended outcome.
- **SOR**, Structured Outcome Record — what actually changed, what validation actually ran, what review actually happened, merge state, unresolved follow-ups, final issue truth.

A bound worktree gives the change a concrete execution context, and tracked implementation happens there while the root checkout stays clean. Typed tools validate and advance lifecycle state.

The separation between STP and SOR is the one that earns its keep. **What was intended and what actually merged are different records, and the second cannot silently overwrite the first.** That's the whole anti-rewriting property, and it's why a failed approach stays legible as a failed approach instead of disappearing into a tidy history.

Note what's in the STP that most planning artifacts omit: *non-goals*. Writing down what a change deliberately won't do is how scope creep becomes visible while it's still cheap.

## Software delivery as a small polis

C-SDLC treats delivery as a governed society of human and machine participants, each with scoped standing and responsibility.

One agent implements a bounded issue. Another reviews the exact changed surface — and only that surface, which is what keeps the review honest. CI supplies integration evidence. A human retains merge and governance authority. Worktrees, branches, and issue records make ownership visible rather than assumed.

The point isn't ceremony. It's that governance is the structure allowing multiple capable actors to cooperate *without erasing responsibility*. Remove it and you don't get speed, you get a codebase nobody can account for.

## What it costs, honestly

Five typed records per change is more ceremony than a commit message, and it would be absurd overhead on a two-person project shipping a prototype.

It stops being absurd at the point where the number of actors exceeds the number of people who can hold the whole picture in their head — which, with agents in the loop, arrives much earlier than it used to. The records aren't there to slow anyone down. They're there because "what was intended" and "what merged" stop being the same question once the thing doing the work isn't the thing accountable for it.

We run our own development this way, which is the only honest evidence we can offer. It's a typed C-SDLC v2 lifecycle, it's in the public repository, and the article series you're reading was itself produced through it.

## The premise underneath

The Cognitive SDLC rests on one idea: software engineering improves when important reasoning becomes inspectable, without pretending every thought can be formalized.

Agents explore, implement, and review. Typed lifecycle state preserves the boundaries. Evidence connects findings to source. Humans exercise authority with an accurate view of what happened.

The goal isn't development without people. It's development where faster machine cognition strengthens engineering accountability instead of dissolving it — which is the live question for every team currently discovering that their agents can write more code than they can responsibly merge.

## Where this goes

Next: the product this lifecycle produced — CodeFriend, and the argument that the maintenance work nobody schedules is exactly the work worth automating. Its beta opens tomorrow; [put your name down](https://codefriend.ai) if repository maintenance is eating your quarter.

**The code is at [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language). We're at [agent-logic.ai](https://agent-logic.ai) — if the coordination problem above is one you're living with, we'd like to hear how you're handling it.**

---

### Go deeper

- [C-SDLC architecture](https://github.com/agent-logic/agent-design-language/blob/main/docs/cognitive-sdlc/architecture.md)
- [Default workflow](https://github.com/agent-logic/agent-design-language/blob/main/docs/default_workflow.md) — the lifecycle in practice
- [Glossary](https://github.com/agent-logic/agent-design-language/blob/main/docs/GLOSSARY.md) — the card lifecycle and workflow terms
- [C-SDLC v3 simplification plan](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md)
