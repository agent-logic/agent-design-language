#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

def validate_matrix(matrix)
  %w[linux macos].each do |platform|
    row = matrix[platform] || raise("missing #{platform}")
    raise("#{platform} must be native live proof") unless row["qualification"] == "live" && row["native"] == true
    raise("#{platform} failed") unless row["outcome"] == "passed"
    %w[runner revision command_profile_digest result_digest receipt].each do |field|
      raise("#{platform} missing #{field}") if row[field].to_s.empty?
    end
  end

  windows = matrix["windows"] || raise("missing windows")
  raise("Windows qualification invalid") unless %w[live fixture].include?(windows["qualification"])
  if windows["qualification"] == "live" && windows["native"] != true
    raise("live Windows row is not native")
  end
  if windows["qualification"] == "fixture" && windows["native"] != false
    raise("fixture Windows row overclaims native proof")
  end
  %w[revision command_profile_digest result_digest receipt].each do |field|
    raise("windows missing #{field}") if windows[field].to_s.empty?
  end
  raise("windows failed") unless windows["outcome"] == "passed"
  "native Linux + macOS, Windows #{windows['qualification']}"
end

if ARGV == ["--self-test"]
  row = {
    "qualification" => "live",
    "native" => true,
    "runner" => "fixture",
    "revision" => "a" * 40,
    "command_profile_digest" => "b" * 64,
    "result_digest" => "c" * 64,
    "receipt" => "fixture.json",
    "outcome" => "passed"
  }
  windows = row.merge("qualification" => "fixture", "native" => false)
  validate_matrix("linux" => row, "macos" => row, "windows" => windows)
  blocked = row.merge("outcome" => "operator_required")
  begin
    validate_matrix("linux" => blocked, "macos" => row, "windows" => windows)
    abort "blocked Linux row unexpectedly passed"
  rescue RuntimeError => error
    abort error.message unless error.message == "linux failed"
  end
  puts "WP-06 platform matrix self-test passed"
  exit 0
end

root = Pathname.new(__dir__).join("../../../..").cleanpath
path = root.join(".csdlc/evidence/5823/platform-matrix.json")
abort "missing platform matrix" unless path.file? && !path.zero?

begin
  summary = validate_matrix(JSON.parse(path.read))
  puts "WP-06 platform matrix valid: #{summary}"
rescue RuntimeError => error
  abort error.message
end
