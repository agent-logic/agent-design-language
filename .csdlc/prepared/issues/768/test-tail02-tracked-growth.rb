#!/usr/bin/env ruby
# PVF: deterministic local validator regression; small resource profile; no network.
# Proof role: later tracked growth must not invalidate immutable TAIL-02 evidence.
require 'fileutils'
require 'json'
require 'open3'
require 'tmpdir'
require 'digest'

ROOT = File.expand_path('../../../..', __dir__)

def run!(*argv, chdir:)
  output, error, status = Open3.capture3(*argv, chdir: chdir)
  raise "command failed: #{argv.join(' ')}\n#{error}" unless status.success?
  JSON.parse(output)
end

def expect_rejection!(directory, expected)
  output, error, status = Open3.capture3('ruby', '.csdlc/prepared/issues/518/validate-documentation-handoff.rb',
    '--candidate-linkage', chdir: directory)
  raise "negative fixture unexpectedly passed: #{expected}" if status.success?
  message = output + error
  raise "wrong rejection for #{expected}: #{message}" unless message.include?(expected)
end

def with_json_change(path)
  original = File.binread(path)
  parsed = JSON.parse(original)
  yield parsed
  File.write(path, JSON.pretty_generate(parsed) + "\n")
  yield :verify
ensure
  File.binwrite(path, original) if original
end

def with_text_change(path)
  original = File.binread(path)
  File.open(path, 'ab') { |file| file.write("\nfixture drift\n") }
  yield
ensure
  File.binwrite(path, original) if original
end

Dir.mktmpdir('adl-tail02-growth-') do |directory|
  _, error, status = Open3.capture3('git', 'clone', '--shared', '--quiet', ROOT, directory)
  raise "fixture clone failed: #{error}" unless status.success?
  FileUtils.cp(
    File.join(ROOT, '.csdlc/prepared/issues/518/validate-documentation-handoff.rb'),
    File.join(directory, '.csdlc/prepared/issues/518/validate-documentation-handoff.rb')
  )
  File.write(File.join(directory, 'README-tail02-growth-fixture.md'), "# Later tracked documentation\n")
  _, error, status = Open3.capture3('git', 'add', '.csdlc/prepared/issues/518/validate-documentation-handoff.rb',
    'README-tail02-growth-fixture.md', chdir: directory)
  raise "fixture add failed: #{error}" unless status.success?
  env = {
    'GIT_AUTHOR_NAME' => 'TAIL-02 fixture', 'GIT_AUTHOR_EMAIL' => 'fixture@example.invalid',
    'GIT_COMMITTER_NAME' => 'TAIL-02 fixture', 'GIT_COMMITTER_EMAIL' => 'fixture@example.invalid'
  }
  _, error, status = Open3.capture3(env, 'git', 'commit', '--quiet', '-m', 'fixture: later tracked growth', chdir: directory)
  raise "fixture commit failed: #{error}" unless status.success?

  historical = run!('ruby', '.csdlc/prepared/issues/518/validate-documentation-handoff.rb', '--historical', chdir: directory)
  linkage = run!('ruby', '.csdlc/prepared/issues/518/validate-documentation-handoff.rb', '--candidate-linkage', chdir: directory)
  raise 'historical denominator changed' unless historical['denominator'] == 737
  raise 'candidate linkage denominator changed' unless linkage['denominator'] == 791

  packet = File.join(directory, 'docs/milestones/v0.92.1/evidence/release/tail-02')
  dependency_path = File.join(packet, 'dependency-observation.json')
  dependency = JSON.parse(File.read(dependency_path))
  with_text_change(File.join(directory, dependency.fetch('source_path'))) do
    expect_rejection!(directory, 'quality source drift')
  end
  with_json_change(dependency_path) do |value|
    if value == :verify
      expect_rejection!(directory, 'quality exceptions changed')
    else
      value['unresolved'] = value.fetch('unresolved') + [{ 'id' => 'fixture-drift' }]
    end
  end
  with_json_change(dependency_path) do |value|
    if value == :verify
      expect_rejection!(directory, 'release decision overclaimed')
    else
      value['quality_gate'] = 'passed'
    end
  end
  with_json_change(dependency_path) do |value|
    if value == :verify
      expect_rejection!(directory, 'predecessor merge missing')
    else
      value['merge_commit'] = '0' * 40
    end
  end
  findings_path = File.join(packet, 'finding-dispositions.json')
  with_json_change(findings_path) do |value|
    if value == :verify
      expect_rejection!(directory, 'undispositioned finding')
    else
      value.first['status'] = 'open'
    end
  end
  reconciliation = dependency.fetch('reconciliation')
  reconciliation_path = File.join(directory, reconciliation.fetch('source_path'))
  with_text_change(reconciliation_path) do
    expect_rejection!(directory, 'accounting source drift')
  end
  [['unowned_exception_count', 1], ['exceptions', []], ['release_authorized', true]].each do |key, replacement|
    original_reconciliation = File.binread(reconciliation_path)
    original_dependency = File.binread(dependency_path)
    begin
      changed = JSON.parse(original_reconciliation)
      changed[key] = replacement
      File.write(reconciliation_path, JSON.pretty_generate(changed) + "\n")
      dependency_with_digest = JSON.parse(original_dependency)
      dependency_with_digest['reconciliation']['source_sha256'] = Digest::SHA256.file(reconciliation_path).hexdigest
      File.write(dependency_path, JSON.pretty_generate(dependency_with_digest) + "\n")
      expect_rejection!(directory, 'accounting disposition mismatch')
    ensure
      File.binwrite(reconciliation_path, original_reconciliation)
      File.binwrite(dependency_path, original_dependency)
    end
  end
  with_json_change(dependency_path) do |value|
    if value == :verify
      expect_rejection!(directory, 'accounting merge missing')
    else
      value['reconciliation']['merge_commit'] = '0' * 40
    end
  end

  puts JSON.generate(schema: 'adl.tail02.tracked_growth_regression.v1', status: 'pass',
    added_tracked_paths: 1, historical_denominator: 737, linkage_denominator: 791,
    retained_guard_negative_fixtures: 10)
end
