#!/usr/bin/env ruby
# frozen_string_literal: true

ROOT = ENV.fetch("ADL_RELEASE_VALIDATION_ROOT", File.expand_path("../../../..", __dir__))
EXPECTED = "0.92.1"
MANIFESTS = %w[
  adl/Cargo.toml
  adl-runtime/Cargo.toml
  adl-runtime-kernel/Cargo.toml
  adl-resilience/Cargo.toml
  adl-characterization/Cargo.toml
  adl-v2/Cargo.toml
  csdlc-v2/Cargo.toml
  tools/remote_validation/Cargo.toml
].freeze
LOCKS = %w[
  adl/Cargo.lock
  adl-runtime/Cargo.lock
  adl-runtime-kernel/Cargo.lock
  adl-resilience/Cargo.lock
  adl-characterization/Cargo.lock
  adl-v2/Cargo.lock
  adl-v2/crates/adl-runtime-v3-adapter/Cargo.lock
  csdlc-v2/Cargo.lock
  tools/remote_validation/Cargo.lock
].freeze
EXPECTED_LOCK_PACKAGES = {
  "adl/Cargo.lock" => %w[adl adl-resilience adl-runtime adl-runtime-kernel],
  "adl-runtime/Cargo.lock" => %w[adl-resilience adl-runtime adl-runtime-kernel],
  "adl-runtime-kernel/Cargo.lock" => %w[adl-runtime-kernel],
  "adl-resilience/Cargo.lock" => %w[adl-resilience],
  "adl-characterization/Cargo.lock" => %w[adl-characterization],
  "adl-v2/Cargo.lock" => %w[adl-adapters adl-cli adl-compiler adl-engine adl-language adl-records adl-runtime-kernel adl-runtime-v3-adapter adl-workcell-conductor adl-workcell-convergence adl-workcell-task-adapter],
  "adl-v2/crates/adl-runtime-v3-adapter/Cargo.lock" => %w[adl-runtime-kernel],
  "csdlc-v2/Cargo.lock" => %w[adl-resilience csdlc-v2],
  "tools/remote_validation/Cargo.lock" => %w[adl-remote-validation]
}.freeze

errors = []
MANIFESTS.each do |relative|
  path = File.join(ROOT, relative)
  version = File.foreach(path).lazy.map { |line| line[/\Aversion\s*=\s*"([^"]+)"/, 1] }.find(&:itself)
  errors << "#{relative}: expected #{EXPECTED}, found #{version || 'missing'}" unless version == EXPECTED
end
LOCKS.each do |relative|
  path = File.join(ROOT, relative)
  unless File.file?(path)
    errors << "#{relative}: missing lockfile"
    next
  end

  local_packages = File.read(path).split(/^\[\[package\]\]\s*$/).drop(1).reject { |package| package.match?(/^source\s*=/) }
  errors << "#{relative}: no local packages found" if local_packages.empty?
  parsed = local_packages.map do |package|
    [package[/^name\s*=\s*"([^"]+)"/, 1], package[/^version\s*=\s*"([^"]+)"/, 1]]
  end
  parsed.each do |name, version|
    errors << "#{relative}: local package #{name || 'unknown'} retained version 0.92.0" if version == "0.92.0"
  end
  EXPECTED_LOCK_PACKAGES.fetch(relative).each do |expected_name|
    matches = parsed.select { |name, _version| name == expected_name }
    errors << "#{relative}: expected exactly one local #{expected_name} package, found #{matches.length}" unless matches.length == 1
    next unless matches.length == 1

    version = matches.first.last
    errors << "#{relative}: local package #{expected_name} expected #{EXPECTED}, found #{version || 'missing'}" unless version == EXPECTED
  end
end

abort errors.join("\n") unless errors.empty?
puts "PASS: 8 release-bearing Cargo manifests and 9 lockfiles consistently use v#{EXPECTED}"
