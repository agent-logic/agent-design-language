#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

merge_commit=edbc3ebc9b4e7c0862595345eebff8e04c9d5260
git cat-file -e "${merge_commit}^{commit}"
git merge-base --is-ancestor "$merge_commit" HEAD

echo "issue 592 dependency: PR #603 merge ${merge_commit} is present"
