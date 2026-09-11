#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

ROOT = "docs/milestones/v0.92.1/evidence/release/tail-06"
REPOSITORY = "agent-logic/agent-design-language"

ISSUES = {
  814 => [822, ["adl-runtime-kernel/src/control.rs", "adl-runtime/tests/shepherd_local_model.rs"]],
  815 => [825, [".csdlc/prepared/issues/815/test_cloud_authorization.py"]],
  816 => [823, [".csdlc/prepared/issues/816/validate-publication-manifest.py", "adl-runtime/tests/config_reload.rs"]],
  817 => [826, [".csdlc/prepared/issues/817/validate-release-truth.py"]],
  818 => [832, [".csdlc/evidence/818/retained-corporate-runtime/reconciliation.json"]],
  819 => [827, [".csdlc/evidence/819/retained-v3/reconciliation.json"]],
  820 => [828, [".csdlc/evidence/820/distributed-runtime/reconciliation.json"]],
  821 => [829, ["docs/milestones/v0.92.1/evidence/release/tail-06/issue-821/preparation.json"]],
  833 => [853, ["docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_REPORT.md", "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXECUTABLE_REVIEW_ADDENDUM.md", "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_REMEDIATION.md", "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXECUTABLE_REVIEW_LEDGER.md", "docs/milestones/v0.92.1/evidence/release/tail-05/ISSUE_834_CANDIDATE_REVALIDATION.md"]],
  834 => [841, ["docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/reconciliation.json"]],
  835 => [840, ["docs/milestones/v0.92.1/evidence/release/current-status/status.json"]],
  836 => [839, ["docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json"]],
  837 => [842, ["csdlc-v3/tests/command_manifest.rs"]],
  843 => [845, [".csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh"]],
  856 => [858, [".csdlc/evidence/856/VALIDATION.md", "docs/csdlc-v3/RELEASE_PREFLIGHT.md", "csdlc-v2/tests/gate9.rs"]]
}.freeze

GROUPS = [
  [%w[D520-RET-001], [818, 819, 820, 821], "Retained corporate, C-SDLC v3, distributed Runtime, and final-gate evidence were reconciled without promoting residual proof gaps."],
  [%w[D520-V3F-001], [817, 843], "Current v3 release truth and retained ready-intent recovery are present."],
  [%w[D520-REL-001 D520-DOC-003 D520-DOC-004 D520-EVID-001 D520-EVID-002], [817], "Canonical release documentation and evidence truth were refreshed."],
  [%w[D520-RUNTIME-001 D520-RUNTIME-002 D520-SEC-004], [814], "Runtime identity, local-model boundary, origin, and redirect behavior were repaired."],
  [%w[D520-SEC-001 D520-SEC-002], [815], "Cloud mutation authorization and replay boundaries were repaired."],
  [%w[D520-SEC-003 D520-TEST-001], [816], "Redaction and hot-reload proof were made executable and non-vacuous."],
  [%w[TPR-001 D833-REPORT-001 D833-REPORT-003 D833-EXEC-001], [833], "The immutable review, executable denominator, historical-state clarification, and narrow candidate-current supersession are retained by PR #853."],
  [%w[TPR-002 D833-REPORT-002 D833-EXEC-003], [834], "The finalized predecessor was explicitly revalidated at the immutable review candidate."],
  [%w[TPR-003], [835], "The release projection remains fail-closed and does not request closure on blocked rows."],
  [%w[TPR-004], [836], "Recursive Rust measurement and adversarial fixtures are retained."],
  [%w[TPR-005], [837], "Active boot paths and retired command surfaces are explicit and tested."],
  [%w[D833-EXEC-002], [856], "Release versions, native-v3 preflight, workspace lock parity, and retained registry compatibility were repaired."]
].freeze

def run!(*argv)
  out, err, status = Open3.capture3(*argv)
  abort(err) unless status.success?
  out
end

def git_blob(revision, path)
  run!("git", "show", "#{revision}:#{path}")
end

def pr_identity(number)
  doc = JSON.parse(run!("gh", "pr", "view", number.to_s, "--repo", REPOSITORY, "--json", "state,headRefOid,mergeCommit"))
  abort("PR ##{number} is not merged") unless doc.fetch("state") == "MERGED" && doc.dig("mergeCommit", "oid")
  [doc.fetch("headRefOid"), doc.dig("mergeCommit", "oid")]
end

review_path = File.join(ROOT, "candidate-review.json")
validation_path = File.join(ROOT, "candidate-validation.json")
identity_rows = ISSUES.to_h do |issue, (pull_request, paths)|
  head_sha, merge_sha = pr_identity(pull_request)
  artifacts = paths.map do |path|
    {"path" => path, "sha256" => Digest::SHA256.hexdigest(git_blob(head_sha, path))}
  end
  [issue, {"issue" => issue, "pull_request" => pull_request, "head_sha" => head_sha, "merge_sha" => merge_sha, "artifacts" => artifacts}]
end
abort("executed candidate validation is missing; run run-candidate-validation.rb") unless File.file?(validation_path)
validation_doc = JSON.parse(File.read(validation_path))
abort("candidate validation did not pass") unless validation_doc["outcome"] == "passed"

residuals = [
  {"id" => "MERGE-LINKAGE-001", "owner_issue" => 849, "target_milestone" => "v0.92.2", "status" => "operator_deferred", "proof_rows" => 4,
   "rationale" => "Publication-linkage proof remains explicitly deferred; it is not a behavioral pass.",
   "release_consequence" => "The four affected criteria remain unproved in v0.92.1."},
  {"id" => "EXECUTABLE-PROOF-ROWS", "owner_issue" => 852, "target_milestone" => "v0.92.2", "status" => "operator_deferred", "proof_rows" => 5,
   "rationale" => "Five evidence-linkage rows were moved by operator decision from superseded #851.",
   "release_consequence" => "The five rows remain unproved and must not be represented as passes."},
  {"id" => "PROOF-INSUFFICIENT-ROWS", "owner_issue" => 852, "target_milestone" => "v0.92.2", "status" => "operator_deferred", "proof_rows" => 7,
   "rationale" => "Five cloud-control and two execution-proof rows lack sufficient proof.",
   "release_consequence" => "The seven rows remain limitations, not behavioral passes."}
]

semantic_dispositions = GROUPS.map do |ids, issues, resolution|
  row = {
    "kind" => ids == ["D520-RET-001"] ? "fixed_with_deferred_proof" : "fixed",
    "source_finding_ids" => ids,
    "resolution" => resolution,
    "release_consequence" => "The listed source finding is dispositioned without promoting separately declared residual proof gaps to passes.",
    "remediation_issues" => issues
  }
  row["residuals"] = residuals if ids == ["D520-RET-001"]
  row
end
subject_path = File.join(ROOT, "review-subject.json")
subject_doc = {"schema" => "adl.v0921.remediation_review_subject.v1", "source_finding_count" => 25, "dispositions" => semantic_dispositions}
File.write(subject_path, JSON.pretty_generate(subject_doc) + "\n")
abort("candidate review is missing; commit review-subject.json and obtain exact-head review") unless File.file?(review_path)
review_doc = JSON.parse(File.read(review_path))
subject_digest = Digest::SHA256.file(subject_path).hexdigest
abort("candidate review does not bind the review-subject digest") unless review_doc["reviewed_content_sha256"] == subject_digest
reviewed_sha = review_doc.fetch("candidate_sha")
abort("candidate review SHA is not immutable") unless reviewed_sha.match?(/\A[0-9a-f]{40}\z/)
abort("review-subject at reviewed SHA differs from current subject") unless Digest::SHA256.hexdigest(git_blob(reviewed_sha, subject_path)) == subject_digest

remediations = identity_rows.to_h do |issue, identity|
  head_sha = identity.fetch("head_sha")
  merge_sha = identity.fetch("merge_sha")
  artifacts = identity.fetch("artifacts")
  review = {
    "basis" => "reviewed_subject_digest",
    "reviewer" => review_doc.fetch("reviewer"),
    "reviewed_sha" => reviewed_sha,
    "observed_pr_head_sha" => reviewed_sha,
    "reviewed_content_sha256" => subject_digest,
    "observed_at" => review_doc.fetch("observed_at"),
    "outcome" => review_doc.fetch("outcome"),
    "findings" => review_doc.fetch("findings"),
    "blockers" => review_doc.fetch("blockers"),
    "report_path" => review_path,
    "sha256" => Digest::SHA256.file(review_path).hexdigest
  }
  validation = [{
    "basis" => "executed_candidate",
    "evidence" => validation_path,
    "sha256" => Digest::SHA256.file(validation_path).hexdigest,
    "outcome" => validation_doc.fetch("outcome")
  }]
  row = identity.merge({"review" => review, "validation" => validation})
  row.merge!({"closure_kind" => "native_review_close", "closure_dependency_issue" => 522}) if issue == 833
  [issue, row]
end

dispositions = semantic_dispositions.map do |semantic|
  semantic.reject { |key, _| key == "remediation_issues" }.merge(
    "remediations" => semantic.fetch("remediation_issues").map { |issue| remediations.fetch(issue) }
  )
end
File.write(File.join(ROOT, "dispositions.json"), JSON.pretty_generate({"schema" => "adl.v0921.finding_dispositions.v3", "dispositions" => dispositions}) + "\n")

manifest_paths = Dir.glob(File.join(ROOT, "*.json")).reject { |path| path.end_with?("packet-manifest.json") }
manifest = {"schema" => "adl.v0921.remediation_packet_manifest.v1", "entries" => manifest_paths.sort.map { |path| {"path" => path, "sha256" => Digest::SHA256.file(path).hexdigest} }}
File.write(File.join(ROOT, "packet-manifest.json"), JSON.pretty_generate(manifest) + "\n")
