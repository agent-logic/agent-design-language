#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
BASE = "4460ec8157da7a53decf28f41e20af8afd19f611"
MAP_RELATIVE = ".csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json"
MAP_SHA256 = "9a6d7834557f626487aae3115464ee60f19b06609b7ea9e6a24399a60eec8745"
PREFIX = ".csdlc/evidence/208/v4/"
OUTPUT = ROOT.join(PREFIX)
PROTECTED = %w[
  adl/.config/nextest.toml
  adl-runtime-kernel/Cargo.toml adl-runtime-kernel/Cargo.lock
  adl-runtime-kernel/src/continuity_control.rs adl-runtime-kernel/src/assembly.rs
  adl-runtime-kernel/src/bin/adl-runtime-kernel.rs adl-runtime-kernel/src/config.rs
  adl-runtime-kernel/src/governance.rs adl-runtime-kernel/src/ingress.rs adl-runtime-kernel/src/lib.rs
  adl-runtime-kernel/src/operations.rs adl-runtime-kernel/src/reasoning.rs
  adl-runtime-kernel/tests/configuration.rs adl-runtime-kernel/tests/kernel_continuity_control.rs
  adl-runtime-kernel/tests/production_acip_wss.rs adl-runtime-kernel/tests/support/runtime_init.rs
  adl-runtime/Cargo.toml adl-runtime/Cargo.lock adl-runtime/src/kernel_continuity_client.rs
  adl-runtime/src/bin/adl-runtime-guardian.rs adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
  adl-runtime/src/distributed/polis_runtime.rs
  adl-runtime/src/guardian.rs adl-runtime/src/lib.rs adl-runtime/tests/guardian_cli.rs
  adl-runtime/tests/kernel_continuity_client.rs
  infra/runtime-v3/runtime-init.toml
  .csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json
  .csdlc/prepared/issues/208/verify-nextest-workspace-contract.rb
  .csdlc/prepared/issues/208/verify-diff-hygiene.rb
  .csdlc/prepared/issues/208/produce-proof-receipt.rb
  .csdlc/prepared/issues/208/validate-proof-receipt.rb
].freeze

def fail_proof(message)
  abort("issue 208 producer: #{message}")
end

def run_command(name, argv, env = {})
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3(env, *argv, chdir: ROOT.to_s)
  finished = Time.now.utc.iso8601(6)
  stdout = stdout.rstrip + (stdout.empty? ? "" : "\n")
  stderr = stderr.rstrip + (stderr.empty? ? "" : "\n")
  File.binwrite(OUTPUT.join("#{name}.stdout.log"), stdout)
  File.binwrite(OUTPUT.join("#{name}.stderr.log"), stderr)
  {
    "argv" => argv, "exit_code" => status.exitstatus, "started_at" => started,
    "finished_at" => finished, "stdout_path" => "#{PREFIX}#{name}.stdout.log",
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_path" => "#{PREFIX}#{name}.stderr.log",
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

def run_concurrent_nextest_wave(commands, suffix = "")
  runtime_name = "runtime-nextest#{suffix}"
  kernel_name = "kernel-nextest#{suffix}"
  runtime = Thread.new do
    run_command(runtime_name, %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client --no-tests=fail])
  end
  kernel = Thread.new do
    run_command(kernel_name, %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime-kernel/Cargo.toml --test kernel_continuity_control --no-tests=fail])
  end
  commands[runtime_name.tr("-", "_")] = runtime.value
  commands[kernel_name.tr("-", "_")] = kernel.value
end

map_path = ROOT.join(MAP_RELATIVE)
fail_proof("map digest drift") unless Digest::SHA256.file(map_path).hexdigest == MAP_SHA256
map = JSON.parse(File.binread(map_path))
cases = map.fetch("cases")
boundaries = map.fetch("boundaries").flat_map { |row| row.fetch("subassertions") }
lifecycle = map.fetch("lifecycle_subassertions")
fail_proof("case contract drift") unless map["case_count"] == 56 && cases.length == 56 &&
  cases.map { |row| row["ordinal"] } == (1..56).to_a && cases.map { |row| row["name"] }.uniq.length == 56 &&
  cases.all? { |row| row["outcome"] == "proved" && row["marker"] == "proved:case:#{row['name']}" }
fail_proof("boundary contract drift") unless map["boundary_row_count"] == 8 && map["subassertion_count"] == 64 && boundaries.length == 64
fail_proof("lifecycle contract drift") unless map["lifecycle_subassertion_count"] == 12 && lifecycle.length == 12

source, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_proof("cannot resolve source") unless status.success? && source.strip.match?(/\A[0-9a-f]{40}\z/)
source = source.strip
dirty, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all", chdir: ROOT.to_s)
dirty_lines = dirty.lines.reject do |line|
  path = line[3..]&.strip
  path&.start_with?(PREFIX) || path == ".csdlc/locks/208.lock"
end
fail_proof("source worktree must be clean") unless status.success? && dirty_lines.empty?
PROTECTED.each do |relative|
  path = ROOT.join(relative)
  fail_proof("unsafe protected path: #{relative}") unless path.file? && !path.symlink?
  committed, committed_status = Open3.capture2("git", "show", "#{source}:#{relative}", chdir: ROOT.to_s)
  fail_proof("protected path absent at source: #{relative}") unless committed_status.success?
  fail_proof("protected path dirty: #{relative}") unless Digest::SHA256.hexdigest(committed) == Digest::SHA256.file(path).hexdigest
end
FileUtils.mkdir_p(OUTPUT, mode: 0o700)

commands = {}
run_concurrent_nextest_wave(commands)
run_concurrent_nextest_wave(commands, "-repeat")
commands["runtime_nextest_isolated"] = run_command("runtime-nextest-isolated", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client --no-tests=fail])
commands["kernel_nextest_isolated"] = run_command("kernel-nextest-isolated", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime-kernel/Cargo.toml --test kernel_continuity_control --no-tests=fail])
commands["runtime_nextest_isolated_repeat"] = run_command("runtime-nextest-isolated-repeat", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client --no-tests=fail])
commands["kernel_nextest_isolated_repeat"] = run_command("kernel-nextest-isolated-repeat", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime-kernel/Cargo.toml --test kernel_continuity_control --no-tests=fail])
commands["production_acip_nextest"] = run_command("production-acip-nextest", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime-kernel/Cargo.toml --test production_acip_wss --no-tests=fail])
commands["guardian_cli_nextest"] = run_command("guardian-cli-nextest", %w[cargo nextest run --config-file adl/.config/nextest.toml --locked --manifest-path adl-runtime/Cargo.toml --test guardian_cli --no-tests=fail])
commands["nextest_workspace_contract"] = run_command("nextest-workspace-contract", %w[ruby .csdlc/prepared/issues/208/verify-nextest-workspace-contract.rb])
commands["runtime_clippy"] = run_command("runtime-clippy", %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --bin adl-runtime-guardian --test guardian_cli --test kernel_continuity_client -- -D warnings])
commands["kernel_clippy"] = run_command("kernel-clippy", %w[cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel --test kernel_continuity_control --test production_acip_wss -- -D warnings])
commands["diff_hygiene"] = run_command("diff-hygiene", %w[ruby .csdlc/prepared/issues/208/verify-diff-hygiene.rb], {"ISSUE_208_EXECUTION_BASE" => BASE, "ISSUE_208_PROVING_SOURCE" => source})
commands["runtime_markers"] = run_command("runtime-markers", %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client -- --nocapture --test-threads=1])
commands["kernel_markers"] = run_command("kernel-markers", %w[cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test kernel_continuity_control -- --nocapture --test-threads=1])
failed = commands.select { |_name, command| command["exit_code"] != 0 }.keys
fail_proof("commands failed: #{failed.join(', ')}") unless failed.empty?
nextest_names = %w[
  runtime_nextest kernel_nextest runtime_nextest_repeat kernel_nextest_repeat
  runtime_nextest_isolated kernel_nextest_isolated
  runtime_nextest_isolated_repeat kernel_nextest_isolated_repeat guardian_cli_nextest
]
nextest_text = nextest_names.flat_map { |name| %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands[name]["#{stream}_path"])) } }.join
fail_proof("nextest denominator mismatch") unless %w[
  runtime_nextest runtime_nextest_repeat runtime_nextest_isolated runtime_nextest_isolated_repeat
].all? { |name|
  %w[stdout stderr].any? { |stream| File.binread(ROOT.join(commands[name]["#{stream}_path"])).include?("21 tests run: 21 passed") }
} && %w[
  kernel_nextest kernel_nextest_repeat kernel_nextest_isolated kernel_nextest_isolated_repeat
].all? { |name|
  %w[stdout stderr].any? { |stream| File.binread(ROOT.join(commands[name]["#{stream}_path"])).include?("35 tests run: 35 passed") }
}
fail_proof("isolated/concurrent nextest process leak") if nextest_text.include?("LEAK")
fail_proof("production ACIP denominator mismatch") unless %w[stdout stderr].any? { |stream|
  File.binread(ROOT.join(commands["production_acip_nextest"]["#{stream}_path"])).include?("2 tests run: 2 passed")
}
fail_proof("Guardian CLI denominator mismatch") unless %w[stdout stderr].any? { |stream|
  File.binread(ROOT.join(commands["guardian_cli_nextest"]["#{stream}_path"])).include?("3 tests run: 3 passed")
}
config_contract_text = %w[stdout stderr].map { |stream|
  File.binread(ROOT.join(commands["nextest_workspace_contract"]["#{stream}_path"]))
}.join
fail_proof("nextest workspace/slow-shard contract missing") unless config_contract_text.include?("PASS: nextest workspace and slow-proof selections remain loadable")
marker_text = %w[runtime_markers kernel_markers].flat_map { |name| %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands[name]["#{stream}_path"])) } }.join
fail_proof("behavior evidence leaked a forbidden LEAK sentinel") if marker_text.include?("LEAK")
receipts = marker_text.lines.map do |line|
  payload = line[/BEHAVIOR_RECEIPT (\{.*\})\s*\z/, 1]
  next unless payload
  JSON.parse(payload)
rescue JSON::ParserError
  fail_proof("malformed behavior receipt")
end.compact
fail_proof("behavior receipt denominator mismatch") unless receipts.length == 56 && receipts.map { |receipt| receipt["case"] }.uniq.length == 56
receipts.each do |receipt|
  fail_proof("behavior receipt schema/outcome mismatch") unless receipt["schema"] == "adl.issue208.behavior_receipt.v1" && receipt["outcome"] == "passed"
  behavior = receipt.fetch("behavior")
  fail_proof("behavior receipt case binding mismatch") unless behavior["case"] == receipt["case"] && behavior["assertion_binding"].to_s.end_with?("::#{receipt['case']}")
  canonical = receipt.fetch("behavior_canonical")
  fail_proof("behavior receipt canonical mismatch") unless JSON.parse(canonical) == behavior
  fail_proof("behavior receipt digest mismatch") unless Digest::SHA256.hexdigest(canonical) == receipt["behavior_sha256"]
  fail_proof("behavior receipt lacks durable witness") unless behavior["durable_witness"].is_a?(Array) && !behavior["durable_witness"].empty?
end
markers = receipts.flat_map { |receipt| receipt.fetch("markers") }
expected = cases.map { |row| row.fetch("marker") } + boundaries.map { |row| row.fetch("marker") } + lifecycle.map { |row| row.fetch("marker") }
fail_proof("marker denominator/parity mismatch") unless markers.sort == expected.sort && markers.uniq.length == expected.length
main_revision, main_status = Open3.capture2("git", "rev-parse", "origin/main", chdir: ROOT.to_s)
fail_proof("cannot resolve current origin/main") unless main_status.success? && main_revision.strip.match?(/\A[0-9a-f]{40}\z/)
system("git", "merge-base", "--is-ancestor", main_revision.strip, source, chdir: ROOT.to_s, out: File::NULL, err: File::NULL) || fail_proof("current origin/main is not ancestral to source")
tree, status = Open3.capture2("git", "rev-parse", "#{source}^{tree}", chdir: ROOT.to_s)
fail_proof("source tree unavailable") unless status.success?
proof = {
  "schema" => "adl.issue208.guardian_kernel_continuity_proof.v4", "issue" => 208,
  "execution_base_revision" => BASE, "main_revision" => main_revision.strip,
  "source_revision" => source, "source_tree" => tree.strip,
  "produced_at" => Time.now.utc.iso8601(6),
  "map" => {"path" => MAP_RELATIVE, "sha256" => MAP_SHA256, "case_count" => 56, "boundary_row_count" => 8, "subassertion_count" => 64, "lifecycle_subassertion_count" => 12},
  "protected_files" => PROTECTED.map { |path| {"path" => path, "sha256" => Digest::SHA256.file(ROOT.join(path)).hexdigest} },
  "commands" => commands, "cases" => cases, "boundary_subassertions" => boundaries,
  "lifecycle_subassertions" => lifecycle,
  "behavior_receipts" => receipts.sort_by { |receipt| cases.index { |row| row["name"] == receipt["case"] } }
}
File.binwrite(OUTPUT.join("execution-proof.json"), JSON.generate(proof) + "\n")
puts "PASS: produced exact issue #208 56-case/64-boundary/12-lifecycle proof at #{source}"
