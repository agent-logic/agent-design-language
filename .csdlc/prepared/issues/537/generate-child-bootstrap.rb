#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"

ROOT = File.expand_path("../../../..", __dir__)

COMMON_CONSTRAINTS = [
  "Use typed C-SDLC v2 lifecycle only",
  "Use a dedicated FastWork issue worktree and issue-bound session goal",
  "Run one bounded exact-head review before publication",
  "Fail closed on stale, skipped, zero-denominator, non-ancestral, or non-proving evidence",
  "Do not widen into another Sprint 9 child's ownership"
].freeze

SPECS = {
  515 => {
    title: "Local-model shadow execution",
    slug: "local-model-shadow-execution",
    goal: "Produce one bounded local-model shadow-execution and comparison path that cannot acquire authority.",
    outcome: "Shadow execution is distinguishable, deterministic, redacted, and unable to mutate or replace the authoritative provider result.",
    scope: ["adl/src/provider", "docs/milestones/v0.92.1/evidence/provider/prov-b", ".csdlc/prepared/issues/515/validate-provider-shadow.rb"],
    dependencies: ["Terminal reviewed and ancestral PROV-A issue #514", "Sprint 9 umbrella #537"],
    criteria: [
      "AC-1: Shadow and authoritative provider paths are unambiguously distinguishable.",
      "AC-2: Inputs and comparison rules are exact and deterministic.",
      "AC-3: Shadow failures cannot mutate or replace the authoritative result.",
      "AC-4: Retained comparison evidence is redacted and source-revision bound.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Provider benchmark marketing claims", "Production provider cutover", "Changing provider authority"],
    validator: ".csdlc/prepared/issues/515/validate-provider-shadow.rb",
    proof: "Shadow isolation, deterministic comparison, authoritative fallback, redaction, and negative mutation checks"
  },
  516 => {
    title: "Release-tail admission",
    slug: "release-tail-admission",
    goal: "Produce one immutable release-tail admission decision for the converged milestone candidate.",
    outcome: "The admission record indexes every exact reviewed-green ancestral root, its artifacts, and a zero-unresolved-collision result.",
    scope: ["docs/milestones/v0.92.1/evidence/integration", "docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md", ".csdlc/prepared/issues/516/validate-release-tail-admission.rb"],
    dependencies: ["Terminal #498", "Terminal #496", "Terminal #494", "Terminal #495", "Terminal #499", "Terminal #505", "Terminal #508", "Terminal #509", "Terminal #51", "Terminal #510", "Terminal #512", "Terminal #513", "Terminal #515", "Sprint 9 umbrella #537"],
    criteria: [
      "AC-1: The exact 13-root denominator is reviewed, merged, terminal, and ancestral to the candidate.",
      "AC-2: Exact revisions and retained artifacts are indexed without substituting successor or sibling authority.",
      "AC-3: Cross-lane path, contract, claim, and evidence collisions are resolved by their owners.",
      "AC-4: Missing, stale, skipped, non-proving, or ambiguous roots deny admission.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Implementing child fixes", "Release approval", "Tagging, releasing, or publishing"],
    validator: ".csdlc/prepared/issues/516/validate-release-tail-admission.rb",
    proof: "Exact merged-authority census, ancestry, artifact index, collision denial, and immutable admission decision"
  },
  517 => {
    title: "Quality gate",
    slug: "release-tail-quality-gate",
    goal: "Produce one quality-gate decision for the exact candidate admitted by #516.",
    outcome: "Every required proving lane passes for the exact candidate and the gate reports zero unowned exceptions.",
    scope: ["docs/milestones/v0.92.1/evidence/release/tail-01", "docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md", ".csdlc/prepared/issues/517/validate-quality-gate.rb"],
    dependencies: ["Terminal reviewed and ancestral INT-01 issue #516", "Sprint 9 umbrella #537"],
    criteria: [
      "AC-1: The exact required quality denominator is complete for the admitted candidate.",
      "AC-2: Every required proving lane passes with a nonzero exact denominator.",
      "AC-3: Skipped, missing, stale, filtered-to-zero, non-proving, or ambiguous results deny the gate.",
      "AC-4: The decision reports zero unowned exceptions.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Documentation repair", "Release ceremony", "Implementing failed-lane fixes"],
    validator: ".csdlc/prepared/issues/517/validate-quality-gate.rb",
    proof: "Quality denominator, zero-test shield, exact-scope result, exception ownership, and fail-closed decision"
  },
  518 => {
    title: "Documentation review and external-review handoff",
    slug: "documentation-external-review-handoff",
    goal: "Produce one exact-revision documentation review and context-free external-review handoff packet.",
    outcome: "The canonical document denominator, links, claims, residual risks, and handoff instructions agree for the exact quality-gated candidate.",
    scope: ["docs/milestones/v0.92.1/evidence/release/tail-02", ".csdlc/prepared/issues/518/validate-doc-review-handoff.rb"],
    dependencies: ["Terminal reviewed and ancestral TAIL-01 issue #517", "Sprint 9 umbrella #537"],
    criteria: [
      "AC-1: All canonical v0.92.1 documents agree on scope, status, and non-claims.",
      "AC-2: Links and release claims resolve to exact source-grounded evidence.",
      "AC-3: Residual risks, skipped surfaces, operator actions, and external-review limits are explicit.",
      "AC-4: The handoff is context-free, redacted, and exact-revision bound.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Product implementation", "Publication", "Merge, tag, or release mutation"],
    validator: ".csdlc/prepared/issues/518/validate-doc-review-handoff.rb",
    proof: "Canonical document inventory, link check, claim audit, redaction, exact-revision handoff, and diff hygiene"
  },
  519 => {
    title: "Publication finalization",
    slug: "publication-candidate-finalization",
    goal: "Produce one exact-revision publication-candidate packet without performing publication or release mutation.",
    outcome: "The packet binds the exact reviewed candidate, correct closing relationships, publication linkage, and redacted artifacts while leaving merge, tag, release, and external publication untouched.",
    scope: ["docs/milestones/v0.92.1/evidence/release/tail-03", ".csdlc/prepared/issues/519/validate-publication-candidate.rb"],
    dependencies: ["Terminal reviewed and ancestral TAIL-02 issue #518", "Sprint 9 umbrella #537"],
    criteria: [
      "AC-1: The publication-candidate packet records the exact reviewed revision and artifact denominator.",
      "AC-2: Issue and pull-request linkage, including closing relationships, is correct and unambiguous.",
      "AC-3: Publication artifacts are redacted and contain no private paths, credentials, or unsupported claims.",
      "AC-4: Stale review, ambiguous linkage, missing artifacts, or digest mismatch denies candidate readiness.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Merge", "Tag", "Release", "External publication", "Release ceremony"],
    validator: ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
    proof: "Publication linkage, exact-head identity, closing relationships, artifact denominator, redaction, and digest checks"
  }
}.freeze

def design_text(issue, spec)
  <<~MD
    # Issue #{issue} Design — #{spec.fetch(:title)}

    ## Goal

    #{spec.fetch(:goal)}

    ## Required Outcome

    #{spec.fetch(:outcome)}

    ## Ownership

    #{spec.fetch(:scope).map { |path| "- `#{path}`" }.join("\n")}

    ## Dependencies

    #{spec.fetch(:dependencies).map { |value| "- #{value}" }.join("\n")}

    ## Safety Boundary

    - This issue owns only the listed result and paths.
    - Missing, stale, skipped, non-proving, or ambiguous evidence fails closed.
    - Validation and independent exact-head review precede publication.

    ## Non-Goals

    #{spec.fetch(:non_goals).map { |value| "- #{value}" }.join("\n")}
  MD
end

def diagram_text(issue, spec)
  dependency = case issue
               when 515 then "#514"
               when 516 then "#498 + #496 + #494 + #495 + #499 + #505 + #508 + #509 + #51 + #510 + #512 + #513 + #515"
               when 517 then "#516"
               when 518 then "#517"
               when 519 then "#518"
               end
  outcome = issue == 519 ? "Candidate packet only<br/>No merge, tag, release, or external publication" : "Reviewed terminal child outcome"
  <<~MMD
    flowchart LR
      D["#{dependency}"] --> I["Issue #{issue}: #{spec.fetch(:title)}"]
      I --> V["Focused fail-closed proof"] --> R["Independent exact-head review"] --> T["#{outcome}"]
  MMD
end

SPECS.each do |issue, spec|
  dir = File.join(ROOT, ".csdlc/prepared/issues/#{issue}")
  FileUtils.mkdir_p(dir)
  design_path = ".csdlc/prepared/issues/#{issue}/design.md"
  diagram_path = ".csdlc/prepared/issues/#{issue}/diagram.mmd"
  File.write(File.join(ROOT, design_path), design_text(issue, spec))
  File.write(File.join(ROOT, diagram_path), diagram_text(issue, spec))

  request = {
    issue: issue,
    repository: "agent-logic/agent-design-language",
    actor: "codex:sprint9-readiness",
    design_path: design_path,
    diagram_path: diagram_path,
    design_reviewer: "pending-independent-review",
    design_approved: false,
    initial: {
      title: spec.fetch(:title), slug: spec.fetch(:slug), version: "v0.92.1",
      goal: spec.fetch(:goal), required_outcome: spec.fetch(:outcome),
      declared_scope: spec.fetch(:scope),
      authority_boundary: ["Issue #{issue} owns only its declared result and paths; Sprint 9 umbrella #537 coordinates but cannot implement or approve this child."],
      operator_constraints: COMMON_CONSTRAINTS,
      task_boundary: "Deliver only issue ##{issue}: #{spec.fetch(:title)}.",
      deliverables: [spec.fetch(:outcome), "Issue-specific retained validation evidence", "Independent exact-head review and truthful terminal record"],
      acceptance_criteria: spec.fetch(:criteria), dependencies: spec.fetch(:dependencies),
      repo_inputs: spec.fetch(:scope) + (issue == 518 ? ["docs/milestones/v0.92.1", "docs/planning/ADL_FEATURE_LIST.md"] : []) + ["docs/milestones/v0.92.1/SPRINT_v0.92.1.md", ".csdlc/prepared/issues/537/sprint-execution-packet.yaml"],
      non_goals: spec.fetch(:non_goals),
      plan_summary: "Verify the predecessor gate, implement the smallest owned result, run focused proof, obtain one exact-head review, publish, shepherd, finish, and clean up.",
      steps: [
        { id: "dependency-gate", action: "Verify every declared predecessor and authority gate", acceptance_ids: ["AC-1"], status: "pending" },
        { id: "implement", action: "Implement the bounded owned result", acceptance_ids: spec.fetch(:criteria).each_index.map { |i| "AC-#{i + 1}" }, status: "pending" },
        { id: "validate-review", action: "Run focused proof and one independent exact-head review", acceptance_ids: ["AC-5"], status: "pending" }
      ],
      affected_areas: spec.fetch(:scope),
      invariants: ["No ownership widening", "No action before dependencies", "No authority substitution", "Review precedes publication", "Failure preserves prior valid state and evidence"],
      risks: ["Dependency truth could drift", "A child could cross another issue's write boundary", "Evidence could overclaim exact candidate or terminal state"],
      planning_profile: issue == 515 ? "large" : "medium",
      stop_conditions: ["Any predecessor is nonterminal or ambiguous", "Any owned-path collision", "Any missing or zero proof target", "Any unsupported completion claim", "Any unresolved actionable review finding"],
      validation_lanes: [
        {
          lane: "issue-#{issue}-focused", proof_role: spec.fetch(:proof),
          acceptance_ids: spec.fetch(:criteria).each_index.map { |i| "AC-#{i + 1}" },
          deterministic: true, resource_profile: issue == 515 ? "large" : "small",
          budget_seconds: issue == 515 ? 1800 : 300, budget_tokens: issue == 515 ? 8000 : 3000,
          argv: [spec.fetch(:validator)], parallel_group: "sprint9-issue-#{issue}",
          defer_reason: "Deferred until this child is bound, every declared predecessor passes, and the issue-owned validator is implemented; missing target, zero denominator, or failure blocks publication."
        },
        {
          lane: "issue-#{issue}-diff-hygiene", proof_role: "Reject malformed tracked changes before exact-head review.",
          acceptance_ids: ["AC-5"], deterministic: true, resource_profile: "small",
          budget_seconds: 30, budget_tokens: 200, argv: ["git", "diff", "--check"],
          parallel_group: "sprint9-hygiene", defer_reason: "Run after the issue has a bounded candidate diff."
        }
      ],
      failure_policy: "Fail closed on dependency, ownership, authority, privacy, validation, exact-revision, or review drift; preserve evidence and route separate defects without widening this issue.",
      review_prompts: ["Does the candidate satisfy every acceptance criterion on its real owned path?", "Does it preserve predecessor authority, sibling ownership, and non-goals?", "Are proof and lifecycle claims exact-revision and non-overstated?"],
      review_scope: "Review issue ##{issue} design, implementation, focused proof, negative cases, ownership, exact-revision identity, and terminal truth."
    }
  }
  File.write(File.join(dir, "bootstrap-request.json"), JSON.pretty_generate(request) + "\n")
end

puts JSON.generate(status: "generated", issues: SPECS.keys.sort)
