# CodeFriend

### Because your code needs a friend.

> **[HERO IMAGE: AL-002 CodeFriend datasheet]**
>
> *The continuous engineering loop is the full product. In Beta 1, CodeFriend runs observe, analyze, plan, validate, review, document and learn across your repository — and at the implement stage it writes tests, not application code. Deliberately: see "What it is, and what it deliberately isn't" below.*

**Every AI coding tool offers to do the interesting part. We built one that does the other part.**

---

Think about what actually ate your last quarter.

Not the architecture work. Not the hard tradeoff you spent two days arguing about and got right. The rest of it: the README that's been wrong since the service was split, the test suite with a coverage number nobody trusts, the dependency upgrade everyone deferred, the ADR that describes a decision you reversed, the changelog written from memory at release time, the diagram that was accurate in March.

None of that is why you became an engineer. All of it is real work, it degrades continuously, and it's the first thing dropped when a deadline appears — which means it's permanently behind.

Now notice what the current wave of AI development tools proposes to do about it. They write the features. They generate the new capability, the interesting function, the greenfield module. They automate the creative part — the part most engineers actively want to do — and leave you the maintenance.

**CodeFriend inverts that.** It's an AI engineering maintenance and intelligence platform, and the promise is one sentence:

> CodeFriend removes engineering maintenance so developers can focus on creativity.

There's a corollary that matters more to whoever approves the time: it makes **technical debt cheap to pay**. Not by rewriting your code — by collapsing the cost of finding it, pricing it, and servicing it without breaking something that works. Debt doesn't go unpaid because teams can't see it. It goes unpaid because paying is expensive. That's the number this changes.

The beta is open.

## What it is, and what it deliberately isn't

CodeFriend is an AI engineering teammate. Its job is to keep your engineering environment healthy and intelligible over time.

It is not an AI software engineer. Not a developer replacement. Not an autocomplete product, not an IDE chatbot, not an autonomous feature-development system, and not something that quietly modifies your repository while you're at lunch.

You keep architecture, product and feature design, user experience, the difficult tradeoffs, innovation, and strategic technical direction.

It takes test generation and maintenance, code and security and architecture and dependency reviews, documentation upkeep, diagrams, dependency hygiene, technical-debt visibility, architectural drift detection, ADR maintenance, release notes, and the repository-health work nobody schedules.

That division is the whole product thesis, and it's why the name isn't a joke. We were not willing to build something that threatens the job of the person using it — that isn't friendly, and a tool you're quietly competing with is not a tool you'll ever trust with your codebase. A friend takes the work you resent and leaves you the work you're good at.

It also isn't only about relief. Taking tedious work off a team is pleasant; it isn't automatically an improvement, since you can automate a bad review and get bad reviews faster. What makes this worth doing is that the tedious work in question is precisely the work that **sets the quality floor** — coverage, boundaries, documentation, review consistency — and the reason it's permanently behind is that humans do it last, under deadline, at the end of a sprint.

So you're not buying fewer chores. You're buying a standard your team already agrees with and cannot consistently hit, applied continuously rather than when someone finds a spare afternoon.

## Making technical debt cheap to pay

Technical debt is the one item on an engineering leader's list that behaves like actual debt. It compounds. It charges interest — every future change in the affected area costs more than it should, forever, until someone pays the principal. And the interest is invisible on any single transaction, obvious only in aggregate.

Most discussions of it get one thing wrong. Teams don't carry debt because they can't see it. Ask any senior engineer where the bodies are buried and you'll get a confident, largely accurate answer in ninety seconds.

They carry it because **paying it down is expensive in exactly the resource they have least of.**

Servicing debt costs senior attention, competing against the roadmap. It produces nothing demoable. It's hard to scope, because the true blast radius isn't known until you're in it. And it risks breaking working software for an improvement nobody outside the team will notice. That's why it loses every prioritization meeting it enters, and why "clean up the payments module" has been carried over for two years.

That's not a knowledge problem. It's a **price** problem.

CodeFriend attacks the price on four fronts:

**Identification becomes continuous and nearly free.** Drift, eroding boundaries, dependency risk, stale decisions and coverage gaps are surfaced as they accrue, with evidence attached — not discovered during an incident.

**Scoping stops being guesswork.** Dependency relationships, module boundaries and a severity-ordered remediation sequence turn "two weeks, possibly six" into an argument with evidence behind it.

**The scary part gets a safety net.** Generated regression, boundary and refactor-preservation tests exist so a cleanup has proof it didn't change behavior. That's the difference between a refactor you can defend and one you can only hope about.

**The case becomes legible to whoever approves the time.** A remediation sequence with severity, evidence, affected paths and residual risk is something a VP of Engineering can weigh against a feature. "The team says the payments module is bad" is not.

To be precise about the boundary: Beta 1 makes debt **visible, priced, sequenced and safe to pay**. It doesn't broadly refactor your code — that's out of scope, for a reason I'll come to. What changes is the economics of the decision, which is the part that was blocking you.

## Four things, and they go further than they sound

Beta 1 does four things well: **diagrams, documentation, reviews, and tests.**

Written down that can look modest. In practice it's a larger intervention than it reads, because of where most codebases actually start.

Take architecture decision records. The standard pitch for ADR tooling assumes you have ADRs and they've drifted. A great many organizations don't have any. The decisions were made in a meeting, or a thread, or someone's head, and the only surviving record is the shape of the code — which is why nobody can answer why the payments service talks to notifications through a queue.

For those teams CodeFriend isn't maintaining a decision record. It's drafting the first one, reconstructed from what the repository and its history show, with inference marked as inference.

And notice the asymmetry, because it's the whole business in one line. Writing an ADR from scratch is expensive for a human — someone has to reconstruct the history, separate what was decided from what merely happened, write it up and get it reviewed, which is why it never reaches the top of anyone's list. Drafting one from repository evidence is cheap for CodeFriend. **Same artifact, radically different price.**

You still review it. It arrives as a draft, not a verdict, and a decision record nobody confirmed is a guess in a template. But reviewing a draft that cites its evidence is a twenty-minute job, and writing one from nothing is a day you'll never schedule.

Diagrams are the same story with a sharper edge. Most repositories have none — or worse, they have one that was accurate in 2023.

A missing diagram is an inconvenience. A **wrong** diagram is a liability, because it gets believed. It's in the onboarding deck. It's what the new engineer reasons from for their first six months. It's what someone sketches from in an incident review, and it quietly omits the service that was added last spring and the dependency that now goes the other way. Nobody updates it, because updating it means re-deriving the whole thing from source — which is the same expensive job as writing it in the first place.

Generated diagrams are source-backed and carry a truth boundary: what's grounded in the repository, what's inferred, what's unknown. That gives you two things a wiki diagram never does. It's right now, and it tells you which parts you should trust.

And it's worth saying why this matters more than it sounds, because "we generate diagrams" is easy to file under *nice-to-have*. **A good diagram makes real problems visible in a way prose cannot.** Structural problems have a shape. You can read forty pages of architecture documentation and not notice that three services form a dependency cycle; you see it in a graph immediately, because it's a loop. A module that everything points at is a paragraph you skim and a hub you cannot miss. A boundary that's been crossed eleven times is a statistic in a report and an obvious violation on a map.

That's not a presentation preference. Some properties of a system are topological, and topology is the one thing text is genuinely bad at conveying. Which is exactly why the missing and wrong diagrams matter: the organization isn't just short of documentation, it's blind to an entire class of problem it has no way to see.

The rest of the list runs the same way. Onboarding documentation for a repository where onboarding means three weeks of shoulder-tapping. A regression test for a failure path that's never had one. Coverage on the module everybody is afraid of.

These aren't improvements to existing practice. For most teams they're the practice arriving for the first time — or arriving correct for the first time — and they compound, because each one makes the next change cheaper to reason about.

Deeper architecture governance — fitness functions, forbidden-import and layer checks, dependency-cycle enforcement as release gates — is the next increment. Beta 1 gets you the evidence and the artifacts. Turning selected rules into automated gates comes after.

## A run, end to end

Point it at a repository, local or GitHub. It establishes explicit scope first: repository identity, commit under review, languages, build system, test commands, CI configuration, documentation locations, which paths are included and excluded, privacy mode, review mode.

That scope is a visible artifact, not an internal detail — you can see exactly what was examined and what wasn't, which is the precondition for trusting anything downstream.

Then it produces repository intelligence (the brief you'd want if you were the new senior engineer on the codebase), answers substantive engineering questions from evidence, and runs **multi-specialist review** across seven lanes: code correctness, security, architecture, dependencies and supply chain, tests and coverage, documentation truth, and synthesis. Not one enormous review prompt — each lane records what it reviewed, what it *didn't*, findings by severity, confidence, evidence, recommended action, and residual risk.

Everything lands in a **review packet**: run manifest, scope, specialist reviews, diagrams, generated tests, documentation observations, security notes, remediation sequence, residual risks, redaction report, final synthesis. The packet is a product surface, not an internal artifact.

## Investigate before enhancing

The most important design decision in the beta is a boundary most products in this space blur.

**Investigation is read-only** — understanding, diagnosing, explaining, reviewing. **Enhancement modifies your software**, and therefore requires stronger authority, explicit approval, and more careful review.

Beta 1 is overwhelmingly investigation-oriented, with one deliberate exception: **test generation**. Tests are tedious, teams want help with them, and generating them improves quality without displacing anyone's creative work. It's the one place where writing code is obviously welcome.

If you're wondering why a system that understands your codebase this well doesn't simply implement the fixes it recommends — that's deliberate. A reviewer that also ships the changes has an obvious conflict of interest, and the thing you're trusting CodeFriend to do is tell you the truth about your architecture. Those two jobs pull against each other, so we've kept them apart.

The rule underneath it governs the whole roadmap:

> **CodeFriend should earn trust before it asks for authority.**

Which is the inverse of how most AI tooling has been arriving — authority first, trust assumed, evidence later.

So: no autonomous merge approval, no silent repository mutation, no autonomous feature development, no broad refactoring, no customer PR changes without explicit approval. And no claim, anywhere, that CodeFriend replaces software engineers.

## Trust is the product

An AI-generated engineering report is fluent by construction, and fluency reads as authority. A confident paragraph and a well-supported paragraph are typographically identical. Here that isn't cosmetic — it's the central risk, because a review tool that's occasionally and invisibly wrong launders guesses into decisions.

So every finding, diagram, recommendation and generated test ties back to evidence, and reports state scope, assumptions, confidence, non-reviewed surfaces and residual risk explicitly.

Two mechanisms enforce that rather than merely encouraging it. **Privacy and redaction gates** — a report *fails* publication when a privacy or evidence boundary is violated. And a **review quality gate**: CodeFriend reviews its own reviews, rejecting or downgrading them when findings lack evidence, severity is unsupported, non-reviewed surfaces are omitted, specialist disagreement has been smoothed over, diagrams contain unsupported relationships, or the report claims approval, compliance or merge-ready status that was never verified.

That last one is what I'd want in any tool producing engineering reports. Unearned confidence isn't a tone problem, it's a defect, and here it's treated as one.

## The repository is the first unit, not the last

Most engineering organizations don't have a codebase. They have hundreds or thousands of repositories in wildly varying states of ownership, documentation, testing and neglect — and no coherent view of which need attention first, which have no active owner, which are drifting, which appear abandoned.

CodeFriend's path runs repository → application → business system → portfolio → engineering organization. Beta 1 doesn't deliver that portfolio view; what it does is ensure the data model and artifact contracts don't block it, which is a design constraint now rather than a rewrite later.

Beta 1 is milestone v0.92.2, and the package is public — exit bar, demo matrix, explicit deferrals. The item we'd point a skeptic at is the last one: **one bounded external open-source repository proof.** Reviewing the repository you grew up in proves very little. Reviewing one you've never seen is the actual test, and it's the test we've set for ourselves in public.

## Built on ADL

CodeFriend runs on the ADL platform: deterministic runtime, structured contracts, ObsMem, Freedom Gate, replay and trace.

That's not architecture for its own sake. A system that reads your entire codebase, reasons about it, writes tests into it and produces reports your organization may act on is exactly the case where "the model suggested it and the application did it" stops being acceptable. Evidence with provenance, typed authority, recorded refusals, replayable runs — the platform is why the trust claims above are enforceable rather than aspirational.

## Try it

**[codefriend.ai](https://codefriend.ai)**

Point it at a repository — ideally one you know well enough to catch it being wrong. That's the most useful thing you can do for us right now: tell us where a finding isn't grounded, where an architecture claim doesn't survive contact with someone who remembers the history, where a diagram asserts a relationship that isn't there.

Because your code needs a friend. Preferably one that has read the whole thing, and says so when it hasn't.

**[codefriend.ai](https://codefriend.ai) · [agent-logic.ai](https://agent-logic.ai) · [github.com/agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language)**

---

### Go deeper

- [v0.92.2 — CodeFriend Beta 1](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92.2/README.md) — milestone package, exit bar, explicit deferrals
- [Demo matrix](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.92.2/DEMO_MATRIX_v0.92.2.md) — what gets proven, and how
- [ADR 0025](https://github.com/agent-logic/agent-design-language/blob/main/docs/adr/0025-codefriend-review-packet-product-boundary.md) — the evidence and product boundary
- [CodeFriend productization](https://github.com/agent-logic/agent-design-language/blob/main/docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md) — the review-packet baseline
