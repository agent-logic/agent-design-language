#!/usr/bin/env ruby

require "json"
require "digest"
require "time"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, ".csdlc/evidence/35")

def abort_unless(condition, message)
  abort(message) unless condition
end

def read_json(path)
  JSON.parse(File.read(path))
rescue Errno::ENOENT, JSON::ParserError => error
  abort("#{path}: #{error.message}")
end

def validate_evidence
  reproduction_path = File.join(EVIDENCE, "background-task-dispatch-reproduction.json")
  ownership_path = File.join(EVIDENCE, "ownership-reconciliation.json")
  inventory_path = File.join(EVIDENCE, "task-inventory-receipts.json")
  readback_path = File.join(EVIDENCE, "task-readback-receipt.json")
  reproduction = read_json(reproduction_path)
  ownership = read_json(ownership_path)
  inventory = read_json(inventory_path)
  readback_receipt = read_json(readback_path)

  abort_unless(reproduction["schema"] == "codex.background_task_dispatch.reproduction.v1", "reproduction schema")
  abort_unless(ownership["schema"] == "codex.background_task_dispatch.ownership.v1", "ownership schema")
  abort_unless(inventory["schema"] == "codex.background_task_dispatch.inventory.v1", "inventory schema")
  abort_unless(readback_receipt["schema"] == "codex.background_task_dispatch.readback.v1", "readback schema")

  canary_id = reproduction["canary_id"]
  prompt_digest = reproduction["prompt_digest"]
  abort_unless(canary_id.is_a?(String) && !canary_id.empty?, "canary id")
  abort_unless(prompt_digest.to_s.match?(/\A[0-9a-f]{64}\z/), "prompt digest")
  abort_unless(ownership.values_at("canary_id", "prompt_digest") == [canary_id, prompt_digest], "ownership identity")

  attempts = reproduction["attempts"]
  abort_unless(attempts.is_a?(Array) && attempts.size == 2, "attempt denominator")
  abort_unless(attempts.map { |attempt| attempt["request_mode"] } == %w[project_discovery projectless_task_create], "attempt order")
  attempts.each do |attempt|
    abort_unless(attempt["timeout_seconds"] == 120, "timeout")
    elapsed = attempt["elapsed_seconds"]
    abort_unless(elapsed.is_a?(Numeric) && elapsed.between?(0, 120), "elapsed time")
  end

  discovery, dispatch = attempts
  discovery_results = %w[project_found typed_failure timeout indeterminate]
  abort_unless(discovery_results.include?(discovery["terminal_result"]), "discovery result")
  if discovery["terminal_result"] == "project_found"
    abort_unless(!discovery["returned_project_id"].to_s.empty?, "project identity")
  else
    abort_unless(!discovery["diagnostic_class"].to_s.empty?, "discovery diagnostic")
  end

  dispatch_results = %w[created typed_failure timeout indeterminate]
  result = dispatch["terminal_result"]
  abort_unless(dispatch_results.include?(result), "dispatch result")
  abort_unless(!dispatch["diagnostic_class"].to_s.empty?, "dispatch diagnostic") unless result == "created"

  abort_unless(ownership["inventory_receipt_sha256"] == Digest::SHA256.file(inventory_path).hexdigest, "inventory receipt digest")
  abort_unless(ownership["readback_receipt_sha256"] == Digest::SHA256.file(readback_path).hexdigest, "readback receipt digest")
  snapshots = inventory["snapshots"]
  abort_unless(snapshots.is_a?(Array) && snapshots.map { |snapshot| snapshot["phase"] } == %w[pre_dispatch post_dispatch], "inventory snapshots")
  snapshot_ids = snapshots.map do |snapshot|
    abort_unless(snapshot["source_operation"] == "codex.list_threads", "inventory source")
    Time.iso8601(snapshot.fetch("observed_at"))
    pages = snapshot["pages"]
    abort_unless(pages.is_a?(Array) && !pages.empty?, "inventory pages")
    expected_cursor = nil
    ids = []
    pages.each do |page|
      abort_unless(page["request_cursor"] == expected_cursor, "inventory cursor chain")
      tasks = page["tasks"]
      abort_unless(tasks.is_a?(Array), "inventory tasks")
      ids.concat(tasks.map { |task| task["task_id"] })
      expected_cursor = page["next_cursor"]
    end
    abort_unless(expected_cursor.nil? && snapshot["pagination_complete"] == true, "inventory completeness")
    abort_unless(ids.all? { |id| id.is_a?(String) && !id.empty? } && ids == ids.uniq, "inventory task identity")
    ids
  rescue ArgumentError, KeyError => error
    abort("inventory observation: #{error.message}")
  end
  pre_ids, post_ids = snapshot_ids
  abort_unless(ownership.values_at("pre_task_ids", "post_task_ids") == [pre_ids, post_ids], "ownership inventory projection")
  [pre_ids, post_ids].each do |ids|
    abort_unless(ids.is_a?(Array) && ids == ids.uniq, "task inventory")
    abort_unless(ids.all? { |id| id.is_a?(String) && !id.empty? }, "task identity")
  end
  delta = post_ids - pre_ids
  abort_unless(ownership["derived_new_task_ids"] == delta, "derived task delta")

  if result == "created"
    task_id = dispatch["returned_task_id"]
    abort_unless(readback_receipt["observed"] == true && readback_receipt["source_operation"] == "codex.read_thread", "created readback observation")
    Time.iso8601(readback_receipt.fetch("observed_at"))
    readback = readback_receipt["task"]
    abort_unless(task_id.is_a?(String) && delta == [task_id], "created task delta")
    abort_unless(readback.is_a?(Hash), "created task readback")
    abort_unless(readback.values_at("task_id", "canary_id", "prompt_digest") == [task_id, canary_id, prompt_digest], "readback identity")
    abort_unless(readback.values_at("repository", "issue", "worktree") == [nil, nil, nil], "canary owns repository state")
    abort_unless(ownership["ownership_disposition"] == "transferred_to_canary", "created ownership")
    abort_unless(ownership["retry_allowed"] == false, "created retry")
  else
    abort_unless(dispatch["returned_task_id"].nil? && delta.empty?, "failed dispatch created a task")
    abort_unless(readback_receipt["observed"] == false && readback_receipt["source_operation"].nil? && readback_receipt["task"].nil?, "failed dispatch readback")
    abort_unless(ownership["ownership_disposition"] == "retained_by_caller", "failed ownership")
    expected_retry = result == "typed_failure"
    abort_unless(ownership["retry_allowed"] == expected_retry, "reconciled retry")
  end

  text = [reproduction_path, ownership_path, inventory_path, readback_path].map { |path| File.read(path) }.join
  abort("machine-local path") if text.match?(%r{/Users/|/Volumes/|/private/tmp})
  abort("secret-shaped content") if text.match?(/BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|authorization\s*[:=]|access[_-]?token\s*[:=]|password\s*[:=]/i)
  puts "PASS: dispatch ownership contract"
end

def validate_docs
  required = {
    "docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH.md" => ["## Success Rule", "## Reconciliation", "## Retry Rule", "## Escalation"],
    "docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH_UPSTREAM_REPORT.md" => ["## Expected Behavior", "## Reproduction", "## Observed Result", "## Inventory Reconciliation", "## Ownership Route", "## Non-Claims"]
  }
  required.each do |relative_path, headings|
    text = File.read(File.join(ROOT, relative_path))
    headings.each_with_index do |heading, index|
      start = text.index(heading)
      abort("missing #{relative_path}: #{heading}") unless start
      finish = index + 1 < headings.size ? text.index(headings[index + 1], start + heading.length) : text.length
      body = text[(start + heading.length)...finish].to_s.strip
      abort("empty #{relative_path}: #{heading}") if body.length < 40
    end
  end
  report = File.read(File.join(ROOT, "docs/tooling/CODEX_BACKGROUND_TASK_DISPATCH_UPSTREAM_REPORT.md"))
  %w[background-task-dispatch-reproduction.json ownership-reconciliation.json task-inventory-receipts.json task-readback-receipt.json diagnostic_class ownership_disposition].each do |required_value|
    abort("upstream report missing #{required_value}") unless report.include?(required_value)
  end
  puts "PASS: operator report contract"
end

def validate_review
  index = read_json(File.join(ROOT, ".csdlc/issues/35/index.json"))
  assignment = index["review_assignment"]
  review = index["review"]
  abort_unless(assignment.is_a?(Hash) && review.is_a?(Hash), "canonical review missing")
  reviewer = assignment["reviewer"]
  abort_unless(reviewer.to_s.start_with?("subagent:") && assignment["assigned_by"] != reviewer, "independent reviewer")
  abort_unless(review["reviewer"] == reviewer && review["reviewed_revision"] == assignment["revision"], "review assignment mismatch")
  abort_unless(review["completed"] == true && review["findings"] == [], "review result")
  required_scope = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues/35/cards/spp.values.json"))).dig("content", "values", "affected_areas")
  abort_unless((required_scope - review["scope"]).empty?, "review scope incomplete")
  puts "PASS: canonical independent review"
end

case ARGV.fetch(0, "")
when "evidence" then validate_evidence
when "docs" then validate_docs
when "review" then validate_review
else abort("usage: validate-dispatch-evidence.rb evidence|docs|review")
end
