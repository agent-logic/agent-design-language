# GCP warm two-node Polis

This directory provides three deliberately separate Terraform states:

- `preparation/` owns temporary hydration VMs and staging disks.
- `snapshot-catalog/` owns durable Runtime and Ollama/model snapshots. Its temporary verifier VM and restored verification disks are removed before the generation is retained.
- this directory is the launch state. It owns only the Runtime/Guardian VM, GPU/Ollama VM, firewall rule, and disposable disks restored from exact snapshots.

Normal launch uses immutable boot-image IDs and exact snapshot self-links. It does not install packages, access Git, build Runtime binaries, pull models, or copy model data. Both launch disks are deleted by ordinary `terraform destroy`; the snapshots cannot be deleted because they live in the separate catalog state.

## Prepare one generation

Use separate backend configuration and variable files for the preparation and snapshot-catalog roots. The operator script hydrates both disks concurrently, waits for both preparation VMs to seal and stop, removes the VM attachments, creates both snapshots concurrently, verifies restored content, removes verifier resources, and destroys the staging state:

```sh
ADL_GCP_LIVE_EXECUTION=authorized \
  ./prepare-snapshot-generation.sh preparation.tfvars snapshot-catalog.tfvars
```

The retained result is the two catalog snapshots only.

## Launch and clean up

Initialize this directory with its own backend, supply exact image IDs, snapshot self-links, manifest digests, and generation, then run:

```sh
ADL_GCP_LIVE_EXECUTION=authorized ./run-live-snapshot-launch.sh launch
./run-live-snapshot-launch.sh destroy
```

The launch receipt separates Terraform apply time from full snapshot-launch-to-ready time. An optional observation timeout stops only the caller and explicitly leaves services running; there is no runtime termination deadline.

## Retire snapshots

Snapshot deletion is intentionally separate. Read the exact generation and snapshot IDs from catalog outputs, then pass all three explicitly:

```sh
./retire-snapshot-generation.sh GENERATION RUNTIME_SNAPSHOT_ID OLLAMA_SNAPSHOT_ID
```

The command fails closed on any mismatch or if verifier resources remain. It never runs as part of ordinary launch cleanup.
