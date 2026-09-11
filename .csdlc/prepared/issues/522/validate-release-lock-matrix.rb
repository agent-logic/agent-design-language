#!/usr/bin/env ruby
require "json"
require "open3"

inventory = JSON.parse(File.read("docs/milestones/v0.92.1/RELEASE_ARTIFACTS.json"))
lockfiles = inventory.fetch("lockfiles")
abort("release lockfile denominator is not exactly 11") unless lockfiles.length == 11 && lockfiles.uniq.length == 11

validated = lockfiles.map do |lockfile|
  manifest = File.join(File.dirname(lockfile), "Cargo.toml")
  abort("active lock owner missing: #{manifest}") unless File.file?(manifest)
  stdout, stderr, status = Open3.capture3("cargo", "metadata", "--locked", "--format-version", "1", "--manifest-path", manifest)
  abort("locked metadata failed for #{manifest}: #{stderr}") unless status.success?
  metadata = JSON.parse(stdout)
  abort("metadata has no packages for #{manifest}") if metadata.fetch("packages").empty?
  {"lockfile" => lockfile, "manifest" => manifest, "packages" => metadata.fetch("packages").length}
end

puts JSON.generate({"status" => "passed", "validated" => validated.length, "locks" => validated})
