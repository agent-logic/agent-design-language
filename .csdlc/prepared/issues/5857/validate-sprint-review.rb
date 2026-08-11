#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
packet = File.join(root, ".csdlc/evidence/5857/sprint-review.json")
abort("sprint_review_missing: #{packet.delete_prefix(root + "/")}") unless File.file?(packet)

document = JSON.parse(File.read(packet))
abort("sprint_review_schema_invalid") unless document["schema"] == "adl.sprint_review.v1"
children = document["children"]
abort("sprint_review_children_missing") unless children.is_a?(Array) && !children.empty?

puts JSON.generate({ schema: "adl.sprint_review_validation.v1", status: "passed", children: children.length })
