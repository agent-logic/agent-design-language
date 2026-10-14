# Retained Sprint owner contracts

The six remaining ADL Sprint helper implementations now live in the reviewed
C-SDLC owner package alongside the previously transferred conductor sources.
`owner_inputs.json` pins all 43 resources and the 20-script composition. Five
of these six helper bodies are unchanged. Parity now requires explicit tracked
and installed roots and refuses canonical lifecycle state writes; its old
implicit-root/write behavior is retired, not claimed equivalent.

Select an authentic complete installed package with the absolute
`CSDLC_CONDUCTOR_PACKAGE_ROOT` path. The public selector routes changes here
through `adl/tools/test_install_adl_operational_skills.sh`, which also requires
an absolute `ADL_OPERATIONAL_SKILLS_SOURCE_ROOT` selecting the complete original
installer resource layout. It retains its copy/symlink/content assertions and
invokes the mapped conductor adapter exactly once. The readiness-metrics test
retains its separate explicitly pinned original source-root contract.

Existing reports/metrics/parity and closeout component proofs are reused for
unchanged inputs. The original 421 assertion-bearing expressions are not a
count of independent tests or a full original-suite pass. Component fixtures
use native transport doubles; native admission, live lifecycle operations and
complete installed-bundle acceptance remain separate. No helper should be
copied back into ADL to bypass package or source-root requirements.
