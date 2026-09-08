#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

root = Pathname.new(__dir__).join("../../../..").realpath
canary_root = Pathname.new(ENV.fetch("ADL_NO_V2_CANARY_ROOT", "/Volumes/FastWork/adl-worktrees/adl-issue-516-no-v2-canary"))
candidate = `git -C #{root} rev-parse origin/main`.strip
abort "invalid candidate" unless candidate.match?(/\A[0-9a-f]{40}\z/)
abort "dirty canary worktree" unless `git -C #{canary_root} status --porcelain`.empty?
system("git", "-C", canary_root.to_s, "checkout", "--detach", candidate, out: File::NULL, err: File::NULL) or abort "cannot select candidate"

v2 = canary_root.join("csdlc-v2")
disabled = canary_root.join("csdlc-v2.disabled")
abort "canary source missing" unless v2.directory?

started_at = Time.now.utc.iso8601(6)
stdout = stderr = nil
status = nil
begin
  File.rename(v2, disabled)
  stdout, stderr, status = Open3.capture3(
    "cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--no-run",
    chdir: canary_root.to_s
  )
ensure
  File.rename(disabled, v2) if disabled.exist?
end
abort "no-v2 canary failed: #{stderr.lines.last(20).join}" unless status&.success?

evidence_dir = root.join(".csdlc/evidence/516")
log_relative = ".csdlc/evidence/516/no-v2-canary-#{candidate[0, 8]}.stderr.log"
log = stderr.gsub(canary_root.to_s, "<canary-worktree>")
evidence_dir.join(File.basename(log_relative)).write(log)
receipt = {
  schema: "adl.v0921.no_v2_canary.v2",
  candidate: candidate,
  gap_owner_issue: 725,
  command: ["cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--no-run"],
  csdlc_v2_present_during_command: false,
  exit_status: status.exitstatus,
  started_at: started_at,
  ended_at: Time.now.utc.iso8601(6),
  source_dependencies: ["csdlc-v3/Cargo.toml", "csdlc-v3/src/authority.rs", "csdlc-v3/src/commands/remote/mod.rs"],
  sanitized_stderr: { path: log_relative, sha256: Digest::SHA256.hexdigest(log), bytes: log.bytesize }
}
evidence_dir.join("no-v2-canary-#{candidate[0, 8]}.json").write(JSON.pretty_generate(receipt) + "\n")
puts JSON.generate(schema: receipt[:schema], candidate: candidate, status: "pass")
