project_id = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
region     = "us-central1"
zone       = "us-central1-c"
generation = "g670-20260903b"

runtime_staging_disk    = "projects/cs-poc-cha8mmii0xk0iaw5vpf8mxf/zones/us-central1-c/disks/adl-663-g670-20260903b-runtime-staging"
ollama_staging_disk     = "projects/cs-poc-cha8mmii0xk0iaw5vpf8mxf/zones/us-central1-c/disks/adl-663-g670-20260903b-ollama-staging"
runtime_manifest_sha256 = "fce41b9879159e7a1f461353045ec6353fba2b32dcb59261e90f97de6461b1b0"
ollama_manifest_sha256  = "c319b14eb88b9ac529f39adae73998279baed92f64a6f353f0e4eedb64af171d"

verification_boot_image = "https://www.googleapis.com/compute/v1/projects/deeplearning-platform-release/global/images/common-cu129-ubuntu-2204-nvidia-580-v20260831"
service_account_email   = "adl-494-gpu-smoke-gpu@cs-poc-cha8mmii0xk0iaw5vpf8mxf.iam.gserviceaccount.com"
subnet_name             = "default"
enable_verifier         = true
