#!/usr/bin/env bash
set -euo pipefail

: "${ADL_ISSUE_602_RUNTIME_INIT:?set ADL_ISSUE_602_RUNTIME_INIT to the deployed Wuji Runtime init file}"
: "${ADL_ISSUE_602_MIGRATION_BUNDLE:?set ADL_ISSUE_602_MIGRATION_BUNDLE to an issue-local artifact path}"
: "${ADL_ISSUE_602_AGENT_CONFIG:?set ADL_ISSUE_602_AGENT_CONFIG to the local Wuji agent config}"

csmctl agent add --config "${ADL_ISSUE_602_AGENT_CONFIG}"
csmctl agent migrate --init "${ADL_ISSUE_602_RUNTIME_INIT}" --id ember-axioma --out "${ADL_ISSUE_602_MIGRATION_BUNDLE}"
csmctl agent rehydrate --init "${ADL_ISSUE_602_RUNTIME_INIT}" --bundle "${ADL_ISSUE_602_MIGRATION_BUNDLE}"
