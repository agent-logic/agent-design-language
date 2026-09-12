# #903 bounded MLX execution plan

The implementation consumes #876's canonical provider definition and registered
ADL workflow Runtime dispatch. Resident-kernel MLX conversation routing is not
part of this result. No actual Metal proof has run yet.

## Selected local smoke

- Host: operator Apple M4 Pro, 64 GiB unified memory.
- Isolated issue-local Python environment: MLX-LM 0.31.3, MLX/Metal 0.32.2.
- Existing cached model: mlx-community/Llama-3.2-3B-Instruct-4bit at
  7f0dc925e0d0afb0322d96f9255cfddf2ba5636e (1,807,496,278 weight bytes).
- HF_HUB_OFFLINE=1 and TRANSFORMERS_OFFLINE=1; use exact existing local snapshot.
  No model download. The server is separately owned by this smoke, on
  127.0.0.1:18093, and is terminated after proof or a 180-second lifetime.
- One request, short fixed greeting prompt, 16 output tokens, 60-second client
  deadline; one prompt/decode worker, one prompt cache, 64 MiB KV cache limit.
- MLX allocation guideline 8 GiB and cache limit 128 MiB. The MLX memory limit is
  advisory, not an OS-enforced RAM ceiling; do not claim a hard memory cap.
- Pin candidate and exact sidecar hash before starting. Retain package versions,
  hardware/OS identity, input/output hashes, nonempty output, elapsed time and
  owned-process termination. No other live provider configuration is changed.

Invoke the ignored mlx_actual_hardware_production_workflow test in
adl/tests/mlx_provider.rs with all explicit ADL_MLX_* inputs. It calls
execute_sequential_with_provider_reload_handle, not the direct provider fixture.

## Failure boundaries

Deterministic tests separately cover invalid endpoint/model/limits, unavailable
service, HTTP errors, malformed or alternate-model responses, byte bounds and
client timeout. Unsupported host builds fail before transport. Client deadline
expiry is not proof that an independently owned MLX server stopped generating.
The smoke supervisor must terminate its own server, retaining this distinction.

## References

- https://github.com/ml-explore/mlx-lm/blob/main/mlx_lm/SERVER.md
- Installed pinned mlx_lm/server.py: nonstream generation completes before reply;
  server-side model selection can download weights unless offline.
- Installed MLX core type stubs: set_memory_limit is an allocation guideline.
