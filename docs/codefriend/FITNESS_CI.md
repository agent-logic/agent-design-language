# CodeFriend fitness CI adapter

`adl/tools/codefriend_fitness_ci.sh --binary <installed-adl> ...` invokes the
installed `adl codefriend fitness ci-run` command. The adapter starts that same
installed binary's existing `fitness run` command, then verifies its retained
report against independently supplied expectations. The shell also checks that fresh, bounded receipts/reports agree with the
process outcome and independently declared pins; an unrelated executable returning
zero without artifacts cannot pass. It defines no new fitness
predicates and cannot turn a violation into a pass.

The shell entrypoint requires Bash and Python 3 for transport postconditions.
All fitness policy evaluation and live evidence validation remain in the installed
Rust product.

Required arguments after `--binary`:

- `--store <admitted-store>` and `--packet-id <expected-packet>`
- `--policy <policy.json>` and `--policy-digest <expected-canonical-policy-digest>`
- `--candidate <exact-git-revision>`
- `--out <new-artifact-directory>`

The expected policy digest is the shared CodeFriend hash of the typed `Policy`,
not a hash copied from the output or a raw-file SHA-256. The committed fixture
manifest records reviewed policy pins. Updating a policy requires updating its
independently reviewed pin; a changed policy cannot silently authenticate itself
through its result artifact.

## Exit and artifact contract

A valid report preserves exit **0 = pass**, **1 = violation**, **2 = assessed
error**. Unsupported or signal exits, absent/truncated/tampered reports, expired
or deleted admission, external identity mismatch, and disagreement between the
original process exit and validated assessment all yield adapter exit 2.

Every invocation first retains validated declared pins in `expectations.json`,
including when the runner subsequently produces no report. These pins are declared
expectations, not a claim that missing output was verified. Every started runner retains `runner-exit.txt`, `runner.stdout.json` and
`runner.stderr.log`. A successfully verified artifact retains `report.json` and
`receipt.json`. A runner command error may produce no report: the rejection
receipt preserves its original exit while setting `artifact_valid=false`, without
inventing an assessment. Failure to start a runner or create artifacts is a
command error and cannot supply a successful receipt. Output directories and
receipts are create-only; existing results never satisfy a new invocation.
Managed admission and output roots must be separate. Symlink and parent path
traversal checks apply before output creation. Receipts contain bounded identities
and static errors; underlying parsing errors and host paths are not exported.
The directory is private on Unix; only selected bounded result files are uploaded.

`adl codefriend fitness ci-verify --store ... --input <report.json> --candidate ...
--packet-id ... --policy-digest ... --runner-exit <integer> --out <new-receipt.json>`
exposes the same verification for a retained result. It still consults live
admission and recomputes the local report; an embedded policy or packet is not
independent CI authority. `fitness ci-run` creates and invokes this contract;
`ci-verify` does not execute the runner or accept an old receipt as proof.

## Automatic CI and evidence

`.github/workflows/codefriend-fitness.yml` runs automatically for relevant PRs and
main pushes. It uses read-only repository permissions, no secrets, no provider
calls, and no analyzed-source scripts. One job builds and installs the product;
consumer jobs reuse that installed binary with its installer provenance.

The **fitness candidate policy gate** admits the actual checkout revision and
checks the declared `.github/codefriend-fitness-policy.json`. Its initial invariant
forbids a direct `reqwest` import in the public governance facade
(`adl/src/codefriend/governance/mod.rs`). This declared file fits the runner’s
bounded grammar. The broader `local.rs` implementation remains unassessed by
this policy; this gate does not establish whole-runner HTTP independence.
It is a narrow literal-import boundary, not a claim of complete dependency or
architecture quality analysis. The candidate gate never suppresses a violation
or error. Artifact upload runs after failure and cannot change the gate outcome.

Separate **fitness fixture qualification (pass/fail/error)** jobs use fixed inert
Git revisions, the same declared fixture policy and installed adapter, and retain
the original failing gate step outcome. Their assertions can pass when they
observe the expected violation/error. That means the fixture contract was proved;
it does **not** mean the assessed policy passed. The production adapter has no
expected-failure option.

For each job, retain `context.json`, `proof.json`, the local report, the adapter
report and receipt, and original process exit. The proof checks exact local/adapter
report equality for the same admitted packet, policy and candidate. A normalized
semantic result supports cross-platform fixture comparison without mistaking
retention timestamps or platform-specific binary hashes for semantic differences.
Actual hosted job conclusions and downloaded artifact inspection are required
before claiming this issue's CI integration complete. Local tests and authored
workflow YAML alone do not establish that claim.

## Validation scope

`codefriend_cf_gov_ci` exercises real subprocess pass/fail/error parity, independent
identity pins, original-exit mismatch, malformed and missing artifacts, absent
runner reports, existing output rejection, managed-root and argument errors.
`adl/tests/fixtures/codefriend/fitness-ci/PVF.json` classifies the required proof.
Run the fixture preparation and proof tools only as the reviewed qualification
harness; they are not an alternative policy engine or permission to run scripts
from an analyzed repository. Full external-product qualification remains CF-PROOF.
