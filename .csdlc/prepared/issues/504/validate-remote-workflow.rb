#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

ROOT = File.expand_path("../../../..", __dir__)

def read(path)
  File.read(File.join(ROOT, path))
end

def json(path)
  JSON.parse(read(path))
end

def assert(condition, message)
  raise message unless condition
end

issue = json(".csdlc/issues/504/index.json")
stp = read(".csdlc/issues/504/cards/stp.md")
sip = read(".csdlc/issues/504/cards/sip.md")
spp_markdown = read(".csdlc/issues/504/cards/spp.md")
spp = json(".csdlc/issues/504/cards/spp.values.json")
vpp = json(".csdlc/issues/504/cards/vpp.values.json")
srp = json(".csdlc/issues/504/cards/srp.values.json")
packet_text = [stp, sip, spp_markdown].join("\n")

assert(issue["issue"] == 504, "wrong issue")
assert(issue["repository"] == "agent-logic/agent-design-language", "wrong repository")
phase = issue["phase"]
assert(["initialized", "ready", "implemented"].include?(phase), "validator expects initialized, ready, or implemented #504")
if phase == "implemented"
  assert(issue["branch"] == "codex/504-v3-e-remote-delivery-workflow-exec", "implemented #504 branch drift")
  assert(issue["worktree"] == File.join(File.dirname(ROOT), "adl-issue-504-v3-e-remote-delivery-workflow-exec"), "implemented #504 worktree drift")
else
  assert(issue["worktree"].nil?, "pre-bind validator requires #504 to remain unbound")
end

required = [
  "Review binds exact immutable scope",
  "Publication modes are explicit",
  "Finish derives terminal truth",
  "Requirements #174 through #178 have positive and refusal proof"
]
required.each { |text| assert(stp.include?(text), "missing acceptance text: #{text}") }

deps = spp.dig("content", "values", "steps").to_s + stp
assert(deps.include?("#503"), "missing #503 dependency")
assert(deps.include?("terminal") && deps.include?("ancestral"), "missing terminal/ancestral dependency language")
assert(packet_text.include?("Closes #504"), "missing visible future closing-linkage requirement")
assert(packet_text.include?("C-SDLC v2 remains") || packet_text.include?("C-SDLC v2"), "missing v2 authority boundary")
assert(packet_text.include?("construction-only") || packet_text.include?("non-authoritative"), "missing v3 non-authority boundary")

lanes = vpp.dig("content", "values", "lanes")
assert(lanes.is_a?(Array) && lanes.length == 1, "pre-bind #504 should expose exactly one executable preparation lane")
lane = lanes.first
assert(lane["lane"] == "prebind-v3-e-preparation", "unexpected pre-bind lane")
assert(lane["argv"] == ["ruby", ".csdlc/prepared/issues/504/validate-remote-workflow.rb"], "pre-bind lane must target this validator")
assert(lane["defer_reason"].nil?, "pre-bind validator must be executable, not deferred")

if phase == "implemented"
  [
    "csdlc-v3/src/commands/remote/mod.rs",
    "csdlc-v3/src/commands/remote/tests.rs",
    "csdlc-v3/src/review/mod.rs",
    "csdlc-v3/src/publication/mod.rs",
  ].each do |path|
    assert(File.file?(File.join(ROOT, path)), "missing implemented artifact #{path}")
  end
  remote = read("csdlc-v3/src/commands/remote/mod.rs")
  remote_tests = read("csdlc-v3/src/commands/remote/tests.rs")
  review = read("csdlc-v3/src/review/mod.rs")
  publication = read("csdlc-v3/src/publication/mod.rs")

  assert(remote.include?("pub(crate) fn deliver"), "deliver must not be a public caller-forgeable authority API")
  assert(remote.include?("pub(crate) fn new("), "verified observation construction must be crate-scoped")
  assert(remote.include?("pub(crate) fn receipt("), "authority receipts must be crate-scoped")
  assert(remote.include?("subject_digest"), "authority receipts must bind to an observed subject digest")
  assert(!remote.include?("pub fn deliver("), "remote delivery must not expose public deliver()")
  assert(!remote.include?("pub fn receipt("), "remote delivery must not expose public receipt minting")
  assert(!remote.include?("pub fn new(value"), "verified observation minting must not be public")
  assert(review.include?("pub(crate) issue: u64"), "publication authorization fields must not be public caller-forgeable authority")
  assert(publication.include?("pub(crate) fn publish("), "publish must not expose a public authorization-bypass API")
  assert(publication.include?("pub(crate) fn derive_finish("), "finish derivation must not expose a public caller-forgeable API")
  assert(publication.include?("pub(crate) fn execute_cleanup_removal("), "cleanup removal must not expose a public caller-forgeable API")
  assert(!publication.include?("pub fn publish("), "publish must not be public")
  assert(!publication.include?("pub fn derive_finish("), "derive_finish must not be public")
  assert(!publication.include?("pub fn execute_cleanup_removal("), "execute_cleanup_removal must not be public")
  assert(remote_tests.include?("part_of_checkpoint_completes_through_end_to_end_delivery_without_cleanup"), "missing end-to-end part-of checkpoint proof")
  assert(remote_tests.include?("whitespace_variant_principal_cannot_self_authorize_publication"), "missing whitespace self-review regression proof")
  assert(remote_tests.include?("cleanup_removal_executes_and_distinguishes_removed_states"), "missing cleanup execution/removal proof")
  assert(remote_tests.include?("already_removed_cleanup_still_requires_terminal_and_preview_gates"), "missing already-removed cleanup gate regression proof")
  assert(review.include?("let left = left.trim();") && review.include?("let right = right.trim();"), "review principal comparison must normalize both sides")
  assert(publication.include?("fs::remove_dir_all(&path)"), "cleanup removal must perform filesystem removal")
  assert(publication.include?("UnregisteredWorktree"), "cleanup must distinguish unregistered worktree states")
end

review_prompts = srp.dig("content", "values", "review_prompts") || []
combined = review_prompts.join("\n") + "\n" + packet_text
["#503", "#174 through #178", "Closes #504"].each do |needle|
  assert(combined.include?(needle), "missing review/planning prompt marker #{needle}")
end

puts JSON.generate(
  {
    schema: "adl.csdlc_v3.issue504.prebind_validation.v1",
    status: "pass",
    issue: 504,
    phase: phase,
    checked: [
      "acceptance_denominator",
      "503_terminal_dependency",
      "v2_authority_boundary",
      "v3_non_authority_boundary",
      "future_closing_linkage",
      "single_executable_prebind_lane",
      phase == "implemented" ? "bound_implemented_artifacts" : "unbound_ready_gate",
      phase == "implemented" ? "sealed_remote_authority_api" : "prebind_authority_plan"
    ]
  }
)
