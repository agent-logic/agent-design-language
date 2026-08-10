# frozen_string_literal: true

require "json"
require "open3"

index = JSON.parse(File.read(".csdlc/issues/5909/index.json"))
abort "issue 5909 is not closed_out" unless index["phase"] == "closed_out"
terminal = index.fetch("terminal")
abort "wrong terminal PR" unless terminal["pull_request"] == 120
abort "wrong terminal disposition" unless terminal["disposition"] == "merged"
abort "wrong terminal head" unless terminal["observed_sha"] == "309d170dc569ca43fa74ce6d73d68857ce82c994"

git_common_dir, stderr, status = Open3.capture3(
  "git", "rev-parse", "--path-format=absolute", "--git-common-dir"
)
abort "cannot resolve Git common directory: #{stderr.strip}" unless status.success?
receipt_path = File.join(git_common_dir.strip, terminal.fetch("receipt_path"))
abort "missing terminal receipt: #{receipt_path}" unless File.file?(receipt_path)
receipt = JSON.parse(File.read(receipt_path))
abort "terminal receipt issue mismatch" unless receipt["issue"] == 5909
receipt_record = receipt.fetch("record")
abort "terminal receipt generation mismatch" unless receipt_record["generation"] == index["generation"]
abort "terminal receipt digest mismatch" unless receipt_record["digest"] == index["digest"]
abort "terminal receipt record mismatch" unless receipt_record == index

spp = JSON.parse(File.read(".csdlc/issues/5909/cards/spp.values.json"))
steps = spp.dig("content", "values", "steps") || []
abort "SPP still has an in-progress terminal step" if steps.any? { |step| step["status"] == "in_progress" }

sor = JSON.parse(File.read(".csdlc/issues/5909/cards/sor.values.json"))
abort "SOR card status is not complete" unless sor["status"] == "complete"
values = sor.dig("content", "values") || {}
expected = {
  "integration_state" => "merged",
  "publication_state" => "closed",
  "merge_state" => "merged",
  "closeout_state" => "complete"
}
expected.each do |field, value|
  abort "SOR #{field} mismatch" unless values[field] == value
end

puts "PASS: issue 5909 terminal records match merged PR 120"
