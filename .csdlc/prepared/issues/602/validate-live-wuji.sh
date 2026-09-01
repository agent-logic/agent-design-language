#!/usr/bin/env bash
set -euo pipefail

: "${ADL_ISSUE_602_RUNTIME_INIT:?set ADL_ISSUE_602_RUNTIME_INIT to the deployed Wuji Runtime init file}"
: "${ADL_ISSUE_602_MIGRATION_BUNDLE:?set ADL_ISSUE_602_MIGRATION_BUNDLE to an issue-local artifact path}"

csmctl agent add --init "${ADL_ISSUE_602_RUNTIME_INIT}" --id gemma-e4b --name "Gemma E4B" --role "local assistant" --provider ollama --model gemma4:e4b-mlx --endpoint http://127.0.0.1:11434
csmctl agent migrate --init "${ADL_ISSUE_602_RUNTIME_INIT}" --id gemma-e4b --out "${ADL_ISSUE_602_MIGRATION_BUNDLE}"
csmctl agent rehydrate --init "${ADL_ISSUE_602_RUNTIME_INIT}" --bundle "${ADL_ISSUE_602_MIGRATION_BUNDLE}"
