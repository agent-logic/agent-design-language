# frozen_string_literal: true

require "digest"
require "json"
require "open3"

module Wp04ProofReceiptContract
  module_function

  SHA256 = /\A[0-9a-f]{64}\z/
  GIT_OID = /\A[0-9a-f]{40,64}\z/

  def git(*args)
    stdout, stderr, status = Open3.capture3("git", *args)
    abort "git #{args.join(' ')} failed: #{stderr.strip}" unless status.success?
    stdout
  end

  def exact_commit(revision, label)
    abort "#{label} revision malformed" unless revision.to_s.match?(GIT_OID)
    resolved = git("rev-parse", "--verify", "#{revision}^{commit}").strip
    abort "#{label} revision is not exact" unless resolved == revision
    resolved
  end

  def ancestor?(older, newer)
    _stdout, _stderr, status = Open3.capture3("git", "merge-base", "--is-ancestor", older, newer)
    status.success?
  end

  def safe_repository_path?(path)
    !path.to_s.empty? && !path.start_with?("/") && !path.split("/").include?("..")
  end

  def digest_file(path, expected, label, allow_empty: false)
    abort "#{label} path must be repository-relative" unless safe_repository_path?(path)
    abort "missing #{label}: #{path}" unless File.file?(path)
    abort "empty #{label}: #{path}" if !allow_empty && File.zero?(path)
    abort "invalid #{label} digest" unless expected.to_s.match?(SHA256)
    abort "#{label} digest mismatch: #{path}" unless Digest::SHA256.file(path).hexdigest == expected
  end

  def validate_source_artifacts(artifacts, paths, revision)
    entries = Array(artifacts)
    abort "source artifacts missing" if entries.empty?
    abort "source artifact paths mismatch" unless entries.map { |entry| entry["path"] } == paths
    entries.each do |artifact|
      path = artifact.fetch("path")
      expected = artifact.fetch("sha256")
      abort "source artifact path must be repository-relative" unless safe_repository_path?(path)
      abort "invalid source artifact digest" unless expected.to_s.match?(SHA256)
      bytes = git("show", "#{revision}:#{path}")
      abort "source artifact digest mismatch: #{path}" unless Digest::SHA256.hexdigest(bytes) == expected
    end
  end

  def source_artifacts_match_revision?(artifacts, revision)
    Array(artifacts).all? do |artifact|
      path = artifact.fetch("path")
      expected = artifact.fetch("sha256")
      next false unless safe_repository_path?(path) && expected.to_s.match?(SHA256)
      stdout, _stderr, status = Open3.capture3("git", "show", "#{revision}:#{path}")
      status.success? && Digest::SHA256.hexdigest(stdout) == expected
    end
  end

  def validate_final_protected_artifacts(artifacts, paths, head)
    validate_source_artifacts(artifacts, paths, head)
    Array(artifacts).each do |artifact|
      digest_file(
        artifact.fetch("path"),
        artifact.fetch("sha256"),
        "final protected artifact"
      )
    end
  end

  def validate_v3_revisions(issue, proof, evidence_path, head)
    prefix = ".csdlc/evidence/#{issue}/"
    abort "execution proof path escapes issue evidence" unless safe_repository_path?(evidence_path) && evidence_path.start_with?(prefix)
    abort "wrong evidence revision strategy" unless proof["evidence_revision_strategy"] == "derive_from_receipt_introduction"
    abort "stored evidence revision is self-referential" if proof.key?("evidence_revision")
    source = exact_commit(proof["source_revision"], "source")
    unless ancestor?(source, head)
      merge_base, _stderr, status = Open3.capture3("git", "merge-base", source, head)
      abort "source revision has no shared ancestry with HEAD" unless status.success?
      merge_base = merge_base.strip
      candidates = git(
        "log", "--format=%H", "--reverse", "#{merge_base}..#{head}", "--", evidence_path
      ).lines.map(&:strip).reject(&:empty?).select do |revision|
        committed, _stderr, status = Open3.capture3("git", "show", "#{revision}:#{evidence_path}")
        status.success? && committed == File.binread(evidence_path) &&
          source_artifacts_match_revision?(proof["source_artifacts"], revision)
      end
      abort "squash-equivalent receipt introduction is not unique" unless candidates.length == 1
      evidence = exact_commit(candidates.fetch(0), "squash-equivalent evidence")
      later_touches = git(
        "log", "--format=%H", "#{evidence}..#{head}", "--", prefix, *Array(proof["protected_paths"])
      ).lines.map(&:strip).reject(&:empty?)
      abort "protected source or evidence changed after squash-equivalent introduction: #{later_touches.join(', ')}" unless later_touches.empty?
      return [source, evidence]
    end
    introductions = git(
      "log", "--format=%H", "--diff-filter=A", "--reverse", "#{source}..#{head}", "--", evidence_path
    ).lines.map(&:strip).reject(&:empty?)
    abort "receipt must have exactly one introduction commit after source" unless introductions.length == 1
    evidence = exact_commit(introductions.fetch(0), "evidence")
    abort "source revision is not an ancestor of evidence revision" unless ancestor?(source, evidence)
    abort "evidence revision is not an ancestor of HEAD" unless ancestor?(evidence, head)
    committed_receipt = git("show", "#{evidence}:#{evidence_path}")
    abort "receipt content differs from evidence revision" unless committed_receipt == File.binread(evidence_path)

    changed = git("diff", "--name-only", source, evidence, "--").lines.map(&:strip).reject(&:empty?)
    abort "source-to-evidence diff is empty" if changed.empty?
    escaped = changed.reject { |path| path.start_with?(prefix) }
    abort "source-to-evidence diff escapes issue evidence: #{escaped.join(', ')}" unless escaped.empty?
    later_touches = git("log", "--format=%H", "#{evidence}..#{head}", "--", prefix).lines.map(&:strip).reject(&:empty?)
    abort "evidence changed after its introduction: #{later_touches.join(', ')}" unless later_touches.empty?
    protected_touches = git(
      "log", "--format=%H", "#{evidence}..#{head}", "--", *Array(proof["protected_paths"])
    ).lines.map(&:strip).reject(&:empty?)
    abort "protected source changed after evidence introduction: #{protected_touches.join(', ')}" unless protected_touches.empty?
    [source, evidence]
  end

  def validate_runner(runner, label)
    abort "#{label} runner missing" unless runner.is_a?(Hash)
    %w[provider run_id os arch].each do |field|
      abort "#{label} runner #{field} missing" if runner[field].to_s.strip.empty?
    end
    abort "#{label} runner identity hash invalid" unless runner["identity_sha256"].to_s.match?(SHA256)
  end

  def validate_command(command, label)
    abort "#{label} command missing" unless command.is_a?(Hash)
    argv = Array(command["argv"])
    abort "#{label} argv missing" if argv.empty? || argv.any? { |part| part.to_s.empty? }
    abort "#{label} command failed" unless command["exit_code"] == 0
    abort "#{label} start time missing" if command["started_at"].to_s.empty?
    abort "#{label} finish time missing" if command["finished_at"].to_s.empty?
    validate_runner(command["runner"], label)
    digest_file(command["stdout_path"], command["stdout_sha256"], "#{label} stdout")
    digest_file(command["stderr_path"], command["stderr_sha256"], "#{label} stderr", allow_empty: true)
    argv
  end

  def validate_artifacts(artifacts, issue, label)
    entries = Array(artifacts)
    abort "#{label} artifacts missing" if entries.empty?
    entries.each do |artifact|
      path = artifact.fetch("path")
      abort "#{label} artifact escapes issue evidence: #{path}" unless path.start_with?(".csdlc/evidence/#{issue}/")
      digest_file(path, artifact.fetch("sha256"), "#{label} artifact")
    end
  end

  def validate_negative_cases(cases, issue)
    entries = Array(cases)
    abort "negative cases missing" if entries.empty?
    entries.each do |entry|
      abort "negative case name missing" if entry["case"].to_s.empty?
      abort "negative case has no proving result" unless %w[denied rejected fenced recovered fail_closed].include?(entry["result"])
      digest_file(entry["evidence_path"], entry["evidence_sha256"], "negative case #{entry['case']}")
      abort "negative evidence escapes issue evidence" unless entry["evidence_path"].start_with?(".csdlc/evidence/#{issue}/")
    end
  end

  def validate(issue:, wp:, paths:, test:, platforms:, required_commands: [])
    evidence_path = ARGV.fetch(0, ".csdlc/evidence/#{issue}/execution-proof.json")
    abort "missing execution proof: #{evidence_path}" unless File.file?(evidence_path)
    proof = JSON.parse(File.read(evidence_path))
    schema = proof["schema"]
    abort "wrong schema" unless %w[adl.wp04.execution_proof.v2 adl.wp04.execution_proof.v3].include?(schema)
    abort "wrong issue" unless proof["issue"] == issue
    abort "wrong WP" unless proof["wp"] == wp
    head = git("rev-parse", "HEAD").strip
    source_revision, evidence_revision = if schema == "adl.wp04.execution_proof.v2"
      source = exact_commit(proof["source_revision"], "source")
      abort "stale source revision" unless source == head
      [source, head]
    else
      validate_v3_revisions(issue, proof, evidence_path, head)
    end
    abort "protected path drift" unless proof["protected_paths"] == paths
    if schema == "adl.wp04.execution_proof.v3"
      validate_source_artifacts(proof["source_artifacts"], paths, source_revision)
      validate_final_protected_artifacts(proof["source_artifacts"], paths, head)
    end

    commands = Array(proof["commands"])
    test_commands = commands.select do |command|
      argv = validate_command(command, "command")
      argv.include?(test) && argv.include?("--no-tests=fail") && command["selected_tests"].to_i.positive?
    end
    abort "missing one nonzero exact test command #{test}" unless test_commands.length == 1
    required_commands.each do |required|
      matches = commands.select { |command| Array(command["argv"]) == required }
      abort "missing exact proving command #{required.join(' ')}" unless matches.length == 1
    end
    validate_negative_cases(proof["negative_cases"], issue)
    validate_artifacts(proof["artifacts"], issue, "execution proof")

    receipts = Array(proof["native_receipts"])
    abort "unexpected native receipt denominator" unless receipts.map { |entry| entry["platform"] }.sort == platforms.sort
    receipts.each do |receipt|
      platform = receipt.fetch("platform")
      abort "stale native receipt for #{platform}" unless receipt["source_revision"] == source_revision
      validate_command(receipt["command"], "#{platform} native")
      validate_artifacts(receipt["artifacts"], issue, "#{platform} native")
    end
    run_ids = receipts.map { |receipt| receipt.dig("command", "runner", "run_id") }
    abort "native runner runs are not distinct" unless run_ids.uniq.length == run_ids.length
    puts "PASS: #{wp} source #{source_revision} evidence #{evidence_revision} logs, artifacts, negatives, and native receipts"
  end
end
