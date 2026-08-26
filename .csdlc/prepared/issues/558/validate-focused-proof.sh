#!/usr/bin/env bash
set -euo pipefail

cargo test \
  --locked \
  --manifest-path adl-runtime/Cargo.toml \
  --lib \
  distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication \
  -- \
  --nocapture

cargo llvm-cov test \
  --locked \
  --manifest-path adl-runtime/Cargo.toml \
  --lib \
  distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication \
  -- \
  --nocapture
