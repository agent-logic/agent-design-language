#!/usr/bin/env ruby
# PVF: deterministic local docs contract, small CPU/Git reads, issue gate.
require 'json'
require 'digest'
require 'open3'
ROOT = File.expand_path('../../../..', __dir__)
PACKET = File.join(ROOT, 'docs/milestones/v0.92.1/evidence/release/tail-03')
SOURCE_DIR = 'docs/milestones/v0.92.1/evidence/release/tail-02/'
CACHE = {}
def blob(sha, path)
  raise 'unsafe revision or path' unless sha.match?(/\A[0-9a-f]{40}\z/) && !path.start_with?('/') && !path.split('/').include?('..')
  CACHE[[sha,path]] ||= begin
    text, error, status = Open3.capture3('git', '-C', ROOT, 'show', "#{sha}:#{path}")
    raise "missing source object: #{path}" unless status.success?
    text
  end
end
def digest(text); Digest::SHA256.hexdigest(text); end
def verify(p, final: false)
  raise 'wrong identity' unless p.values_at('schema','issue','repository','source_issue','source_pr') == ['adl.tail03.publication_candidate.v1',519,'agent-logic/agent-design-language',518,753]
  raise 'release mutation or approval' unless p['release_approval'] == false && p['mutations_performed'] == []
  expected = [{'issue'=>518,'pull_request'=>753,'keyword'=>'Closes #518'}, {'issue'=>519,'pull_request'=>nil,'keyword'=>'Closes #519'}]
  raise 'ambiguous closing relationships' unless p['closing_relationships'] == expected
  sha = p.fetch('source_head'); reviewed = p.fetch('reviewed_source_head')
  paths, _, status = Open3.capture3('git','-C',ROOT,'ls-tree','-r','--name-only',sha,'--',SOURCE_DIR)
  raise 'artifact denominator mismatch' unless status.success? && p['artifacts'].map { |a| a['path'] }.sort == paths.lines.map(&:strip).sort
  p['artifacts'].each { |a| raise "artifact hash mismatch: #{a['path']}" unless digest(blob(sha,a['path'])) == a['sha256'] }
  raise 'wrong manifest path' unless p['handoff_manifest'] == SOURCE_DIR+'handoff-content.json'
  manifest = JSON.parse(blob(sha,p['handoff_manifest']))
  docs = manifest.fetch('documents')
  raise 'document denominator mismatch' unless docs.size == p['document_count'] && docs.map { |d| d['path'] }.uniq.size == docs.size
  docs.each { |d| raise "document hash mismatch: #{d['path']}" unless digest(blob(sha,d['path'])) == d['sha256'] }
  ref=p.fetch('review_index')
  raise 'wrong review record' unless ref['path'] == '.csdlc/issues/518/index.json'
  record=blob(sha,ref['path'])
  raise 'review record digest mismatch' unless digest(record)==ref['sha256']
  review=JSON.parse(record).fetch('review')
  raise 'incomplete source review' unless review['completed'] == true && review['reviewed_revision'].split(':')[1] == reviewed && !review.fetch('scope').empty?
  raise 'unresolved actionable source review' if review.fetch('findings').any? { |f| f['actionable'] && f['disposition']=='open' }
  review['scope'].each { |path| raise "source changed after review: #{path}" unless blob(sha,path)==blob(reviewed,path) }
  if final
    raise 'preparation is not final acceptance' unless p['status']=='final' && p['final_blockers']==[]
    observation=JSON.parse(File.read(File.join(PACKET,'source-pr.json')))
    raise 'source PR not merged at expected head' unless observation['number']==753 && observation['state']=='MERGED' && observation['headRefOid']==sha && observation['baseRefName']=='main' && observation['mergedAt']
    raise 'source closing linkage missing' unless observation.fetch('body').match?(/\bCloses #518\b/)
    merge=observation.fetch('mergeCommit').fetch('oid')
    raise 'invalid merge revision' unless merge.match?(/\A[0-9a-f]{40}\z/)
    _,_,ok=Open3.capture3('git','-C',ROOT,'merge-base','--is-ancestor',merge,'origin/main')
    raise 'source merge not on canonical main' unless ok.success?
    (docs.map { |d| d['path'] }+p['artifacts'].map { |a| a['path'] }).uniq.each { |path| raise "merged content drift: #{path}" unless blob(sha,path)==blob(merge,path) }
  else
    raise 'premature final claim' unless p['status']=='preparation' && !p['final_blockers'].empty?
  end
  true
end
begin
  mode=ARGV.fetch(0,'--all')
  raise 'unknown mode' unless %w[--preparation --self-test --linkage --exact-head --redaction --all].include?(mode)
  packet=JSON.parse(File.read(File.join(PACKET,'candidate.json')))
  verify(packet, final: !%w[--preparation --self-test].include?(mode))
  Dir.glob(File.join(PACKET,'**','*')).select { |f| File.file?(f) }.each do |path|
    text=File.read(path)
    raise "redaction failure: #{File.basename(path)}" if text.match?(%r{/Users/|/Volumes/|/private/tmp/|-----BEGIN .*PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{20,}|AKIA[A-Z0-9]{16}})
  end
  negatives=0
  if mode=='--self-test'
    [:hash,:linkage,:premature_final].each do |damage|
      broken=Marshal.load(Marshal.dump(packet))
      case damage
      when :hash then broken['artifacts'][0]['sha256']='0'*64
      when :linkage then broken['closing_relationships'][1]['keyword']='Closes #518'
      when :premature_final then broken['status']='final'
      end
      rejected=false
      begin; verify(broken); rescue StandardError; rejected=true; end
      raise "negative fixture accepted: #{damage}" unless rejected
      negatives+=1
    end
  end
  puts JSON.pretty_generate(status:'pass',mode:mode,documents:packet['document_count'],artifacts:packet['artifacts'].size,negative_fixtures:negatives,final_acceptance:!%w[--preparation --self-test].include?(mode),release_approval:false)
rescue StandardError => error
  puts JSON.pretty_generate(status:'blocked',message:error.message,final_acceptance:false)
  exit 1
end
