#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
root = File.expand_path("../../../..", __dir__)
index = JSON.parse(File.read(File.join(root, ".csdlc/issues/500/index.json")))
design = File.read(File.join(root, index.fetch("design_path")))
abort("missing v2 authority boundary") unless design.include?("C-SDLC v2 remains the sole operational lifecycle authority")
abort("missing predecessor mapping") unless %w[#161 #162 #163].all? { |id| design.include?(id) }
abort("missing proportional lifecycle contract") unless design.include?("## Proportional lifecycle contract")
abort("missing risk-based removal rule") unless design.include?("do not materially reduce delivery risk")
abort("missing minutes-not-hours outcome") unless design.include?("minutes, not hours")
abort("missing retained-gate hazard rule") unless design.include?("concrete hazard")
puts '{"schema":"adl.v0921.v3a_design_readiness.v1","outcome":"passed","issue":500}'
