#!/usr/bin/env ruby
# PVF: deterministic local validator regression; small resource profile; no network.
# Proof role: later tracked growth must not invalidate immutable TAIL-02 evidence.
require 'fileutils'
require 'json'
require 'open3'
require 'tmpdir'

ROOT = File.expand_path('../../../..', __dir__)

def run!(*argv, chdir:)
  output, error, status = Open3.capture3(*argv, chdir: chdir)
  raise "command failed: #{argv.join(' ')}\n#{error}" unless status.success?
  JSON.parse(output)
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

  puts JSON.generate(schema: 'adl.tail02.tracked_growth_regression.v1', status: 'pass',
    added_tracked_paths: 1, historical_denominator: 737, linkage_denominator: 791)
end
