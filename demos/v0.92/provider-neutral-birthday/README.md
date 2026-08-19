# Provider-neutral birthday proof

Issue #341 retains the WP-18B provider-neutral proof matrix here. Local reference mode is deterministic validator proof only. Live positive mode requires at least two approved real provider credentials and records only redacted metadata and digests.

Each proof mode starts local ACIP/TCP provider-agent listeners, sends versioned ACIP envelopes, and retains redacted `acip-trace-*.json` files. The trace files record envelope and receipt digests plus listener/probe events, not raw prompts, raw outputs, or credential material.

Local deterministic proof:

```bash
bash adl/tools/test_v092_provider_neutral_proof.sh
```

Live provider proof, using approved key-file environment variables:

```bash
ADL_OPENAI_API_KEY_FILE="$HOME/keys/openai2.key" \
ADL_ANTHROPIC_API_KEY_FILE="$HOME/keys/claude2.key" \
ADL_ISSUE341_ANTHROPIC_MODEL=claude-opus-5 \
bash adl/tools/demo_v092_provider_neutral_birthday.sh --mode positive

python3 adl/tools/validate_v092_provider_neutral_proof.py \
  demos/v0.92/provider-neutral-birthday/proof-matrix-positive.json \
  --require-live
```

Private Observatory proof:

```bash
bash adl/tools/demo_v092_provider_neutral_birthday.sh --mode observatory

python3 adl/tools/validate_v092_provider_neutral_proof.py \
  demos/v0.92/provider-neutral-birthday/proof-matrix-observatory.json \
  --require-observatory
```
