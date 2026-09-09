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
def redaction_patterns
  [
    ['machine_local_path', %r{/Users/|/Volumes/|/private/tmp/}],
    ['credential_location', %r{/(?:Users|Volumes)/[^[:space:]`'"]*(?:key|keys|credential|credentials|token|secret)[^[:space:]`'"]*}i],
    ['private_key_block', /-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----/],
    ['github_token', /gh[pousr]_[A-Za-z0-9]{20,}/],
    ['aws_access_key', /AKIA[A-Z0-9]{16}/],
    ['bearer_token', /Bearer[[:space:]]+[A-Za-z0-9._~+\/=-]{16,}/],
    ['common_provider_token', /(?:OPENAI|ANTHROPIC|DEEPSEEK|GEMINI|GOOGLE|AWS|GITHUB)_[A-Z0-9_]*(?:API_)?(?:KEY|TOKEN)[[:space:]]*=[[:space:]]*[^[:space:]]{8,}/i]
  ]
end
def redaction_findings(path, text)
  findings = []
  text.each_line.with_index(1) do |line, index|
    redaction_patterns.each do |category, pattern|
      findings << { 'path' => path, 'line' => index, 'category' => category } if line.match?(pattern)
    end
  end
  findings
end
def redaction_safe?(text)
  redaction_findings('<inline>', text).empty?
end
def scan_manifest_documents!(manifest, expected_count)
  docs = manifest.fetch('documents')
  raise 'document denominator mismatch' unless docs.size == expected_count && docs.map { |d| d['path'] }.uniq.size == docs.size
  findings = []
  docs.each do |d|
    path = d.fetch('path')
    raise "unsafe document path: #{path}" if path.start_with?('/') || path.split('/').include?('..')
    full_path = File.join(ROOT, path)
    raise "unreadable referenced document: #{path}" unless File.file?(full_path) && File.readable?(full_path)
    text = File.read(full_path)
    raise "document hash mismatch: #{path}" unless digest(text) == d.fetch('sha256')
    findings.concat(redaction_findings(path, text))
  end
  unless findings.empty?
    first = findings.first
    raise "redaction failure: #{first['path']}:#{first['line']}:#{first['category']}"
  end
  { 'documents_scanned' => docs.size, 'findings' => findings.size }
end
def verify(p, final: false, observation: nil)
  raise 'wrong identity' unless p.values_at('schema','issue','repository','source_issue','source_pr') == ['adl.tail03.publication_candidate.v1',519,'agent-logic/agent-design-language',518,753]
  raise 'release mutation or approval' unless p['release_approval'] == false && p['mutations_performed'] == []
  expected = [{'issue'=>518,'pull_request'=>753,'keyword'=>'Closes #518'}, {'issue'=>519,'pull_request'=>nil,'keyword'=>'Closes #519'}]
  raise 'ambiguous closing relationships' unless p['closing_relationships'] == expected
  sha = p.fetch('source_head'); reviewed = p.fetch('reviewed_source_head')
  paths, _, status = Open3.capture3('git','-C',ROOT,'ls-tree','-r','--name-only',sha,'--',SOURCE_DIR)
  raise 'artifact denominator mismatch' unless status.success? && p['artifacts'].map { |a| a['path'] }.sort == paths.lines.map(&:strip).sort
  p['artifacts'].each { |a| raise "artifact hash mismatch: #{a['path']}" unless digest(blob(sha,a['path'])) == a['sha256'] }
  raise 'wrong manifest path' unless p['handoff_manifest'] == SOURCE_DIR+'handoff-content.json'
  manifest = JSON.parse(File.read(File.join(ROOT, p['handoff_manifest'])))
  scan_manifest_documents!(manifest, p['document_count'])
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
    observation ||= JSON.parse(File.read(File.join(PACKET,'source-pr.json')))
    raise 'wrong source repository' unless observation['url']=='https://github.com/agent-logic/agent-design-language/pull/753'
    %w[--all].each do |flag|
      %w[--get-url --get-url-push].each do |kind|
        argv = kind=='--get-url' ? ['remote','get-url',flag,'origin'] : ['remote','get-url','--push',flag,'origin']
        urls,_,ok=Open3.capture3('git','-C',ROOT,*argv)
        allowed=['https://github.com/agent-logic/agent-design-language.git','https://github.com/agent-logic/agent-design-language','git@github.com:agent-logic/agent-design-language.git']
        raise 'noncanonical origin' unless ok.success? && !urls.lines.empty? && urls.lines.all? { |url| allowed.include?(url.strip) }
      end
    end
    raise 'source PR not merged at expected head' unless observation['number']==753 && observation['state']=='MERGED' && observation['headRefOid']==sha && observation['baseRefName']=='main' && observation['mergedAt']
    raise 'source closing linkage missing' unless observation.fetch('body').match?(/\bCloses #518\b/)
    merge=observation.fetch('mergeCommit').fetch('oid')
    raise 'invalid merge revision' unless merge.match?(/\A[0-9a-f]{40}\z/)
    _,_,ok=Open3.capture3('git','-C',ROOT,'merge-base','--is-ancestor',merge,'origin/main')
    raise 'source merge not on canonical main' unless ok.success?
    (p['artifacts'].map { |a| a['path'] }).uniq.each { |path| raise "merged content drift: #{path}" unless blob(sha,path)==blob(merge,path) }
  else
    raise 'premature final claim' unless p['status']=='preparation' && !p['final_blockers'].empty?
  end
  true
end
begin
  mode=ARGV.fetch(0,'--all')
  raise 'unknown mode' unless %w[--preparation --self-test --linkage --exact-head --redaction --all].include?(mode)
  packet=JSON.parse(File.read(File.join(PACKET,'candidate.json')))
  final = mode != '--preparation' && (mode != '--self-test' || packet['status']=='final')
  verify(packet, final: final)
  packet_findings = []
  Dir.glob(File.join(PACKET,'**','*')).select { |f| File.file?(f) }.each do |path|
    text=File.read(path)
    packet_findings.concat(redaction_findings(path.sub(ROOT+'/', ''), text))
  end
  unless packet_findings.empty?
    first = packet_findings.first
    raise "redaction failure: #{first['path']}:#{first['line']}:#{first['category']}"
  end
  negatives=0
  if mode=='--self-test'
    observation=JSON.parse(File.read(File.join(PACKET,'source-pr.json')))
    cases=[:hash,:linkage,:status]
    cases += [:unmerged,:wrong_head,:wrong_repository,:merge_drift] if final
    cases.each do |damage|
      broken=Marshal.load(Marshal.dump(packet)); obs=Marshal.load(Marshal.dump(observation))
      case damage
      when :hash then broken['artifacts'][0]['sha256']='0'*64
      when :linkage then broken['closing_relationships'][1]['keyword']='Closes #518'
      when :status then broken['status']=final ? 'preparation' : 'final'
      when :unmerged then obs['state']='OPEN'
      when :wrong_head then obs['headRefOid']='0'*40
      when :wrong_repository then obs['url']='https://github.com/other/repository/pull/753'
      when :merge_drift then obs['mergeCommit']['oid']=Open3.capture3('git','-C',ROOT,'rev-parse',"#{obs['mergeCommit']['oid']}^1")[0].strip
      end
      expected={hash:/artifact hash mismatch/,linkage:/ambiguous closing/,status:/preparation is not final acceptance|premature final claim/,unmerged:/source PR not merged/,wrong_head:/source PR not merged/,wrong_repository:/wrong source repository/,merge_drift:/merged content drift|missing source object/}.fetch(damage)
      rejected=false
      begin
        verify(broken,final:final,observation:obs)
      rescue StandardError => failure
        raise "wrong rejection for #{damage}: #{failure.message}" unless failure.message.match?(expected)
        rejected=true
      end
      raise "negative fixture accepted: #{damage}" unless rejected
      negatives+=1
    end
    [
      ('/'+'Users/example/private'),
      ('/'+'Users/example/keys/provider.json'),
      '-----BEGIN PRIVATE KEY-----',
      ('ghp_'+'x'*24),
      ('AKIA'+'A'*16),
      ('Bearer '+'x'*24),
      ('OPENAI_API_KEY='+'x'*24)
    ].each do |sensitive|
      raise 'redaction negative accepted' if redaction_safe?(sensitive)
      negatives+=1
    end
  end
  puts JSON.pretty_generate(status:'pass',mode:mode,documents:packet['document_count'],artifacts:packet['artifacts'].size,manifest_documents_scanned:packet['document_count'],redaction_findings:0,negative_fixtures:negatives,final_acceptance:final,release_approval:false)
rescue StandardError => error
  puts JSON.pretty_generate(status:'blocked',message:error.message,final_acceptance:false)
  exit 1
end
