#!/usr/bin/env bash
# Driver for the cocogitto `pre-commit` git hook (see [git_hooks] in cog.toml).
# Installed per clone with `cog install-hook --all` — .git/hooks is untracked,
# so until that runs, only CI catches problems.
set -euo pipefail

separator() {
    echo -e "\n--- $1 ---"
}

separator "Running pre-commit hooks"
pre-commit run -a --show-diff-on-failure

# Uncomment once gitleaks is part of the workflow (brew install gitleaks).
# separator "Scanning for secrets"
# gitleaks git -v

separator "Running cargo fmt"
cargo fmt --all

separator "Running cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

separator "Running tests with nextest"
cargo nextest run --all-features --no-tests=warn
