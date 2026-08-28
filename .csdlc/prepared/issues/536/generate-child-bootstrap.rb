#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "fileutils"

ROOT = File.expand_path("../../../..", __dir__)

COMMON_CONSTRAINTS = [
  "Use typed C-SDLC v2 lifecycle only",
  "Use a dedicated FastWork issue worktree and issue-bound session goal",
  "Run one bounded exact-head review before publication",
  "Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data",
  "Do not widen into another Sprint 8 child's ownership"
].freeze

SPECS = {
  261 => {
    title: "Podcast show identity, artwork, rights, metadata, and mailbox readiness",
    slug: "podcast-show-identity",
    goal: "Produce one operator-approved, collision-reviewed podcast identity packet for downstream episode and hosting work.",
    outcome: "A versioned show-identity manifest, final rights-backed artwork, consistent metadata, and redacted company-mailbox readiness proof are accepted without publishing anything.",
    scope: ["docs/milestones/v0.92.1/evidence/podcast/51-a", "demos/podcast/show-identity"],
    dependencies: ["Sprint 8 umbrella #536", "Operator approval of the final show identity", "Company-controlled mailbox ownership"],
    criteria: [
      "AC-1: The operator approves a collision-reviewed title and complete show metadata.",
      "AC-2: Final artwork has exact source, rights, dimensions, color space, format, and digest evidence.",
      "AC-3: Company-mailbox receipt proves readiness without retaining private addresses, credentials, or verification codes.",
      "AC-4: One versioned manifest is internally consistent and consumable by #342 and #262.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Hosting or RSS publication", "Directory submission", "Episode production", "Public launch"],
    validator: ".csdlc/prepared/issues/261/validate-show-identity.rb",
    proof: "Show-name collision, rights, metadata parity, artwork constraints, mailbox redaction, and digest validation"
  },
  342 => {
    title: "Podcast Studio first ten episode packages",
    slug: "podcast-first-ten-episode-packages",
    goal: "Create ten complete review-ready episode packages using the approved #261 identity inputs without owning production hosting or feed publication.",
    outcome: "Ten complete episode-package directories pass package, audio, metadata, redaction, editorial, and digest checks while production feed and deployment remain outside this issue.",
    scope: ["demos/podcast/episode-packages", "docs/milestones/v0.92.1/evidence/podcast/wp-24a", "adl/tools/generate_podcast_launch_packet.py", "adl/tools/validate_podcast_launch_packet.py", "adl/tools/test_podcast_launch_packet.sh"],
    dependencies: ["Sprint 8 umbrella #536", "Terminal #261 canonical show identity and rights inputs", "Retained Podcast Studio v2 proof and approved route/storage decision"],
    criteria: [
      "AC-1: All ten episode packages contain every required script, audio, transcript, note, metadata, artwork, enclosure fragment, redaction, QA, and review artifact.",
      "AC-2: Audio and manifest digests, duration, sample rate, channels, loudness, peak, ID3, artwork, listen check, and archive records agree.",
      "AC-3: Episode enclosure fragments reject local paths, drafts, unstable GUIDs, and metadata mismatches without mutating the production feed.",
      "AC-4: Rights, consent, synthetic-voice provenance, and redaction remain truthful and privacy-safe.",
      "AC-5: Source-SHA-bound playback receipts and exact-head review pass before terminal completion."
    ],
    non_goals: ["Production feed ownership", "Hosting or deployment", "Directory submission", "Mailbox verification", "Public launch"],
    validator: "adl/tools/test_podcast_launch_packet.sh",
    proof: "Ten-package completeness, audio and metadata parity, redaction, playback, and ownership-boundary validation"
  },
  262 => {
    title: "Podcast production hosting, RSS, enclosures, and playback",
    slug: "podcast-production-hosting",
    goal: "Publish and validate the canonical production podcast feed and stable HTTPS media enclosures from approved identity and terminal episode packages.",
    outcome: "The production feed, enclosure metadata, byte-range behavior, and representative desktop/mobile playback are source-grounded, digest-consistent, and rollback-safe.",
    scope: ["demos/podcast/feed.xml", "docs/milestones/v0.92.1/evidence/podcast/51-b", "adl/tools/record_podcast_native_playback.sh", "adl/tools/record_podcast_browser_playback.mjs", "adl/tools/record_podcast_ios_safari_playback.sh"],
    dependencies: ["Terminal #261", "Terminal #342 episode packages", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: The canonical feed validates with no local, preview, placeholder, smoke-test, or fixture URLs.",
      "AC-2: Every enclosure is stable HTTPS media with correct MIME type, bytes, duration, GUID, date, and digest.",
      "AC-3: HEAD, GET, and 206 byte-range behavior plus representative desktop/mobile playback pass.",
      "AC-4: Feed, artwork, show metadata, and episode metadata match #261 and #342 exactly.",
      "AC-5: Rollback preserves episode packages and prior evidence; exact-head review is clean."
    ],
    non_goals: ["Show identity decisions", "Episode production", "Directory submission", "Public launch announcement"],
    validator: ".csdlc/prepared/issues/262/validate-podcast-hosting.rb",
    proof: "RSS, enclosure, HTTPS, MIME, byte-range, desktop/mobile playback, rollback, and metadata parity"
  },
  263 => {
    title: "Podcast directory submission runbooks and operator preflight",
    slug: "podcast-directory-runbooks",
    goal: "Prepare current provider-specific directory submission runbooks and one redacted operator preflight without mutating provider accounts.",
    outcome: "Apple, Spotify, Amazon, and YouTube runbooks identify every account-side and irreversible step, consume the exact production feed, and hand a safe ledger schema to #264.",
    scope: ["docs/milestones/v0.92.1/evidence/podcast/51-c", "docs/podcast/directory-runbooks"],
    dependencies: ["Terminal #261", "Terminal #262", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: Each runbook is verified against current official provider instructions at exact-head review time.",
      "AC-2: The exact #261 identity and #262 production feed/enclosures are referenced.",
      "AC-3: Account, 2FA, rights, verification, submit, and publish actions are marked operator-controlled.",
      "AC-4: No provider mutation, directory submission, credential capture, or public launch occurs.",
      "AC-5: The ledger schema retains canonical IDs/status without secrets or unsupported acceptance claims."
    ],
    non_goals: ["Provider submission", "Provider account creation or mutation", "Hosting implementation", "Public launch"],
    validator: ".csdlc/prepared/issues/263/validate-directory-runbooks.rb",
    proof: "Official-instruction freshness, feed identity, operator-action census, redaction, and ledger-schema checks"
  },
  264 => {
    title: "Operator-authorized podcast directory submissions",
    slug: "podcast-directory-submissions",
    goal: "Execute only separately authorized provider submissions and retain truthful redacted IDs, status, correction, monitoring, and rollback evidence.",
    outcome: "Each authorized submission has an exact provider identity and truthful status while unauthorized providers remain untouched.",
    scope: ["docs/milestones/v0.92.1/evidence/podcast/51-d", "docs/podcast/submission-ledger"],
    dependencies: ["Terminal #263", "Explicit future provider-specific operator authorization", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: No execution occurs before terminal #263 and explicit authorization naming each provider.",
      "AC-2: Every submission uses the exact reviewed feed, identity, artwork, rights declaration, and company account.",
      "AC-3: Canonical IDs, URLs, and status are retained without credentials, verification codes, or unsupported acceptance claims.",
      "AC-4: Destination links activate only after live verification; corrections and rollback preserve history.",
      "AC-5: Exact-head reviews pass before external action and after final reconciliation."
    ],
    non_goals: ["Automatic submission", "Action before explicit authorization", "Credential retention", "Hosting redesign", "Advertising or monetization"],
    validator: ".csdlc/prepared/issues/264/validate-directory-submissions.rb",
    proof: "Authorization, provider readback, ID/status truth, redaction, correction, rollback, and no-unauthorized-mutation checks"
  },
  511 => {
    title: "Observatory experience design",
    slug: "observatory-experience-design",
    goal: "Produce one reviewed Observatory information, interaction, state, hierarchy, and accessibility contract grounded in available Runtime fields.",
    outcome: "A complete experience-design contract covers every view and empty, degraded, recovery, revoked, keyboard, and screen-reader state without inventing Runtime authority.",
    scope: ["demos/html-observatory/design", "docs/observatory", "docs/milestones/v0.92.1/evidence/observatory/obs-a"],
    dependencies: ["Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: Every view has a stable information and interaction contract.",
      "AC-2: Empty, degraded, recovery, revoked, loading, and incompatible states are designed.",
      "AC-3: Keyboard and screen-reader flows plus focus and announcement behavior are specified.",
      "AC-4: Runtime field census proves no invented field or authority is introduced.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Production implementation", "Unity TLS or adapter work", "Runtime API changes"],
    validator: ".csdlc/prepared/issues/511/validate-observatory-experience.rb",
    proof: "Information-contract, state-matrix, accessibility-plan, and Runtime-field-census checks"
  },
  84 => {
    title: "Live Unity Observatory Runtime v3 integration",
    slug: "unity-observatory-runtime-v3",
    goal: "Bind the approved Unity Observatory to the stable Runtime v3 API and WSS contract with authenticated controls, explicit failure states, and native Editor proof.",
    outcome: "The Unity client consumes authentic Runtime v3 state, issues only authorized controls, reconnects safely, and retains positive and negative native Editor evidence without a parallel transport.",
    scope: ["demos/v0.91.6/unity-observatory", "adl/tools/validate_v092_unity_observatory_live.sh", "docs/milestones/v0.92.1/evidence/observatory/unity"],
    dependencies: ["Terminal reviewed #251 TLS 1.2 authority", "Terminal #122 public exposure", "Terminal evidence inputs #340 and #256", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: The Unity adapter consumes public HTTPS snapshots, WSS events, signed snapshot commands, local cursor continuity, correlation, and backpressure semantics.",
      "AC-2: The approved shell uses one Runtime v3 transport and the derived contract cannot fork from shared authority.",
      "AC-3: Only externally signed snapshot control is available; denied and unavailable actions never escalate authority.",
      "AC-4: TLS refusal, version mismatch, stale/offline data, malformed events, unavailable Runtime, and backpressure are explicit states.",
      "AC-5: Native Unity Editor proof uses the real Runtime path, builds no player, and exact-head review is clean."
    ],
    non_goals: ["HTML redesign", "Runtime API implementation", "TLS authority", "Provider integration", "AWS work", "Player build"],
    validator: "adl/tools/validate_v092_unity_observatory_live.sh",
    proof: "Unity client, contract parity, authorization, redaction, reconnect, failure-state, and native Editor checks"
  },
  512 => {
    title: "Authentic Runtime Observatory redesign implementation",
    slug: "observatory-redesign-implementation",
    goal: "Implement the accepted #511 Observatory design against authentic Runtime projections.",
    outcome: "The HTML Observatory implements the accepted contracts with exact browser, accessibility, redaction, recovery, and authentic Runtime-route proof.",
    scope: ["demos/html-observatory/app.js", "demos/html-observatory/styles.css", "adl/tools/validate_layer8_authority_observatory_ui.sh", "docs/milestones/v0.92.1/evidence/observatory/obs-b"],
    dependencies: ["Terminal #511", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: Every accepted #511 view, state, hierarchy, and interaction contract is implemented.",
      "AC-2: Final proof consumes accepted #511 contracts and source-grounded Runtime projections; backlog issue #84 is independent and is not a gate.",
      "AC-3: Exact browser, keyboard, screen-reader, redaction, degraded, recovery, and revoked cases pass.",
      "AC-4: No mock substitutes for the required authentic Runtime route.",
      "AC-5: Exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["TLS 1.2 implementation", "Public exposure", "Unity client changes", "Runtime API redesign"],
    validator: "adl/tools/validate_layer8_authority_observatory_ui.sh",
    proof: "Authentic Runtime route, exact browser cases, accessibility, redaction, recovery, and design-contract parity"
  },
  51 => {
    title: "Podcast publication coordination closeout",
    slug: "podcast-publication-coordination",
    goal: "Coordinate and reconcile the bounded podcast launch children without implementing their work or authorizing provider actions.",
    outcome: "One integrated podcast status view reconciles #261, #342, #262, #263, and #264 terminal truth, including an explicitly accepted operator-blocked #264 disposition when applicable.",
    scope: [".csdlc/issues/51", ".csdlc/prepared/issues/51", ".csdlc/evidence/51"],
    dependencies: ["Terminal #261", "Terminal #342", "Terminal #262", "Terminal #263", "Terminal #264 or explicit operator-accepted blocked disposition", "Sprint 8 umbrella #536"],
    criteria: [
      "AC-1: #261, #342, #262, and #263 have truthful terminal outcomes.",
      "AC-2: #264 has a terminal authorized outcome or an explicit operator-accepted blocked disposition.",
      "AC-3: Identity, rights, artwork, episode packages, feed, enclosures, runbooks, provider status, monitoring, correction, and rollback evidence agree where applicable.",
      "AC-4: No credential, verification code, private account data, or unsupported provider-acceptance claim enters retained evidence.",
      "AC-5: One integrated exact-head review has no unresolved actionable findings."
    ],
    non_goals: ["Child implementation", "Automatic provider submission", "Episode production", "Public launch without authorization"],
    validator: ".csdlc/prepared/issues/51/validate-podcast-coordination.rb",
    proof: "Exact child terminal denominator, cross-child metadata parity, provider-status truth, privacy, and integrated closeout checks"
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
    - All external mutations and private material remain governed by the operator constraints.
    - Validation and exact-head review precede publication.

    ## Non-Goals

    #{spec.fetch(:non_goals).map { |value| "- #{value}" }.join("\n")}
  MD
end

def diagram_text(issue, spec)
  deps = spec.fetch(:dependencies).each_index.map { |index| "  D#{index + 1}[\"Dependency #{index + 1}\"] --> I" }.join("\n")
  <<~MMD
    flowchart LR
    #{deps}
      I["Issue #{issue}"] --> P["Focused proof"] --> R["Exact-head review"] --> T["Terminal result"]
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
    actor: "codex:sprint8-readiness",
    design_path: design_path,
    diagram_path: diagram_path,
    design_reviewer: "pending-independent-review",
    design_approved: false,
    initial: {
      title: spec.fetch(:title),
      slug: spec.fetch(:slug),
      version: "v0.92.1",
      goal: spec.fetch(:goal),
      required_outcome: spec.fetch(:outcome),
      declared_scope: spec.fetch(:scope),
      authority_boundary: ["Issue #{issue} owns only its declared result and paths; Sprint 8 umbrella #536 coordinates but cannot implement or approve this child."],
      operator_constraints: COMMON_CONSTRAINTS + (issue == 264 ? ["External provider action is forbidden until a new explicit authorization names each approved provider"] : []),
      task_boundary: "Deliver only issue ##{issue}: #{spec.fetch(:title)}.",
      deliverables: [spec.fetch(:outcome), "Issue-specific retained validation evidence", "Exact-head review and truthful terminal record"],
      acceptance_criteria: spec.fetch(:criteria),
      dependencies: spec.fetch(:dependencies),
      repo_inputs: spec.fetch(:scope) + ["docs/milestones/v0.92.1/SPRINT_v0.92.1.md", ".csdlc/prepared/issues/536/sprint-execution-packet.yaml"],
      non_goals: spec.fetch(:non_goals),
      plan_summary: "Validate dependencies, implement the smallest owned result, run focused proof, obtain one exact-head review, publish, shepherd, finish, and clean up.",
      steps: [
        { id: "dependency-gate", action: "Verify all issue-specific dependencies and operator gates", acceptance_ids: ["AC-1"], status: "pending" },
        { id: "implement", action: "Implement the bounded owned result", acceptance_ids: spec.fetch(:criteria).each_index.map { |i| "AC-#{i + 1}" }, status: "pending" },
        { id: "validate-review", action: "Run focused proof and one exact-head review before publication", acceptance_ids: ["AC-5"], status: "pending" }
      ],
      affected_areas: spec.fetch(:scope),
      invariants: ["No ownership widening", "No action before dependencies", "No secret retention", "Review precedes publication", "Failure preserves prior valid state and evidence"],
      risks: ["Dependency truth could drift", "A child could cross another issue's write boundary", "Evidence could overclaim external or Runtime state"],
      planning_profile: issue == 264 || issue == 84 || issue == 512 || issue == 342 ? "large" : "medium",
      stop_conditions: ["Any dependency is nonterminal or ambiguous", "Any owned-path collision", "Any missing proof target", "Any secret or private material would enter evidence", "Any unresolved actionable review finding"],
      validation_lanes: [
        {
          lane: "issue-#{issue}-focused",
          proof_role: spec.fetch(:proof),
          acceptance_ids: spec.fetch(:criteria).each_index.map { |i| "AC-#{i + 1}" },
          deterministic: issue != 264 && issue != 84,
          resource_profile: issue == 264 || issue == 84 || issue == 512 || issue == 342 ? "large" : "small",
          budget_seconds: issue == 264 || issue == 84 || issue == 512 || issue == 342 ? 1800 : 300,
          budget_tokens: issue == 264 || issue == 84 || issue == 512 || issue == 342 ? 8000 : 3000,
          argv: [spec.fetch(:validator)],
          parallel_group: "sprint8-issue-#{issue}",
          defer_reason: "Deferred until the issue is bound, all declared dependencies pass, and the owned validator or proof target is implemented; missing target, zero tests, or any failure blocks publication."
        },
        {
          lane: "issue-#{issue}-diff-hygiene",
          proof_role: "Reject malformed tracked changes before exact-head review.",
          acceptance_ids: ["AC-5"],
          deterministic: true,
          resource_profile: "small",
          budget_seconds: 30,
          budget_tokens: 200,
          argv: ["git", "diff", "--check"],
          parallel_group: "sprint8-hygiene",
          defer_reason: "Run after the issue has a bounded candidate diff."
        }
      ],
      failure_policy: "Fail closed on dependency, ownership, authority, privacy, validation, exact-revision, or review drift; preserve evidence and route separate defects without widening the issue.",
      review_prompts: ["Does the candidate satisfy every acceptance criterion on its real owned path?", "Does it preserve sibling ownership, operator authority, privacy, and rollback?", "Are all proof claims exact-revision and non-overstated?"],
      review_scope: "Review issue ##{issue} design, implementation, focused proof, negative cases, ownership boundary, privacy, and terminal truth."
    }
  }
  File.write(File.join(dir, "bootstrap-request.json"), JSON.pretty_generate(request) + "\n")
end

puts JSON.generate({ status: "generated", issues: SPECS.keys.sort })
