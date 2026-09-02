#!/usr/bin/env bash
set -euo pipefail

: "${ADL_VERTEX_GCP_KEY:?set ADL_VERTEX_GCP_KEY to the approved service account key path}"
: "${ADL_VERTEX_GCP_PROJECT:=cs-poc-cha8mmii0xk0iaw5vpf8mxf}"

proof_dir=".csdlc/evidence/608/live-vertex"
gcloud_config="${proof_dir}/gcloud-config"

mkdir -p "${gcloud_config}"
export CLOUDSDK_CONFIG="${gcloud_config}"
if ! gcloud auth activate-service-account --key-file "${ADL_VERTEX_GCP_KEY}" --project "${ADL_VERTEX_GCP_PROJECT}" >/dev/null 2>&1; then
  echo "gcloud auth activate-service-account failed for configured Vertex proof key" >&2
  exit 1
fi

cargo build --manifest-path adl/Cargo.toml -p adl
./adl/target/debug/adl "${proof_dir}/vertex_2_5_model_matrix_native_endpoint.adl.yaml" --run --trace --quiet --out "${proof_dir}/2-5-native-endpoint-out"
./adl/target/debug/adl "${proof_dir}/vertex_global_3x_model_matrix_native_endpoint.adl.yaml" --run --trace --quiet --out "${proof_dir}/global-3x-native-endpoint-out"

rm -rf -- "${gcloud_config}"
