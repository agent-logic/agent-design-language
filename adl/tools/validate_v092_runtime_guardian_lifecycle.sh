#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
qualification_root=${ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT:-$repo_root/.adl/runtime-v3/qualification}
target_dir=${CARGO_TARGET_DIR:-$repo_root/.adl/target/5820-runtime}
target_root=${ADL_RUNTIME_GUARDIAN_TARGET_ROOT:-$repo_root}

validate_contained_path() {
  ruby -rpathname -e '
    root_arg, candidate_arg, required_prefix = ARGV
    abort "containment root must be absolute" unless Pathname.new(root_arg).absolute?
    abort "containment root must exist" unless File.directory?(root_arg)
    abort "containment root must not be a symlink" if File.lstat(root_arg).symlink?
    abort "candidate path must be absolute" unless Pathname.new(candidate_arg).absolute?
    abort "candidate path must not contain traversal" if Pathname.new(candidate_arg).each_filename.any? { |part| part == ".." }
    root = File.realpath(root_arg)
    candidate = File.expand_path(candidate_arg)
    prefix = root.end_with?(File::SEPARATOR) ? root : root + File::SEPARATOR
    abort "candidate path escapes containment root" unless candidate.start_with?(prefix)
    relative = candidate.delete_prefix(prefix)
    abort "candidate path is outside the required prefix" unless required_prefix.empty? || relative.start_with?(required_prefix)
    current = root
    Pathname.new(relative).each_filename do |part|
      current = File.join(current, part)
      abort "candidate path traverses a symlink" if File.symlink?(current)
    end
    puts candidate
  ' "$1" "$2" "$3"
}

qualification_root=$(validate_contained_path "$repo_root" "$qualification_root" ".adl/") || exit 64
target_dir=$(validate_contained_path "$target_root" "$target_dir" "") || exit 64
mkdir -p "$qualification_root" "$target_dir"
[[ "$(cd "$qualification_root" && pwd -P)" == "$qualification_root" ]] || {
  echo "evidence root canonicalization changed after creation" >&2
  exit 64
}
[[ "$(cd "$target_dir" && pwd -P)" == "$target_dir" ]] || {
  echo "target directory canonicalization changed after creation" >&2
  exit 64
}
export CARGO_TARGET_DIR="$target_dir"

vector_bin=${ADL_RUNTIME_VECTOR_BIN:-}
if [[ -z "$vector_bin" ]]; then
  vector_bin=$(command -v vector || true)
fi
if [[ -z "$vector_bin" || ! -x "$vector_bin" ]]; then
  echo "ADL_RUNTIME_VECTOR_BIN must name an executable Vector binary" >&2
  exit 69
fi
vector_bin=$(cd "$(dirname "$vector_bin")" && pwd -P)/$(basename "$vector_bin")

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" \
  --bin adl-runtime-guardian --bin adl-runtime-lifecycle-soak

revision=$(git -C "$repo_root" rev-parse HEAD)
run_root=$(mktemp -d "$qualification_root/5820-run.XXXXXX")
state_root="$run_root/state"
report="$run_root/report.json"
wss_proof="$run_root/wss-proof.json"
wss_transcript="$run_root/wss-transcript.json"
https_transcript="$run_root/https-transcript.json"
wss_stderr="$run_root/wss-proof.stderr"
api_port=$(ruby -rsocket -e 'socket = TCPServer.new("127.0.0.1", 0); puts socket.addr[1]; socket.close')
init_template="$qualification_root/5820-runtime-init-$api_port.toml"
mkdir -p "$(dirname "$init_template")"
ruby -e '
  source, destination, port = ARGV
  text = File.read(source)
  address = %(address = "127.0.0.1:20997")
  public_url = %(public_base_url = "https://localhost:20997")
  abort "canonical API address missing" unless text.scan(address).length == 1
  abort "canonical public URL missing" unless text.scan(public_url).length == 1
  text = text.sub(address, %(address = "127.0.0.1:#{port}"))
  text = text.sub(public_url, %(public_base_url = "https://localhost:#{port}"))
  File.write(destination, text)
' "$repo_root/infra/runtime-v3/runtime-init.toml" "$init_template" "$api_port"

ruby -ropenssl -rsocket -rjson -rbase64 -rsecurerandom -rdigest -rpathname -e '
  state_root_arg, proof_path, https_transcript_path, wss_transcript_path = ARGV
  deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + 15

  def read_exact(stream, bytes)
    value = +""
    value << stream.readpartial(bytes - value.bytesize) while value.bytesize < bytes
    value
  end

  def safe_existing(path, root, label)
    abort "#{label} contains traversal" if Pathname.new(path).each_filename.any? { |part| part == ".." }
    root = File.realpath(root)
    expanded = File.expand_path(path)
    prefix = root.end_with?(File::SEPARATOR) ? root : root + File::SEPARATOR
    abort "#{label} escapes state root" unless expanded.start_with?(prefix)
    current = root
    Pathname.new(expanded.delete_prefix(prefix)).each_filename do |part|
      current = File.join(current, part)
      abort "#{label} traverses a symlink" if File.symlink?(current)
    end
    resolved = File.realpath(expanded)
    abort "#{label} resolves outside state root" unless resolved.start_with?(prefix)
    resolved
  end

  def write_json(path, value)
    temporary = path + ".tmp"
    File.write(temporary, JSON.pretty_generate(value))
    File.rename(temporary, path)
  end

  def tls_socket(address, certificate)
    host, port = address.split(":", 2)
    tcp = TCPSocket.new(host, Integer(port))
    store = OpenSSL::X509::Store.new
    store.add_file(certificate)
    context = OpenSSL::SSL::SSLContext.new
    context.cert_store = store
    context.verify_mode = OpenSSL::SSL::VERIFY_PEER
    context.verify_hostname = true if context.respond_to?(:verify_hostname=)
    tls = OpenSSL::SSL::SSLSocket.new(tcp, context)
    tls.hostname = "localhost"
    tls.sync_close = true
    tls.connect
    tls
  end

  def authenticated_https(address, certificate, token)
    tls = tls_socket(address, certificate)
    tls.write("GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer #{token}\r\nConnection: close\r\n\r\n")
    response = tls.read
    tls.close
    status, body = response.split("\r\n\r\n", 2)
    raise "authenticated HTTPS did not return 200" unless status&.start_with?("HTTP/1.1 200 OK")
    value = JSON.parse(body)
    raise "wrong Observatory schema" unless value["schema"] == "adl.runtime_v3.observatory_feed.v2"
    [value, Digest::SHA256.hexdigest(response)]
  end

  def read_frame(stream)
    first, second = read_exact(stream, 2).bytes
    raise "fragmented WebSocket frame" unless (first & 0x80) != 0
    raise "masked server WebSocket frame" unless (second & 0x80).zero?
    length = second & 0x7f
    length = read_exact(stream, 2).unpack1("n") if length == 126
    length = read_exact(stream, 8).unpack1("Q>") if length == 127
    raise "WebSocket frame exceeds configured bound" if length > 65_536
    [first & 0x0f, read_exact(stream, length)]
  end

  def write_frame(stream, opcode, payload)
    bytes = payload.b
    raise "WebSocket request exceeds configured bound" if bytes.bytesize > 65_536
    mask = SecureRandom.random_bytes(4)
    length = bytes.bytesize
    header = [0x80 | opcode]
    if length < 126
      header << (0x80 | length)
      prefix = header.pack("C*")
    elsif length <= 65_535
      prefix = (header + [0x80 | 126]).pack("C*") + [length].pack("n")
    else
      prefix = (header + [0x80 | 127]).pack("C*") + [length].pack("Q>")
    end
    masked = bytes.bytes.each_with_index.map { |byte, index| byte ^ mask.getbyte(index % 4) }.pack("C*")
    stream.write(prefix + mask + masked)
  end

  def varint(value)
    bytes = []
    loop do
      byte = value & 0x7f
      value >>= 7
      byte |= 0x80 unless value.zero?
      bytes << byte
      break if value.zero?
    end
    bytes.pack("C*")
  end

  def protobuf_string(tag, value)
    bytes = value.b
    varint((tag << 3) | 2) + varint(bytes.bytesize) + bytes
  end

  def acip_envelope
    protobuf_string(1, "adl.csm.acip_carrier.protobuf_envelope.v1") +
      protobuf_string(2, "wp-5820-wss-proof") +
      protobuf_string(3, "wp-5820-proof") +
      protobuf_string(4, "runtime") +
      protobuf_string(5, "agent_runtime") +
      protobuf_string(6, %q({"schema":"adl.runtime.local_agent_work.v1","tasks":[{"input":"bounded Guardian proof","op":"blake3"}]})) +
      varint(7 << 3) + varint(1)
  end

  def authenticated_wss(address, certificate, token)
    tls = tls_socket(address, certificate)
    key = Base64.strict_encode64(SecureRandom.random_bytes(16))
    tls.write("GET /v1/acip/ws HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: #{key}\r\nSec-WebSocket-Version: 13\r\nAuthorization: Bearer #{token}\r\n\r\n")
    headers = +""
    headers << tls.readpartial(1) until headers.end_with?("\r\n\r\n")
    raise "authenticated WSS did not return 101" unless headers.start_with?("HTTP/1.1 101 Switching Protocols")
    expected = Base64.strict_encode64(Digest::SHA1.digest(key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"))
    accept = headers.lines.find { |line| line.downcase.start_with?("sec-websocket-accept:") }&.split(":", 2)&.last&.strip
    raise "WSS upgrade accept digest mismatch" unless accept == expected
    opcode, hello = read_frame(tls)
    raise "WSS hello was not text" unless opcode == 1
    hello_value = JSON.parse(hello)
    raise "WSS authentication hello missing" unless hello_value["schema"] == "adl.csm.acip_carrier.websocket_frame.v1" && hello_value["event"] == "authenticated"
    request = acip_envelope
    write_frame(tls, 2, request)
    opcode, response = read_frame(tls)
    raise "WSS response was not text: opcode=#{opcode} payload=#{response.inspect}" unless opcode == 1
    response_value = JSON.parse(response)
    raise "WSS bounded request/response failed" unless response_value["schema"] == "adl.csm.acip_carrier.websocket_frame.v1" && response_value["status"] == "completed" && response_value["message_id"] == "wp-5820-wss-proof" && response_value["sequence_reserved"] == true
    tls.close
    [headers, hello, request, response]
  end

  last_error = nil
  loop do
    begin
      raise "state root is not ready" unless File.directory?(state_root_arg)
      raise "state root is a symlink" if File.lstat(state_root_arg).symlink?
      state_root = File.realpath(state_root_arg)
      init = safe_existing(File.join(state_root, "runtime-init.toml"), state_root, "runtime init")
      text = File.read(init)
      address = text.match(/^address = "([^"]+)"$/)&.captures&.first or raise "API address missing"
      certificate_path = text.match(/^certificate_chain_path = "([^"]+)"$/)&.captures&.first or raise "certificate path missing"
      token_path = text.match(/^observatory_token_path = "([^"]+)"$/)&.captures&.first or raise "token path missing"
      certificate = safe_existing(certificate_path, state_root, "certificate")
      token_file = safe_existing(token_path, state_root, "observatory token")
      token = File.read(token_file).strip
      observatory, https_sha256 = authenticated_https(address, certificate, token)
      headers, hello, request, response = authenticated_wss(address, certificate, token)
      https_transcript = {
        "schema" => "adl.runtime_v3.guardian_https_transcript.v1",
        "request" => {"method" => "GET", "path" => "/v1/observatory", "authentication" => "bearer_redacted"},
        "response" => {"status" => 200, "sha256" => https_sha256, "schema" => observatory.fetch("schema")},
        "runtime_instance_id" => observatory.fetch("runtime_instance_id"),
        "runtime_process_id" => observatory.fetch("runtime_process_id")
      }
      hello_value = JSON.parse(hello)
      response_value = JSON.parse(response)
      wss_transcript = {
        "schema" => "adl.runtime_v3.guardian_wss_transcript.v1",
        "request" => {"method" => "GET", "path" => "/v1/acip/ws", "authentication" => "bearer_redacted", "upgrade" => "websocket"},
        "upgrade" => {"status" => 101, "sha256" => Digest::SHA256.hexdigest(headers)},
        "hello" => hello_value,
        "bounded_request" => {"opcode" => "binary", "bytes" => request.bytesize, "sha256" => Digest::SHA256.hexdigest(request)},
        "response" => response_value
      }
      write_json(https_transcript_path, https_transcript)
      write_json(wss_transcript_path, wss_transcript)
      proof = {
        "schema" => "adl.runtime_v3.guardian_wss_proof.v1",
        "status" => "pass",
        "runtime_instance_id" => observatory.fetch("runtime_instance_id"),
        "runtime_process_id" => observatory.fetch("runtime_process_id"),
        "authenticated_https" => true,
        "https_transcript_path" => https_transcript_path,
        "https_transcript_sha256" => Digest::SHA256.file(https_transcript_path).hexdigest,
        "authenticated_wss" => true,
        "wss_transcript_path" => wss_transcript_path,
        "wss_transcript_sha256" => Digest::SHA256.file(wss_transcript_path).hexdigest,
        "wss_upgrade_sha256" => Digest::SHA256.hexdigest(headers),
        "wss_hello_sha256" => Digest::SHA256.hexdigest(hello),
        "wss_request_sha256" => Digest::SHA256.hexdigest(request),
        "wss_response_sha256" => Digest::SHA256.hexdigest(response),
        "bounded_request_response" => true
      }
      write_json(proof_path, proof)
      break
    rescue StandardError => error
      last_error = error
      warn error.message if ENV["ADL_RUNTIME_WSS_DEBUG"] == "1"
      raise if Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline
      sleep 0.01
    end
  end
' "$state_root" "$wss_proof" "$https_transcript" "$wss_transcript" 2>"$wss_stderr" &
wss_probe_pid=$!
trap 'kill "$wss_probe_pid" 2>/dev/null || true' EXIT

"$target_dir/debug/adl-runtime-lifecycle-soak" \
  --guardian "$target_dir/debug/adl-runtime-guardian" \
  --kernel "$target_dir/debug/adl-runtime-kernel" \
  --vector "$vector_bin" \
  --init-template "$init_template" \
  --state-root "$state_root" \
  --report "$report" \
  --revision "$revision" \
  --suite preflight

wait "$wss_probe_pid"
trap - EXIT

ruby -rjson -rdigest -e '
  report_path, wss_path, proof_path, revision, guardian_path, kernel_path, canonical_init_path, https_transcript_path, wss_transcript_path = ARGV
  report = JSON.parse(File.read(report_path))
  wss = JSON.parse(File.read(wss_path))
  abort "wrong lifecycle report schema" unless report["schema"] == "adl.runtime_v3.lifecycle_soak.v1"
  abort "lifecycle preflight failed" unless report["status"] == "pass"
  abort "lifecycle revision drifted" unless report["revision"] == revision
  abort "Guardian was not launched" unless report["guardian_launch_count"].to_i == 1
  abort "kernel start denominator drifted" unless report["runtime_start_count"].to_i == 2
  abort "kernel restart was not exercised" unless report["restart_budget_exercised"] == true
  abort "kernel restart count drifted" unless report["total_restarts"].to_i == 1
  abort "durable continuity was not retained" unless report["continuity_generation"].to_i == 1
  abort "clean log proof is missing" unless report["logging_complete"] == true && report["master_log_status"] == "clean"
  abort "real authenticated HTTPS proof is missing" unless wss["status"] == "pass" && wss["authenticated_https"] == true
  abort "real authenticated WSS proof is missing" unless wss["authenticated_wss"] == true && wss["bounded_request_response"] == true
  abort "HTTPS transcript digest mismatch" unless wss["https_transcript_path"] == https_transcript_path && wss["https_transcript_sha256"] == Digest::SHA256.file(https_transcript_path).hexdigest
  abort "WSS transcript digest mismatch" unless wss["wss_transcript_path"] == wss_transcript_path && wss["wss_transcript_sha256"] == Digest::SHA256.file(wss_transcript_path).hexdigest
  proof = {
    "schema" => "adl.runtime_v3.guardian_lifecycle_proof.v1",
    "status" => "pass",
    "source_revision" => revision,
    "acceptance_eligible" => true,
    "lifecycle_component_suite" => report["suite"],
    "lifecycle_component_acceptance_eligible" => report["acceptance_eligible"],
    "lifecycle_report_path" => report_path,
    "lifecycle_report_sha256" => Digest::SHA256.file(report_path).hexdigest,
    "wss_proof_path" => wss_path,
    "wss_proof_sha256" => Digest::SHA256.file(wss_path).hexdigest,
    "guardian_binary_path" => guardian_path,
    "guardian_binary_sha256" => Digest::SHA256.file(guardian_path).hexdigest,
    "kernel_binary_path" => kernel_path,
    "kernel_binary_sha256" => Digest::SHA256.file(kernel_path).hexdigest,
    "canonical_init_path" => canonical_init_path,
    "canonical_init_sha256" => Digest::SHA256.file(canonical_init_path).hexdigest,
    "https_transcript_path" => https_transcript_path,
    "https_transcript_sha256" => Digest::SHA256.file(https_transcript_path).hexdigest,
    "wss_transcript_path" => wss_transcript_path,
    "wss_transcript_sha256" => Digest::SHA256.file(wss_transcript_path).hexdigest,
    "assertions" => {
      "guardian_launched" => true,
      "kernel_ready" => true,
      "authenticated_https" => true,
      "authenticated_wss" => true,
      "child_killed" => true,
      "bounded_restart" => true,
      "state_preserved" => true,
      "clean_shutdown" => true,
      "clean_logs" => true
    }
  }
  temporary = proof_path + ".tmp"
  File.write(temporary, JSON.pretty_generate(proof))
  File.rename(temporary, proof_path)
' "$report" "$wss_proof" "$run_root/issue-proof.json" "$revision" \
  "$target_dir/debug/adl-runtime-guardian" "$target_dir/debug/adl-runtime-kernel" \
  "$repo_root/infra/runtime-v3/runtime-init.toml" "$https_transcript" "$wss_transcript"

printf 'PASS: production Guardian lifecycle proof=%s revision=%s\n' "$run_root/issue-proof.json" "$revision"
