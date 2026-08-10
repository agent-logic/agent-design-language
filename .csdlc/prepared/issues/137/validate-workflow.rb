#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"

ROOT = File.expand_path("../../../..", __dir__)
WORKFLOW = File.join(ROOT, ".github/workflows/wp04-native-distributed.yml")
SOURCE = File.read(WORKFLOW)

def assert_contract(condition, message)
  abort("workflow contract failed: #{message}") unless condition
end

def require_text(text)
  assert_contract(SOURCE.include?(text), "missing #{text.inspect}")
end

assert_contract(SOURCE.scan(/^  workflow_dispatch:$/).length == 1, "workflow_dispatch denominator")
assert_contract(!SOURCE.match?(/^  (pull_request|push|schedule):$/), "unexpected automatic trigger")
require_text("source_sha:")
require_text("required: true")
require_text('if [[ ! "$SOURCE_SHA" =~ ^[0-9a-f]{40}$ ]]')
assert_contract(SOURCE.scan('DISPATCH_SHA: ${{ github.sha }}').length == 2,
                "dispatch SHA environment denominator")
assert_contract(SOURCE.scan('if [[ "$SOURCE_SHA" != "$DISPATCH_SHA" ]]').length == 2,
                "dispatch SHA equality gate denominator")
assert_contract(SOURCE.scan('ref: ${{ inputs.source_sha }}').length == 2, "exact checkout denominator")
assert_contract(SOURCE.scan('test "$(git rev-parse HEAD)" = "$SOURCE_SHA"').length == 2,
                "post-checkout revision verification denominator")

%w[linux macos windows].each do |platform|
  require_text("- platform: #{platform}")
end
%w[ubuntu-latest macos-latest windows-latest].each do |runner|
  require_text("os: #{runner}")
end
assert_contract(SOURCE.scan("- platform:").length == 3, "platform denominator")
require_text('name: distributed-guardian-native-${{ matrix.platform }}')
require_text("timeout-minutes: 20")
require_text("timeout-minutes: 10")
require_text("fail-fast: false")
require_text("permissions:\n  contents: read\n  actions: read")

producer = SOURCE.split("  produce-native-receipt:\n", 2).fetch(1).split("  validate-native-receipts:\n", 2).fetch(0)
aggregate = SOURCE.split("  validate-native-receipts:\n", 2).fetch(1)
[producer, aggregate].each do |job|
  gate = job.index('if [[ "$SOURCE_SHA" != "$DISPATCH_SHA" ]]')
  checkout = job.index("uses: actions/checkout@")
  assert_contract(gate && checkout && gate < checkout, "dispatch SHA gate must precede checkout")
end
token = aggregate.index('GITHUB_TOKEN: ${{ github.token }}')
gate = aggregate.index('if [[ "$SOURCE_SHA" != "$DISPATCH_SHA" ]]')
assert_contract(token && gate && gate < token, "dispatch SHA gate must precede token exposure")

action_refs = SOURCE.scan(%r{uses: [^@\s]+@([^\s]+)}).flatten
assert_contract(action_refs.length == 7, "pinned action-use denominator")
assert_contract(action_refs.all? { |revision| revision.match?(/\A[0-9a-f]{40}\z/) },
                "every action must use a full commit pin")
require_text("tool: nextest@0.9.140")
require_text("fallback: none")
require_text("run: bash adl/tools/validate_v092_distributed_guardian.sh")

require_text('name: wp04-native-distributed-${{ matrix.platform }}-${{ github.run_id }}-${{ github.run_attempt }}')
require_text('pattern: wp04-native-distributed-*-${{ github.run_id }}-${{ github.run_attempt }}')
require_text("path: .csdlc/evidence/5878/native/")
require_text("if-no-files-found: error")
require_text("merge-multiple: true")
require_text("needs: produce-native-receipt")
require_text('test -s ".csdlc/evidence/5878/native/$platform/receipt.json"')
require_text('GITHUB_TOKEN: ${{ github.token }}')
require_text("run: ruby adl/tools/validate_v092_distributed_native_receipts.rb")

stdout, stderr, status = Open3.capture3("bash", "adl/tools/test_ci_path_policy.sh", chdir: ROOT)
unless status.success?
  warn(stdout)
  warn(stderr)
  abort("repository path-policy contract failed")
end

puts "PASS: WP-04 native workflow exact-SHA, matrix, receipt, attestation, and path-policy contracts"
