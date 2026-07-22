#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[transaction-fault-matrix fresh-install-override rollback-window-evidence cutover-budgets post-merge-exact]
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

warn("#{lane} is execution-gated and has no implementation in the preparation-only packet")
puts JSON.pretty_generate(status: "blocked", lane: lane, reason: "#5344 must be live-merged and ancestral with accepted soak/rollback handoff")
exit 2
