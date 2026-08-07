#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, ".csdlc/evidence/19")

def load_json(name)
  JSON.parse(File.read(File.join(EVIDENCE, name)))
end

def require_truth(condition, message)
  raise message unless condition
end

manifest = load_json("deployment-manifest.json")
live = load_json("live-verification.json")
source = File.join(ROOT, "demos/_preview/podcast/index.html")
source_digest = Digest::SHA256.file(source).hexdigest

require_truth(manifest.fetch("issue") == 19, "deployment manifest issue mismatch")
require_truth(live.fetch("issue") == 19, "live verification issue mismatch")
require_truth(source_digest == live.fetch("source_sha256"), "source digest mismatch")
require_truth(
  live.fetch("live_sha256").values.all? { |digest| digest == source_digest },
  "live preview digest mismatch"
)

active = manifest.fetch("active_objects")
preview_objects = active.select { |object| object.fetch("source") == "demos/_preview/podcast/index.html" }
require_truth(preview_objects.length == 2, "both preview HTML object keys are required")
require_truth(preview_objects.all? { |object| object.fetch("sha256") == source_digest }, "preview object digest mismatch")

aws = manifest.fetch("aws_boundary")
require_truth(aws.fetch("approved_business_account_verified"), "business AWS account was not verified")
require_truth(aws.fetch("services_invoked").sort == %w[cloudfront s3 sts], "unexpected AWS service set")
require_truth(aws.fetch("compute_services_invoked").empty?, "compute service invocation recorded")
require_truth(aws.fetch("ec2_operations").empty?, "EC2 operation recorded")
require_truth(aws.fetch("identifiers_redacted"), "infrastructure identifiers are not redacted")

after = manifest.fetch("after")
require_truth(after.fetch("preview_status") == 200, "preview route is not proven live")
require_truth(after.fetch("production_route_status") == 403, "production route changed")
require_truth(after.fetch("noindex_nofollow"), "preview robots boundary missing")
require_truth(after.fetch("external_asset_requests").zero?, "external asset request recorded")
require_truth(after.fetch("script_count").zero?, "script execution recorded")

%w[browser_desktop browser_mobile].each do |key|
  browser = live.fetch(key)
  screenshot = File.join(EVIDENCE, browser.fetch("screenshot"))
  require_truth(File.file?(screenshot), "missing #{key} screenshot")
  require_truth(
    Digest::SHA256.file(screenshot).hexdigest == browser.fetch("screenshot_sha256"),
    "#{key} screenshot digest mismatch"
  )
  require_truth(browser.fetch("console_errors_or_warnings").zero?, "#{key} console failure recorded")
  require_truth(browser.fetch("scripts").zero?, "#{key} script execution recorded")
  require_truth(browser.fetch("scroll_width") == browser.fetch("client_width"), "#{key} horizontal overflow")
end

failure = live.fetch("initial_failure")
failure_screenshot = File.join(EVIDENCE, failure.fetch("screenshot"))
require_truth(File.file?(failure_screenshot), "initial failure screenshot is missing")
require_truth(
  Digest::SHA256.file(failure_screenshot).hexdigest == failure.fetch("screenshot_sha256"),
  "initial failure screenshot digest mismatch"
)

puts "podcast_preview_deployment_evidence: PASS"
