# Issue 602 live Wuji acceptance

- Candidate revision: `30d8b36a9d0afc92da790a5d8e83dd63b9ca3f1d`
- Host: Wuji
- Isolated Runtime listener: `127.0.0.1:21997`
- Runtime instance before restart: `aed88ac17aaf4744b596d8e1d0ee25ea`
- Agent configuration: canonical name `ember.axioma`, display name `Ember Axioma`, provider `ollama`, model `gemma4:e4b-mlx`
- Candidate binary SHA-256:
  - `adl-runtime-kernel`: `263b19072c34ac2796f7a30e726f209be3306c6737ee9e56ec710e095a39e238`
  - `adl-runtime-guardian`: `d5346cf466fafb1fd73dc086f4b9fada9066607a831d889645885aedf27928d9`
  - `csmctl`: `6ef7fbe563651c2336a3f266cceb27ee2aec49e62782246448de377f6bcf8b57`

## Observed results

1. Candidate startup admitted the configured Shepherd and reported it healthy, ready, available, and communication-eligible.
2. `csmctl agent add --config <config-file>` admitted `ember-axioma`.
3. Repeating the add returned `already_present` without creating a duplicate.
4. `csmctl agent checkpoint ember-axioma` created a checkpoint with digest `28f7a40841c35bf0b35035ccb276d7b28cbf2fcb689b5d006df46b28047db8ba`.
5. `csmctl agent migrate ember-axioma` created a freeze-dried bundle with digest `b154d576d8d47a88e4a1cc034e6257b4aa689283794d344f63f90351ac98150e` and removed the source resident.
6. `csmctl agent rehydrate` restored the resident from that bundle.
7. An authenticated Observatory WSS conversation sent turn `issue-602-gemma4-e4b-mlx` to `ember-axioma` with correlation ID `60260260260260260260260260260261`. The Runtime first returned `accepted`, then returned `delivered` with a nonempty reply exactly matching `EMBER-602-OK`. This exercised the configured local Ollama model `gemma4:e4b-mlx` through the governed conversation path with the candidate's independent 10-minute queue-admission allowance and 15-minute provider-execution allowance.
8. Before restart, the roster contained exactly Shepherd and Ember Axioma; both were healthy, ready, available, and communication-eligible.
9. Guardian shutdown produced terminal state `shutdown_checkpointed`, the Runtime child exited successfully, and `clean_checkpointed_shutdown` was true.
10. Restarting the same exact candidate restored Shepherd and Ember Axioma from persisted state. Both again reported healthy, ready, available, and communication-eligible; the Shepherd source revision was the exact candidate revision above.

The acceptance run used an isolated state root and listener so it did not replace or interrupt the permanent Runtime on port 20997. Credentials, TLS material, generated configuration, checkpoint payloads, and freeze-dried payloads are intentionally excluded from retained repository evidence.

## Residual observation

The isolated proof's Vector exporter did not become ready, so readiness reported the bounded `observability_not_ready` degradation while the Runtime remained available. Both Guardian shutdowns still completed with `shutdown_checkpointed` and `clean_checkpointed_shutdown: true`, and restart restored both agents. This isolated observability condition is outside #602's agent-lifecycle and conversation-timeout acceptance surface and is not claimed as fixed here.
