# #498 CORP-D corporate diligence acceptance design

## Intent

#498 produces exactly one corporate diligence acceptance decision for the v0.92.1
corporate acceptance sprint. The decision binds a repository-local diligence
index to the live dispositions from CORP-A (#482), CORP-B (#483), and CORP-C
(#497), while keeping private legal advice and counsel-controlled judgments out
of Git.

## Boundary

The issue owns only diligence acceptance:

- `docs/operations/corporate/diligence/**`
- `docs/milestones/v0.92.1/evidence/corporate/corp-d/**`
- `.csdlc/prepared/issues/498/**`
- `.csdlc/evidence/498/**`

It consumes CORP-A/B/C as prerequisite inputs. It must not repair CORP-C,
replace counsel, infer legal conclusions, expose private diligence material, or
mutate provider, account, billing, credential, DNS, certificate, CI, Terraform,
or deployment state.

## Execution gate

CORP-D cannot accept diligence until CORP-C is live merged into `main` and the
merge commit is ancestral to the #498 execution base. A draft, pending, or
non-closing CORP-C PR is not enough. The prerequisite census validator must
record CORP-C as unresolved until that condition is true.

Typed finish and worktree cleanup receipts for prerequisites are audit-only;
they do not substitute for live merged/ancestral GitHub truth.

## Deliverable shape

The executor should produce:

1. A diligence index listing each prerequisite blocker/disposition source.
2. A prerequisite census binding #482, #483, and #497 to live issue/PR/merge
   state.
3. Counsel-boundary receipts that identify only public or redacted receipt
   references, never advice content.
4. A corporate diligence acceptance record that either accepts the complete
   denominator or fails closed with unresolved blockers.
5. Validation evidence proving private material is absent and each acceptance
   criterion maps to retained evidence.

## Validation approach

Preparation validation is offline and deterministic. Execution validation
includes live GitHub readbacks for prerequisite state and local redaction checks
over the issue-owned diligence evidence.
