#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

ROOT = File.expand_path("../../../..", __dir__)
SCHEDULE_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json")
README_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.md")

def fail_with(message)
  warn "CORP-A provenance/license validation failed: #{message}"
  exit 1
end

fail_with("missing schedule #{SCHEDULE_PATH}") unless File.file?(SCHEDULE_PATH)
fail_with("missing Markdown schedule #{README_PATH}") unless File.file?(README_PATH)

schedule = JSON.parse(File.read(SCHEDULE_PATH))
readme = File.read(README_PATH)

fail_with("schedule source basis is empty") if schedule.fetch("source_basis").empty?
fail_with("schedule source basis must include independent denominator") unless schedule.fetch("source_basis").any? { |row| row.fetch("path") == ".csdlc/prepared/issues/482/canonical-critical-asset-denominator.v1.json" }
fail_with("authority boundary must exclude private instruments") unless schedule.dig("authority_boundary", "private_instruments_included") == false
fail_with("authority boundary must exclude credentials") unless schedule.dig("authority_boundary", "credential_material_included") == false
fail_with("missing acceptance authority") unless schedule.fetch("acceptance_authority").fetch("authority_receipt_ref").include?("corp-a-authority-receipt-2026-08-26-r1")
fail_with("acceptance authority date mismatch") unless schedule.fetch("acceptance_authority").fetch("accepted_at") == schedule.fetch("accepted_at")
fail_with("acceptance authority role mismatch") unless schedule.fetch("acceptance_authority").fetch("accepted_by") == schedule.fetch("accepted_by")

schedule.fetch("assets").each do |asset|
  id = asset.fetch("id")
  provenance = asset.fetch("provenance")
  licensing = asset.fetch("licensing")
  trademark = asset.fetch("trademark")
  assignment = asset.fetch("assignment")

  fail_with("#{id} has no provenance source refs") if provenance.fetch("source_refs").empty?
  fail_with("#{id} has no provenance evidence refs") if provenance.fetch("evidence_refs").empty?
  fail_with("#{id} missing licensing disposition") if licensing.fetch("disposition").strip.empty?
  fail_with("#{id} missing licensing review route") if licensing.fetch("review_route").strip.empty?
  fail_with("#{id} missing trademark disposition") if trademark.fetch("disposition").strip.empty?
  fail_with("#{id} missing trademark counsel route") if trademark.fetch("counsel_route").strip.empty?
  fail_with("#{id} assignment is not accepted") unless assignment.fetch("disposition") == "operator-accepted-custody"
  fail_with("#{id} assignment receipt mismatch") unless assignment.fetch("receipt_ref") == asset.fetch("custody_receipt_ref")
  fail_with("#{id} not listed in Markdown schedule") unless readme.include?(id)

  class_name = asset.fetch("asset_class")
  next unless class_name.match?(/brand|domain|media|assignment|license/)

  route = trademark.fetch("counsel_route")
  fail_with("#{id} brand/domain/media/assignment/license class lacks counsel route") unless route.downcase.include?("counsel")
end

puts "CORP-A provenance/license/trademark routes ok: #{schedule.fetch("assets").length} assets checked"
