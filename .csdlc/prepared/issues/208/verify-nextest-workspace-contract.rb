#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
CONFIG = ROOT.join("adl/.config/nextest.toml")
MAP = ROOT.join(".csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json")
PRODUCER = ROOT.join(".csdlc/prepared/issues/208/produce-proof-receipt.rb")

def fail_contract(message)
  abort("issue 208 nextest workspace contract: #{message}")
end

config = File.binread(CONFIG)
cases = JSON.parse(File.binread(MAP)).fetch("cases").map { |entry| entry.fetch("name") }
fail_contract("binary selector can reject unrelated package selections") if config.include?("binary(")
fail_contract("override denominator drift") unless config.scan(/^\[\[profile\.default\.overrides\]\]$/).length == 1
filter = config.lines.grep(/^filter = /)
fail_contract("filter denominator drift") unless filter.length == 1
alternatives = filter.first[/test\(\/\^\((.*)\)\$\/\)/, 1]&.split("|")
fail_contract("filter is not the exact ordered issue case set") unless alternatives == cases
fail_contract("global leak policy drift") unless config.include?("leak-timeout = \"100ms\"")
fail_contract("tracked leak policy must remain fail-closed") unless config.include?('leak-timeout = { period = "5s", result = "fail" }')
producer = File.binread(PRODUCER)
fail_contract("standalone proof lanes must load the tracked config explicitly") unless producer.scan("nextest run --config-file adl/.config/nextest.toml").length == 8

puts "PASS: nextest workspace and slow-proof selections remain loadable without absent-binary operators; exact 56-case leak policy remains fail-closed"
