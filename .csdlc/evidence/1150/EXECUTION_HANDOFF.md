# CF-05 actual qualification execution handoff

## Current gate

Actual qualification is **NO-GO** until CodeFriend INTEGRATE #45 publishes and
accepts one exact installed lockset. Four producer inputs are accepted; #45 is
still open and its green draft PR #91 identifies itself as
`proposed_unaccepted`, with `acceptance_authority: false`, native status
`not_admitted`, and hosted/local installed journeys pending.

Running the product before that lockset is accepted would create non-credit
evidence against a moving candidate. No provider-free or paid scenario should
start merely to create activity.

## Accepted producer inputs

| Input | Accepted identity | Qualification boundary |
|---|---|---|
| CF-04 #34 | CodeFriend PR #86, head `559ed261a9f132c9f6e52db45c2c8ab50bf6fe83`, merge `dc4b5c3c39563925cc08d9100d1862892a4a4aad`, Product CI `37167802049` | Provider-free recovery mechanics; no claim of external provider effect or cost reversal |
| CT-05 #44 | CodeFriend PR #89, head `33c0e2d8e3a65d993dc22ecb72f9d6652b49b521`, merge `b200caf8668e707ac10b2b47ba7c88441d7af593`; qualified source `29bbec94aab82d856f6a42d2b1af517e0c2f8f9d`; product artifact `11299261894`; 44/44 proof `94ba4c72fc49e1e8a7df774aa6e2e889ff5037b396ce36c0fdd9622c22967159` | Installed catalog, 44/44; zero provider/deployment credit |
| ADL #1148 | CodeFriend PR #55, head `a07b8cf1dc040412b5c1099ab10c1bd7230cb667`, merge `12134f79cb1c43c4f9031c1410bb45c88608c39a`, Product CI `37080324235` | Accepted citation-grounding producer fix |
| ADL #1149 | CodeFriend PR #54, head `adef56b9accd946a531611b41364d23fdcb71e37`, merge `35cc6e74fec2ee56a71c04ec31110ea4acccf45d`, Product CI `37080322952` | Accepted safe-recovery fix; historical provider effect and spend remain unknown |

The final website source is available for #45 to consume: CodeFriend.ai PR #26,
head `8f03c97365bf0554382dbffefb064697a734181b`, merge
`2b983fe56f0ac055c029af5d68d920db74ca3f81`, tree
`085e6961769e7af395abcd06c430b624a410efc7`, archive SHA-256
`6be8d6c1b507a18a117b9142d1642249f868050956fc81a53e070a73310fe191`.
It includes the waiting-list source and proves deployment readiness only; it is
not deployed and does not provide installed journey credit.

## Start condition and freeze packet

When #45 is accepted, freeze a new private candidate packet containing:

1. the merged #45 source, lockset bytes, acceptance receipt, and exact installed
   CodeFriend, Runtime, template, website, ADL, and agent identities;
2. the final website PR #26 identities above;
3. macOS and Linux OS/interpreter/translation identities;
4. the pinned ADL six-file and Vector ten-file source scopes, with source hashes
   before execution;
5. create-only evidence destinations and one independent operator identity;
6. the source-exact Q01-Q24 map from retained #915 handoff SHA-256
   `70df5cb0c723367ceb2dee306f455bc0c6515d6276588931cd78e2d2fe418956`.

If any installed identity differs from the accepted #45 packet, stop and refresh
the candidate rather than translating old proof to new bytes.

## Provider-free phase

Execute this phase across all 12 cells after the lockset freeze. These runs can
prove setup, ingestion, deterministic analysis, fitness, security/refusal,
baseline-negative, approval/export, source-immutability, provenance, and
reproduction portions of Q01-Q08 and Q13-Q23. They cannot complete Q09, the
provider-dependent portions of Q03/Q10-Q12/Q14/Q15/Q22, or any cell's review
stage.

Use the retained owner harness from #915. Allocate a new private evidence root,
require the manifest path not to exist, and enable shell no-clobber before
creating it. Never truncate or edit an old result into a pass:

```text
test ! -e <private-root>/manifest.prepared.json
set -o noclobber
python3 adl/tools/qualify_codefriend_beta1.py template > <private-root>/manifest.prepared.json
```

For each CLI cell, write this exact argv array to its new `argv.json`:

```text
["codefriend", "journey", "resume", "--output", "<private-root>/<cell>/journey", "--request", "<private-root>/<cell>/request.json"]
```

Then invoke `capture` once. The capture owner performs the binary/candidate
checks and executes the command; do not execute the argv separately:

```text
python3 adl/tools/qualify_codefriend_beta1.py capture \
  --binary <installed-adl> \
  --binary-sha256 <accepted-adl-sha256> \
  --candidate-file <private-root>/candidate.json \
  --argv-file <private-root>/<cell>/argv.json \
  --output <private-root>/<cell>/capture
```

Capture the installed local agent for each local-agent website cell:

```text
python3 adl/tools/qualify_codefriend_beta1.py capture \
  --entrypoint agent \
  --binary <installed-codefriend-agent> \
  --binary-sha256 <accepted-agent-sha256> \
  --candidate-file <private-root>/candidate.json \
  --argv-file <private-root>/<cell>/agent-argv.json \
  --output <private-root>/<cell>/agent-capture
```

The agent argv is exactly `once --store <private-store> --consent
<reviewed-consent.json>`. Hosted and paired-local browser cells require fresh
invited and uninvited users, cross-user isolation, disconnect/reconnect,
cancellation, and retained terminal-operation identity. Public deployment and a
live audience are not required; use the installed/local or owned nonpublic
candidate from #45.

For every cell, retain all eight stages (`setup`, `ingestion`, `analysis`,
`review`, `synthesis`, `plans`, `comparison`, `approved_exports`) and the
applicable negative scenarios. After the provider-free pass, run the offline
audit only as a consistency check. Its expected result is `incomplete` because
provider and human observations are not yet complete:

```text
python3 adl/tools/qualify_codefriend_beta1.py audit \
  --manifest <private-root>/manifest.json \
  --evidence-root <private-root> \
  --offline
```

## Provider-backed phase requiring separate approval

Q09 requires four isolated review perspectives to call the registered provider
and retain exact provider, model, profile, candidate, usage, and result
identities. The auditor requires at least four calls and four generated results
for each cell's review scenario. With 12 cells, the first-review minimum is
therefore **48 provider calls**. This is not the full paid denominator: compatible
repeat/comparison work and provider failure/cancellation scenarios add calls.

The retained #915 proposal used a conservative reservation of USD 0.425 per
call, USD 1.70 per four-lane group, and one durable USD 10 ledger. At that bound,
48 first-review calls alone reserve USD 20.40, so the historical USD 10 cap is
insufficient. Before any provider POST:

1. select the exact registered model/profile and verify current pricing;
2. set per-call input/output token ceilings and a total durable ledger cap;
3. authorize the first 12 four-lane groups or explicitly reduce scope while
   preserving the unresolved denominator;
4. reconcile actual usage after each group; unknown outcomes retain their full
   reservation and forbid replay;
5. separately authorize compatible repeat, controlled-change comparison, and
   any real provider failure/cancellation observation after first-pass usage is
   known.

No paid call is authorized by this handoff.

## Human inspection and final decision

An independent human must inspect invited/uninvited browser behavior, user and
agent isolation, HTML navigation, rendered PDF pages, and Markdown/HTML/PDF
semantic parity. Human approval must bind exact current artifacts, destination,
claims, and versions; stale, withheld, or tampered approval must be rejected.

Only after every scenario is recorded should the owner run the online audit and
independent review:

```text
python3 adl/tools/qualify_codefriend_beta1.py audit \
  --manifest <private-root>/manifest.json \
  --evidence-root <private-root>
```

Until all 12 cells and 24 obligations resolve from exact-candidate evidence,
accepted qualification remains 0/12 and the issue remains open.
