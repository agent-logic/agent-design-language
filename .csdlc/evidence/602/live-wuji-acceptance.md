# Issue 602 live Wuji acceptance

- Candidate revision: `fec04696f6c6754f224c0cd25cb1ac496c00e89b`
- Host: Wuji
- Isolated Runtime listener: `127.0.0.1:21997`
- Runtime instance before restart: `877eec6fb44b414e8c1086cefeafb588`
- Agent configuration: canonical name `ember.axioma`, display name `Ember Axioma`, provider `ollama`, model `gemma4:e4b-mlx`
- Candidate binary SHA-256:
  - `adl-runtime-kernel`: `0961de3ef4a259db51d1ca2893a1471360af873f8b486d14eb88c3ebc385d96e`
  - `adl-runtime-guardian`: `82a22334bf0a6fa943608c5e99cbdb5abc0106aab01bcb6ac2f12e4bb6252702`
  - `csmctl`: `73bf4607fbaf64c677dfe9fdb58b998743affad659b312a2d6b4952159621205`

## Observed results

1. Candidate startup admitted the configured Shepherd and reported it healthy, ready, available, and communication-eligible.
2. `csmctl agent add --config <config-file>` admitted `ember-axioma`.
3. Repeating the add returned `already_present` without creating a duplicate.
4. `csmctl agent checkpoint ember-axioma` created a checkpoint with digest `593bf43931776679bb78ea38f9e2df739b7fbcca89b80fbb22b34687b202c640`.
5. `csmctl agent migrate ember-axioma` created a freeze-dried bundle with digest `b9c6707fbc124764d0f6dc075bb5af24d14ce9b7133785dee78b2fd2dfb28459` and removed the source resident.
6. `csmctl agent rehydrate` restored the resident from that bundle.
7. An authenticated Observatory WSS conversation sent turn `issue-602-gemma4-e4b-mlx` to `ember-axioma` with correlation ID `60260260260260260260260260260260`. The Runtime first returned `accepted`, then returned `delivered` with a nonempty reply exactly matching `EMBER-602-OK`. This exercised the configured local Ollama model `gemma4:e4b-mlx` through the governed conversation path.
8. Before restart, the roster contained exactly Shepherd and Ember Axioma; both were healthy, ready, available, and communication-eligible.
9. Guardian shutdown produced terminal state `shutdown_checkpointed`, the Runtime child exited successfully, and `clean_checkpointed_shutdown` was true.
10. Restarting the same exact candidate restored Shepherd and Ember Axioma from persisted state. Both again reported healthy, ready, available, and communication-eligible; the Shepherd source revision was the exact candidate revision above.

The acceptance run used an isolated state root and listener so it did not replace or interrupt the permanent Runtime on port 20997. Credentials, TLS material, generated configuration, checkpoint payloads, and freeze-dried payloads are intentionally excluded from retained repository evidence.

## Residual observation

Guardian shutdown also logged `master_log_drain_incomplete` for the observability drain. The Runtime still completed the clean checkpointed shutdown and restored both agents. This observation is outside #602's agent-lifecycle acceptance surface and is not claimed as fixed here.
