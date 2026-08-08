#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5824")
SOURCES = %w[csdlc-v2/src/cards.rs csdlc-v2/src/model.rs].freeze
ALLOWED = %w[typed_complete finite_gap intentionally_extensible].freeze
OWNER_KEYS = %w[parser formatter schema editor validator markdown tests].freeze

definitions = {}
SOURCES.each do |relative|
  source = ROOT.join(relative).read
  source.scan(/pub enum\s+([A-Z][A-Za-z0-9_]*)/) do |name|
    definitions[name.first] = { "source" => relative, "kind" => "enum" }
  end
  source.scan(/closed_enum!\(([A-Z][A-Za-z0-9_]*)/) do |name|
    definitions[name.first] = { "source" => relative, "kind" => "enum" }
  end

  current_container = nil
  current_kind = nil
  current_variant = nil
  depth = 0
  source.each_line do |line|
    if current_container.nil? && (match = line.match(/^pub (struct|enum)\s+([A-Z][A-Za-z0-9_]*)\s*\{/))
      current_kind = match[1]
      current_container = match[2]
      depth = line.count("{") - line.count("}")
    elsif current_container
      if current_kind == "enum" && (match = line.match(/^\s*([A-Z][A-Za-z0-9_]*)\s*(?:\{|,)/))
        current_variant = match[1]
      end
      line.scan(/\b([a-z][a-z0-9_]*):\s*(?:Option<)?String/) do |field|
        owner = [current_container, current_variant, field.first].compact.join(".")
        definitions[owner] = { "source" => relative, "kind" => "string_candidate" }
      end
      depth += line.count("{") - line.count("}")
      if depth.zero?
        current_container = nil
        current_kind = nil
        current_variant = nil
      end
    end
  end
end
abort "restricted-field candidate denominator is empty" if definitions.empty?

inventory_path = EVIDENCE.join("enum-inventory.json")
abort "missing enum inventory" unless inventory_path.file? && !inventory_path.zero?
rows = JSON.parse(inventory_path.read)
abort "enum inventory must be an array" unless rows.is_a?(Array)
fields = rows.map { |row| row["field"] }
abort "duplicate enum inventory field" unless fields.uniq.length == fields.length
missing = definitions.keys - fields
extra = fields - definitions.keys
abort "enum denominator mismatch; missing=#{missing.sort.join(', ')} extra=#{extra.sort.join(', ')}" unless missing.empty? && extra.empty?

rows.each do |row|
  field = row["field"]
  definition = definitions.fetch(field)
  abort "#{field} source mismatch" unless row["source"] == definition.fetch("source")
  abort "#{field} kind mismatch" unless row["kind"] == definition.fetch("kind")
  abort "#{field} invalid disposition" unless ALLOWED.include?(row["disposition"])
  if row["kind"] == "string_candidate" && row["disposition"] == "typed_complete"
    abort "#{field} string candidate cannot be typed_complete"
  end
  abort "#{field} stored_string must be explicit" unless [true, false].include?(row["stored_string"])
  owners = row["owners"]
  abort "#{field} owners must be an object" unless owners.is_a?(Hash)
  abort "#{field} owner denominator mismatch" unless owners.keys.sort == OWNER_KEYS.sort
  OWNER_KEYS.each do |owner|
    value = owners.fetch(owner)
    abort "#{field} #{owner} lacks a source-grounded disposition" if value.to_s.empty?
    next if value.start_with?("not_applicable:") && value.length > "not_applicable:".length

    path, symbol = value.split("#", 2)
    candidate = ROOT.join(path)
    abort "#{field} #{owner} references missing path #{path}" unless candidate.file?
    if symbol && !symbol.empty?
      abort "#{field} #{owner} references missing symbol #{symbol}" unless candidate.read.include?(symbol)
    end
  end
end

denominator = definitions.keys.sort
denominator_sha256 = Digest::SHA256.hexdigest(JSON.generate(denominator))
decision_path = EVIDENCE.join("enum-audit-decision.json")
abort "missing enum audit decision" unless decision_path.file? && !decision_path.zero?
decision = JSON.parse(decision_path.read)
abort "decision denominator digest mismatch" unless decision["denominator_sha256"] == denominator_sha256
abort "decision audited fields mismatch" unless Array(decision["audited_fields"]).sort == denominator
source_sha256 = SOURCES.to_h { |source| [source, Digest::SHA256.hexdigest(ROOT.join(source).read)] }
abort "decision source digest mismatch" unless decision["source_sha256"] == source_sha256
inventory_sha256 = Digest::SHA256.hexdigest(JSON.generate(rows))
abort "decision inventory digest mismatch" unless decision["inventory_sha256"] == inventory_sha256
finite_gaps = rows.select { |row| row["disposition"] == "finite_gap" }.map { |row| row["field"] }.sort
abort "decision finite-gap set mismatch" unless Array(decision["finite_gap_fields"]).sort == finite_gaps

if finite_gaps.empty?
  abort "no-gap audit must record no_duplicate_work" unless decision["outcome"] == "no_duplicate_work"
  abort "no-gap audit overclaims implementation" unless decision["implementation_required"] == false
  abort "no-gap audit lacks explicit disposition" unless decision["no_duplicate_work"] == true
else
  abort "finite-gap audit lacks selected family" unless decision["outcome"] == "finite_gap_selected"
  selected = Array(decision["selected_family"]).sort
  abort "selected family must be a nonempty subset of finite gaps" if selected.empty? || !(selected - finite_gaps).empty?
  abort "finite-gap audit must require implementation" unless decision["implementation_required"] == true
end

puts "WP-07 enum inventory valid: #{denominator.length} source-derived restricted types, #{finite_gaps.length} finite gaps"
