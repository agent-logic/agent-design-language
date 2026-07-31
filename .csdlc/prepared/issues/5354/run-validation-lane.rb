#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "base64"
require "fileutils"
require "json"
require "open3"
require "openssl"
require "pathname"
require "securerandom"
require "socket"
require "uri"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5354
PACKET = ROOT.join(".csdlc/evidence/5354/convergence-proof.v1.json")
FIXTURE = ROOT.join("adl-characterization/corpus/v1/fixtures/mock-run.adl.yaml")
DEMO_MATRIX = ROOT.join("docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md")
FEATURE_MATRIX = ROOT.join("docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md")
WP14A_MERGE = "72fbf30c74a5193ea41f042c76c5986a48e59d6c"
UNITY_MERGES = {
  4739 => "e02e138409771c9321341bf599cd1c2cd0ecaef1",
  4741 => "491f9570289f3355b86a2ee13736695fd56bd870",
  5332 => "89fb6847a2a78effd66b73607681f463077ea018",
  5683 => "75246bfd76935f567809fedd52e92b2cd80c679e"
}.freeze
UNITY_PROOF = ROOT.join(".csdlc/evidence/5683/LIVE_UNITY_PROOF.md")
UNITY_IMAGES = [
  ROOT.join(".csdlc/evidence/5683/final-full-hd-game-view.png"),
  ROOT.join(".csdlc/evidence/5683/final-qhd-game-view.png")
].freeze
ALLOWED_LANES = %w[integrated-live-demo claim-boundary-matrix complete post-merge-exact].freeze
POST_PROOF_METADATA_PREFIXES = [
  ".csdlc/evidence/5354/convergence-proof.v1.json",
  ".csdlc/issues/5354/",
  ".csdlc/prepared/issues/5354/assign-",
  ".csdlc/prepared/issues/5354/record-review",
  ".csdlc/prepared/issues/5354/advance-reviewed",
  ".csdlc/prepared/issues/5354/publish"
].freeze

def fail_lane(message)
  warn("#5354 convergence: #{message}")
  exit 1
end

def run!(*argv, env: {})
  out, err, status = Open3.capture3(env, *argv, chdir: ROOT.to_s)
  fail_lane("#{argv.join(' ')} failed: #{err.strip}\n#{out.strip}") unless status.success?
  out
end

def git(*args)
  run!("git", "-C", ROOT.to_s, *args).strip
end

def installed_binary(name)
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  fail_lane("missing installed typed binary #{name}") unless binary.file? && binary.executable?
  binary.to_s
end

def json_command(*argv, env: {})
  JSON.parse(run!(*argv, env: env))
rescue JSON::ParserError => e
  fail_lane("#{argv.join(' ')} returned invalid JSON: #{e.message}")
end

def assert_ancestor(sha)
  _out, _err, status = Open3.capture3(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", sha, "HEAD"
  )
  fail_lane("required merge #{sha} is not ancestral to HEAD") unless status.success?
end

def sha256(path)
  fail_lane("missing retained artifact #{path.relative_path_from(ROOT)}") unless path.file?
  Digest::SHA256.file(path).hexdigest
end

def read_exact(io, length)
  bytes = +"".b
  bytes << io.readpartial(length - bytes.bytesize) while bytes.bytesize < length
  bytes
end

def read_websocket_frame(io)
  header = read_exact(io, 2).bytes
  opcode = header[0] & 0x0f
  masked = (header[1] & 0x80) != 0
  length = header[1] & 0x7f
  length = read_exact(io, 2).unpack1("n") if length == 126
  length = read_exact(io, 8).unpack1("Q>") if length == 127
  mask = masked ? read_exact(io, 4).bytes : nil
  payload = read_exact(io, length)
  payload = payload.bytes.each_with_index.map { |byte, index| byte ^ mask[index % 4] }.pack("C*") if masked
  [opcode, payload]
end

def masked_text_frame(payload)
  payload = payload.b
  mask = SecureRandom.random_bytes(4)
  length = payload.bytesize
  length_header =
    if length <= 125
      [0x80 | length].pack("C")
    elsif length <= 65_535
      [0x80 | 126, length].pack("Cn")
    else
      [0x80 | 127, length].pack("CQ>")
    end
  masked = payload.bytes.each_with_index.map do |byte, index|
    byte ^ mask.getbyte(index % 4)
  end.pack("C*")
  [0x81].pack("C") + length_header + mask + masked
end

def prove_live_websocket(runtime_base, ca_cert)
  uri = URI(runtime_base)
  websocket_path = "/v1/observatory/ws"
  tcp = TCPSocket.new(uri.host, uri.port)
  context = OpenSSL::SSL::SSLContext.new
  context.verify_mode = OpenSSL::SSL::VERIFY_NONE
  socket = OpenSSL::SSL::SSLSocket.new(tcp, context)
  socket.hostname = uri.host
  socket.sync_close = true
  socket.connect
  expected_certificate = OpenSSL::X509::Certificate.new(ca_cert.read)
  fail_lane("live WSS certificate pin rejected") unless
    Digest::SHA256.hexdigest(socket.peer_cert.to_der) ==
      Digest::SHA256.hexdigest(expected_certificate.to_der) &&
    OpenSSL::SSL.verify_certificate_identity(socket.peer_cert, uri.host)

  key = Base64.strict_encode64(SecureRandom.random_bytes(16))
  socket.write(
    "GET #{websocket_path} HTTP/1.1\r\n" \
    "Host: #{uri.host}:#{uri.port}\r\n" \
    "Upgrade: websocket\r\n" \
    "Connection: Upgrade\r\n" \
    "Sec-WebSocket-Version: 13\r\n" \
    "Sec-WebSocket-Key: #{key}\r\n\r\n"
  )
  response = +""
  response << socket.gets until response.end_with?("\r\n\r\n")
  expected_accept = Base64.strict_encode64(
    Digest::SHA1.digest(key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11")
  )
  fail_lane("live WSS upgrade rejected") unless
    response.start_with?("HTTP/1.1 101") &&
    response.downcase.include?("upgrade: websocket") &&
    response.downcase.include?("sec-websocket-accept: #{expected_accept.downcase}")

  opcode, payload = read_websocket_frame(socket)
  feed = JSON.parse(payload)
  fail_lane("live WSS feed rejected") unless
    opcode == 1 && feed["schema"] == "adl.runtime_v3.observatory_feed.v2"

  socket.write(masked_text_frame("{}"))
  control_result = nil
  5.times do
    frame_opcode, frame_payload = read_websocket_frame(socket)
    next unless frame_opcode == 1

    candidate = JSON.parse(frame_payload)
    if candidate["schema"] == "adl.runtime_v3.observatory_ws_control_result.v1"
      control_result = candidate
      break
    end
  end
  fail_lane("live WSS client-to-server response rejected") unless
    control_result&.fetch("status") == "rejected" &&
    control_result["error"] == "write_authentication_required"
  {
    "endpoint" => websocket_path,
    "feed_schema" => feed["schema"],
    "client_message" => "unauthenticated_write_probe",
    "server_response" => control_result["error"]
  }
ensure
  socket&.close
  tcp&.close
end

def cargo_env
  target = Pathname.new(
    ENV.fetch("ADL_WP15_TARGET_DIR", ROOT.join(".adl/target/wp15").to_s)
  ).expand_path
  temporary = Pathname.new(
    ENV.fetch("ADL_WP15_TMPDIR", ROOT.join(".adl/target/wp15-tmp").to_s)
  ).expand_path
  FileUtils.mkdir_p(target)
  FileUtils.mkdir_p(temporary)
  {
    "CARGO_NET_OFFLINE" => "true",
    "CARGO_TARGET_DIR" => target.to_s,
    "TMPDIR" => temporary.to_s
  }
end

def prove_integration
  run!("ruby", ".csdlc/prepared/issues/5354/check-dependencies.rb")
  revision = git("rev-parse", "HEAD")
  assert_ancestor(WP14A_MERGE)
  UNITY_MERGES.each_value { |sha| assert_ancestor(sha) }

  env = cargo_env
  adl_binary = Pathname.new(env.fetch("CARGO_TARGET_DIR")).join("debug/adl-v2")
  unless adl_binary.file? && adl_binary.executable?
    run!(
      "cargo", "build", "--locked", "--offline",
      "--manifest-path", "adl-v2/Cargo.toml",
      "-p", "adl-cli", "--bin", "adl-v2",
      env: env
    )
  end

  plan_raw = run!(adl_binary.to_s, "plan", FIXTURE.relative_path_from(ROOT).to_s, "--yaml")
  run_raw = run!(adl_binary.to_s, "run", FIXTURE.relative_path_from(ROOT).to_s, "--yaml")
  plan = JSON.parse(plan_raw)
  execution = JSON.parse(run_raw)
  fail_lane("ADL v2 plan contract rejected") unless
    plan["schema"] == "adl.plan.v1" &&
    plan["ok"] == true &&
    plan.dig("result", "contract") == "adl.execution-plan.v1"
  fail_lane("ADL v2 engine contract rejected") unless
    execution["schema"] == "adl.run.v1" &&
    execution["ok"] == true &&
    execution.dig("result", "contract") == "adl.engine.v1" &&
    execution.dig("result", "status") == "ready"
  fail_lane("ADL v2 plan/run source identity drifted") unless
    plan.dig("result", "source_digest") == execution.dig("result", "plan", "source_digest")

  run!(
    "cargo", "test", "--locked", "--offline",
    "--manifest-path", "adl-v2/Cargo.toml",
    "-p", "adl-runtime-v3-adapter",
    "canonical_ingress_maps_success_and_result_record",
    "--", "--exact",
    env: env
  )
  run!(
    "cargo", "test", "--locked", "--offline",
    "--manifest-path", "adl-runtime-kernel/Cargo.toml",
    "--test", "observatory",
    "observatory_websocket_allows_public_reads_and_requires_login_for_writes",
    "--", "--exact",
    env: env
  )

  runtime_base = ENV.fetch("ADL_RUNTIME_V3_BASE_URL") {
    fail_lane("ADL_RUNTIME_V3_BASE_URL must name the configured HTTPS Runtime v3 base URL")
  }.sub(%r{/+\z}, "")
  ca_cert = Pathname.new(ENV.fetch("ADL_RUNTIME_V3_CA_CERT") {
    fail_lane("ADL_RUNTIME_V3_CA_CERT must name the trusted Runtime v3 certificate")
  }).expand_path
  fail_lane("Runtime v3 base URL must use HTTPS") unless runtime_base.start_with?("https://")
  fail_lane("Runtime v3 CA certificate is missing") unless ca_cert.file?

  curl = lambda do |path|
    json_command(
      "curl", "--fail", "--silent", "--show-error",
      "--proto", "=https", "--tlsv1.2",
      "--cacert", ca_cert.to_s,
      "#{runtime_base}#{path}"
    )
  end
  health = curl.call("/v1/health")
  observatory = curl.call("/v1/observatory")
  live_websocket = prove_live_websocket(runtime_base, ca_cert)
  components = health.dig("snapshot", "components")
  fail_lane("live Runtime v3 health schema rejected") unless
    health.dig("snapshot", "schema") == "adl.runtime.control_snapshot.v1" &&
    health["observability_ready"] == true &&
    components.is_a?(Hash) &&
    !components.empty? &&
    components.values.all? { |state| state == "running" }
  fail_lane("live Observatory contract rejected") unless
    observatory["schema"] == "adl.runtime_v3.observatory_feed.v2" &&
    observatory.dig("control", "websocket_full_duplex") == true &&
    observatory.dig("control", "bearer_token_required_for_read") == false &&
    observatory.dig("control", "login_required_for_mutation") == true

  selected = JSON.parse(
    run!(installed_binary("csdlc-install"), "resolve", "--repo", ".", "--issue", ISSUE.to_s)
  )
  fail_lane("C-SDLC selector did not resolve v2") unless selected == "v2"
  doctor = json_command(installed_binary("csdlc-doctor"), "--repo", ".", "--issue", ISSUE.to_s)
  fail_lane("typed #5354 doctor rejected") unless
    doctor["status"] == "pass" && Array(doctor["findings"]).empty?
  index = JSON.parse(ROOT.join(".csdlc/issues/5354/index.json").read)
  claim = index.fetch("claim")
  fail_lane("typed claim does not own the convergence surfaces") unless
    [
      "adl-v2/Cargo.lock",
      "docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md",
      "docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md"
    ].all? { |path| claim.fetch("protected_paths").include?(path) }

  unity_proof_digest = sha256(UNITY_PROOF)
  unity_image_digests = UNITY_IMAGES.to_h do |path|
    [path.relative_path_from(ROOT).to_s, sha256(path)]
  end

  packet = {
    "schema" => "adl.v0918.wp15.convergence_proof.v1",
    "issue" => ISSUE,
    "status" => "pass",
    "revision" => revision,
    "predecessor" => {
      "issue" => 5384,
      "pull_request" => 5726,
      "merge_revision" => WP14A_MERGE,
      "closeout_gate" => "not_required"
    },
    "adl_v2" => {
      "fixture" => FIXTURE.relative_path_from(ROOT).to_s,
      "plan_contract" => plan.dig("result", "contract"),
      "source_digest" => plan.dig("result", "source_digest"),
      "plan_output_sha256" => Digest::SHA256.hexdigest(plan_raw),
      "run_contract" => execution.dig("result", "contract"),
      "run_status" => execution.dig("result", "status"),
      "checkpoint_contract" => execution.dig("result", "snapshot", "checkpoint_contract"),
      "runtime_v2_used" => false
    },
    "runtime_v3" => {
      "adapter_test" => "canonical_ingress_maps_success_and_result_record",
      "wss_test" => "observatory_websocket_allows_public_reads_and_requires_login_for_writes",
      "live_wss" => live_websocket,
      "live_schema" => observatory["schema"],
      "runtime_instance_id" => observatory["runtime_instance_id"],
      "lifecycle" => health.dig("snapshot", "lifecycle"),
      "components_running" => components.length,
      "observability_ready" => health["observability_ready"],
      "websocket_full_duplex" => observatory.dig("control", "websocket_full_duplex"),
      "public_reads" => !observatory.dig("control", "bearer_token_required_for_read"),
      "login_required_for_mutation" => observatory.dig("control", "login_required_for_mutation"),
      "continuity_generation" => observatory.dig("continuity", "checkpoint", "generation")
    },
    "csdlc_v2" => {
      "selected_generation" => selected,
      "phase" => doctor["phase"],
      "generation" => doctor["generation"],
      "claim_id" => claim["id"],
      "doctor_status" => doctor["status"],
      "closeout_blocks_execution" => false
    },
    "unity" => {
      "merged_children" => UNITY_MERGES.map do |issue, merge|
        {"issue" => issue, "merge_revision" => merge}
      end,
      "live_proof" => UNITY_PROOF.relative_path_from(ROOT).to_s,
      "live_proof_sha256" => unity_proof_digest,
      "image_sha256" => unity_image_digests,
      "claim" => "accepted editor, Play Mode, and presentation proof",
      "non_claim" => "retained images do not prove player-build readiness or live Runtime/cloud authority"
    },
    "claim_boundaries" => [
      {"claim" => "ADL v2 validates, compiles, and initializes the accepted engine plan", "disposition" => "proven"},
      {"claim" => "ADL v2 dispatch reaches Runtime v3 canonical ingress", "disposition" => "proven"},
      {"claim" => "Runtime v3 serves TLS-verified public reads and full-duplex WSS while requiring login for writes", "disposition" => "proven"},
      {"claim" => "C-SDLC v2 is the selected typed lifecycle and governs this issue", "disposition" => "proven"},
      {"claim" => "Unity editor, Play Mode, and presentation proof is integrated", "disposition" => "proven"},
      {"claim" => "Runtime v2 participates in WP-15", "disposition" => "explicit_non_claim"},
      {"claim" => "Unity images prove player-build, Runtime, or cloud authority", "disposition" => "explicit_non_claim"},
      {"claim" => "WP-15 independently proves every v0.91.8 release feature", "disposition" => "explicit_non_claim"}
    ]
  }

  encoded = JSON.pretty_generate(packet) + "\n"
  %r{/(Users|Volumes|private/tmp)/}.match?(encoded) &&
    fail_lane("retained packet contains a host-absolute path")
  encoded.match?(/private[_ -]?key|bearer[_ -]?token|credential/i) &&
    fail_lane("retained packet contains credential-bearing material")
  FileUtils.mkdir_p(PACKET.dirname)
  PACKET.write(encoded)
  packet
rescue JSON::ParserError, KeyError => e
  fail_lane("invalid convergence data: #{e.message}")
end

def prove_matrices
  fail_lane("missing convergence packet") unless PACKET.file?
  packet = JSON.parse(PACKET.read)
  fail_lane("convergence packet is not passing") unless
    packet["schema"] == "adl.v0918.wp15.convergence_proof.v1" &&
    packet["issue"] == ISSUE &&
    packet["status"] == "pass"
  proof_revision = packet.fetch("revision")
  _out, _err, status = Open3.capture3(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", proof_revision, "HEAD"
  )
  fail_lane("convergence packet revision is not ancestral to HEAD") unless status.success?
  post_proof_paths = git("diff", "--name-only", "#{proof_revision}..HEAD").lines.map(&:strip).reject(&:empty?)
  disallowed = post_proof_paths.reject do |path|
    POST_PROOF_METADATA_PREFIXES.any? { |allowed| path == allowed || path.start_with?(allowed) }
  end
  fail_lane("convergence packet is stale after substantive changes: #{disallowed.join(', ')}") unless disallowed.empty?
  live_wss = packet.dig("runtime_v3", "live_wss")
  fail_lane("convergence packet lacks a real live WSS exchange") unless
    live_wss == {
      "endpoint" => "/v1/observatory/ws",
      "feed_schema" => "adl.runtime_v3.observatory_feed.v2",
      "client_message" => "unauthenticated_write_probe",
      "server_response" => "write_authentication_required"
    }
  dispositions = packet.fetch("claim_boundaries").map { |entry| entry.fetch("disposition") }
  fail_lane("claim-boundary packet lacks proven claims") unless dispositions.include?("proven")
  fail_lane("claim-boundary packet lacks explicit non-claims") unless dispositions.include?("explicit_non_claim")

  [DEMO_MATRIX, FEATURE_MATRIX].each do |path|
    text = path.read
    fail_lane("#{path.basename} does not cite the convergence packet") unless
      text.include?(".csdlc/evidence/5354/convergence-proof.v1.json")
    fail_lane("#{path.basename} does not preserve Unity non-claims") unless
      text.include?("player-build") && text.include?("Runtime") && text.include?("cloud")
  end
  packet
rescue JSON::ParserError, KeyError => e
  fail_lane("invalid matrix evidence: #{e.message}")
end

lane = ARGV.fetch(0) { fail_lane("usage: run-validation-lane.rb LANE") }
fail_lane("unknown validation lane: #{lane}") unless ALLOWED_LANES.include?(lane)

case lane
when "integrated-live-demo"
  packet = prove_integration
when "claim-boundary-matrix"
  packet = prove_matrices
when "complete", "post-merge-exact"
  packet = prove_integration
  prove_matrices
end

puts JSON.generate(
  status: "pass",
  issue: ISSUE,
  lane: lane,
  revision: packet.fetch("revision"),
  packet: PACKET.relative_path_from(ROOT).to_s
)
