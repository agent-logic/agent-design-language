# GCP-B1 corrective design

Replace the static-key bootstrap path with short-lived impersonation of the
existing Terraform bootstrap service account. Constrain Terraform to the
accepted company project, `us-west2`, and one private remote-state bucket. A
reviewed saved plan precedes apply. Live proof covers bucket controls, an
immutable-generation recovery canary, clean-directory backend reinitialization,
and absence of local state or credential residue. Roll back an empty,
unadopted bucket; never delete an adopted state bucket automatically.
