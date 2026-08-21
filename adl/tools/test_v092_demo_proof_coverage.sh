#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN_ROOT="${REPO_ROOT}/.csdlc/evidence/308/validator-negative-fixtures"

cd "${REPO_ROOT}"

python3 adl/tools/validate_v092_demo_proof_coverage.py --root .

rm -rf "${RUN_ROOT}"
trap 'rm -rf "${RUN_ROOT}"' EXIT
mkdir -p \
  "${RUN_ROOT}/docs/milestones/v0.92/review" \
  "${RUN_ROOT}/adl/tools"
cp docs/milestones/v0.92/DEMO_MATRIX_v0.92.md \
  "${RUN_ROOT}/docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"
cp docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md \
  "${RUN_ROOT}/docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
cp docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md \
  "${RUN_ROOT}/docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"
cp docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md \
  "${RUN_ROOT}/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md"
cp adl/tools/validate_v092_demo_proof_coverage.py \
  "${RUN_ROOT}/adl/tools/validate_v092_demo_proof_coverage.py"
cp adl/tools/test_v092_demo_proof_coverage.sh \
  "${RUN_ROOT}/adl/tools/test_v092_demo_proof_coverage.sh"

python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"

rm "${RUN_ROOT}/docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected missing required path fixture to fail" >&2
  exit 1
fi

cp docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md \
  "${RUN_ROOT}/docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"

python3 - "$RUN_ROOT/docs/milestones/v0.92/DEMO_MATRIX_v0.92.md" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text()
text = text.replace("| D1 | First birthday proof | A named identity can cross the birth boundary with required evidence. | Birthday record, witness set, receipt, and review packet. | blocked_with_evidence | AEE-014 |",
                    "| D1 | First birthday proof | A named identity can cross the birth boundary with required evidence. | Birthday record, witness set, receipt, and review packet. | accepted | AEE-014 |")
path.write_text(text)
PY

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected planned-as-passed fixture to fail" >&2
  exit 1
fi

cp docs/milestones/v0.92/DEMO_MATRIX_v0.92.md "${RUN_ROOT}/docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"
python3 - "$RUN_ROOT/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text()
text = text.replace("| AEE-019 | WP-21, WP-21A | Reduction and refactoring | planned | pending-#309 | pending-#309 | pending-#309 | pending-#309-review | pending-#309-command | Blocked until #308 is terminal; not accepted release evidence. |",
                    "| AEE-019 | WP-20 | Demo matrix and proof coverage | accepted | current-issue-head | docs/milestones/v0.92/DEMO_MATRIX_v0.92.md | adl/tools/test_v092_demo_proof_coverage.sh | pre-pr-review-required | python3 adl/tools/validate_v092_demo_proof_coverage.py --root . | Duplicate accepted owner/surface fixture. |")
path.write_text(text)
PY

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected duplicate accepted owner/surface fixture to fail" >&2
  exit 1
fi

cp docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md "${RUN_ROOT}/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md"
python3 - "$RUN_ROOT/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text()
text = text.replace("adl/tools/test_v092_demo_proof_coverage.sh | pre-pr-review-required",
                    "missing-negative-proof.log | pre-pr-review-required")
path.write_text(text)
PY

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected missing negative artifact fixture to fail" >&2
  exit 1
fi

cp docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md "${RUN_ROOT}/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md"
python3 - "$RUN_ROOT/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text()
text = text.replace("Accepted only for WP-20 validator/matrix reconciliation, not for product demos.",
                    "Unsupported platform claims accepted for product demos.")
path.write_text(text)
PY

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected unsupported platform claim fixture to fail" >&2
  exit 1
fi

cp docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md "${RUN_ROOT}/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md"
python3 - "$RUN_ROOT/docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text()
text = text.replace("Accepted only for WP-20 validator/matrix reconciliation, not for product demos.",
                    "Synthetic proof accepted for product demos.")
path.write_text(text)
PY

if python3 adl/tools/validate_v092_demo_proof_coverage.py --root "${RUN_ROOT}"; then
  echo "expected synthetic proof fixture to fail" >&2
  exit 1
fi

echo "v0.92 demo proof coverage negative tests: PASS"
