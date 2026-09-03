#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
d = File.read(File.join(root, ".csdlc/prepared/issues/508/design.md"))
abort("missing DRT-B dependency") unless d.include?("#507")
abort("missing authenticity boundary") unless d.include?("Runtime-authentic")
puts '{"schema":"adl.v0921.drt_c.readiness.v1","outcome":"passed"}'
