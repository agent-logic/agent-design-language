# Issue 602 live Wuji acceptance

- Candidate revision: `4cd16d5ad38a7520f12a1b328146da1eae87dd38`
- Host: Wuji
- Isolated Runtime listener: `127.0.0.1:21997`
- Runtime instance before restart: `877eec6fb44b414e8c1086cefeafb588`
- Agent configuration: canonical name `ember.axioma`, display name `Ember Axioma`, provider `ollama`, model `gemma4:e4b-mlx`
- Candidate binary SHA-256:
  - `adl-runtime-kernel`: `a7ea339c8480aece4f2986b2d403fa099f05a122c2c855b02086d7fbefa44465`
  - `adl-runtime-guardian`: `08ae4f1968606878958e4b97281f3418e5cd7aa69453da32296da99b8a0ba8e9`
  - `csmctl`: `96abdc7fa7ba07720048e22543596ca07ce9df4b327c053d9ce67c141f4fc559`

## Observed results

1. Candidate startup admitted the configured Shepherd and reported it healthy, ready, available, and communication-eligible.
2. `csmctl agent add --config <config-file>` admitted `ember-axioma`.
3. Repeating the add returned `already_present` without creating a duplicate.
4. `csmctl agent checkpoint ember-axioma` created a checkpoint with digest `593bf43931776679bb78ea38f9e2df739b7fbcca89b80fbb22b34687b202c640`.
5. `csmctl agent migrate ember-axioma` created a freeze-dried bundle with digest `b9c6707fbc124764d0f6dc075bb5af24d14ce9b7133785dee78b2fd2dfb28459` and removed the source resident.
6. `csmctl agent rehydrate` restored the resident from that bundle.
7. Before restart, the roster contained exactly Shepherd and Ember Axioma; both were healthy, ready, available, and communication-eligible.
8. Guardian shutdown produced terminal state `shutdown_checkpointed`, the Runtime child exited successfully, and `clean_checkpointed_shutdown` was true.
9. Restarting the same exact candidate restored Shepherd and Ember Axioma from persisted state. Both again reported healthy, ready, available, and communication-eligible; `agent get ember-axioma` reported healthy.

The acceptance run used an isolated state root and listener so it did not replace or interrupt the permanent Runtime on port 20997. Credentials, TLS material, generated configuration, checkpoint payloads, and freeze-dried payloads are intentionally excluded from retained repository evidence.

## Residual observation

Guardian shutdown also logged `master_log_drain_incomplete` for the observability drain. The Runtime still completed the clean checkpointed shutdown and restored both agents. This observation is outside #602's agent-lifecycle acceptance surface and is not claimed as fixed here.
