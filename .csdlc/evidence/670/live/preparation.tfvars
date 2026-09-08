project_id = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
region     = "us-central1"
zone       = "us-central1-c"
generation = "g670-20260903b"

service_account_email  = "adl-494-gpu-smoke-gpu@cs-poc-cha8mmii0xk0iaw5vpf8mxf.iam.gserviceaccount.com"
subnet_name            = "default"
preparation_boot_image = "https://www.googleapis.com/compute/v1/projects/deeplearning-platform-release/global/images/common-cu129-ubuntu-2204-nvidia-580-v20260831"

runtime_bundle_uri    = "gs://adl-issue509-artifacts-cs-host-377d41e71a824f92802120/runtime/issue670/g670-20260903b/runtime-linux-complete.tar.gz#1788473201455657"
runtime_bundle_sha256 = "602a2ec89507e6810b4373ae2001cfa376675f19af0512fac67473b756055d10"
ollama_bundle_uri     = "gs://adl-issue509-artifacts-cs-host-377d41e71a824f92802120/models/ollama/issue509/issue509-relay-a1c440cc7b1e-20260902074759/portable-model-bundle.json#1788335574892889"
ollama_bundle_sha256  = "b8dc0f77b1a5e0e006634f3918517cc65f2506185d5d09b68d984becf422db84"

runtime_disk_size_gib  = 20
ollama_disk_size_gib   = 30
attach_preparation_vms = true
