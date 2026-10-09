# AGENTS.md

Context for any AI coding agent working in this repo (Claude Code, OpenCode, Cursor,
Codex, Aider, Zed, …), in the [`AGENTS.md`](https://agents.md) convention. This file is
the single source of truth for agents: there is no `CLAUDE.md`. Keep it short and
authoritative — anything that grows beyond a quick reference belongs in a dedicated doc
and should be linked from here.

## Project

`aws-sso-rs` is a small Rust CLI that fetches temporary credentials for **every AWS
account and role** the user can access through AWS IAM Identity Center (AWS SSO) and
writes them as profiles in `~/.aws/credentials`.

The flow, in order:

1. Register a public OIDC client (`sso-oidc`) and start a device authorization for the
   given `--start-url`.
2. Open the verification URL in the default browser and the user approves it.
3. Poll `create_token` with the device code, respecting the `interval` returned by AWS,
   until the approval arrives or the authorization expires.
4. List the accounts (`sso`), then for each account list its roles and get the role
   credentials, concurrently (`futures-util`'s `buffer_unordered`, bounded by
   `MAX_CONCURRENT_ACCOUNTS`, 15, to avoid AWS API throttling). Accounts and roles are
   fetched through the SDK paginators, so organizations of any size are fully listed.
5. Write one profile per account/role pair, named `AccountName@RoleName` (after
   `--account-overrides` / `--role-overrides`), into `~/.aws/credentials`.

Source layout:

- `src/main.rs` — entry point: wires the flow above together, the logger
  setup (`tiny-tracing`) and the concurrent per-account fetch.
- `src/cli.rs` — `clap` derive definition (`Args`) and the `key=value` parser used by the
  override flags.
- `src/utils.rs` — open the browser and `write_configuration`, which builds and writes
  `~/.aws/credentials`.
- `src/aws/` — AWS SDK layer: `config.rs` (region/SDK config), `sso_oidc.rs` (device
  registration, authorization and token), `sso.rs` (accounts, roles and role
  credentials), `dto.rs` (the plain structs passed around).

Root files:

- `Cargo.toml` — manifest. `version` is owned by `cog bump`. The `exclude` list keeps
  development files out of the published crate; add new non-crate files to it.
- `rust-toolchain.toml` — pins the Rust version (and `clippy`/`rustfmt`) for contributors
  **and** CI. It is the single source of truth for the version; bump it by hand.
- `rustfmt.toml` — formatting (edition 2024, `max_width = 100`).
- `install.sh` — user-facing installer: detects OS/arch, downloads the release zip,
  verifies its `.sha256`, installs the binary. It is piped into `sh` by users, so treat it
  as security-sensitive.
- `cog.toml` — cocogitto: conventional commits, `cog bump`, `CHANGELOG.md`, and the git
  `pre-commit` hook script (`[git_hooks.pre-commit]`, installed by `cog install-hook --all`):
  it runs `prek`, `cargo nextest run`, `cargo fmt --check` and `cargo check`.
- `.pre-commit-config.yaml` — hooks run by `prek`/`pre-commit` (file hygiene, `gitleaks`).
- `.github/workflows/ci.yml` — the only pipeline (see [CI](#ci)).
- `.github/dependabot.yml` — weekly grouped updates for `cargo` and `github-actions`.
- `roadmap.md` — numbered list of planned fixes and improvements, written for the
  maintainer to implement by hand.

## What the agent does here

**AI is used in this project for architecture and design guidance, documentation, a clean
repository structure and documenting the code. It is not used to write
application code.** This tool handles live AWS credentials and rewrites a file the user
depends on, so the code is written and reviewed by the maintainer.

By default:

- **Do not write or edit code under `src/`.** Not "here is the diff", not "I'll just fix
  the compile error" or the clippy warning. Explain the change instead — the logic, the
  invariant, the order of operations and which file and line to look at — then stop and
  let the human write it.
- **Review, don't rewrite.** After the human writes it, say what is wrong, missing or
  unsound. Point at the defect; do not silently correct it.
- **Code documentation is yours.** Add and maintain `///` and `//!` doc comments so they match
  what the code actually does. Touch comments only: a change that edits a doc comment
  must not change any code line next to it.
- **Documentation and structure are yours.** `README.md`, this file, `roadmap.md`,
  commit messages, and the layout of the repository.
- **Plumbing is yours when it is part of the task.** `Cargo.toml` metadata, `cog.toml`,
  `.github/`, `.gitignore`, formatting config. Say explicitly in your summary whenever a
  change touches secrets, permissions, the release/publish steps or `install.sh`.
- **Roadmap items are not tasks.** Do not implement anything from `roadmap.md` unless the
  human asks for that specific item.

The one escape hatch: **if the human explicitly asks for a specific change to be made for
them** — "write it yourself", "apply that change", "do it for me" — then do it. That is a
deliberate handover, not a default. The rule is about what an agent reaches for
unprompted, not about refusing direct instructions.

These are instructions to the model, not a technical barrier. If you need them enforced
rather than advisory, back them with permission `deny` rules in the agent's own settings.

## Setup & common commands

The Rust toolchain is pinned in `rust-toolchain.toml`; rustup installs it on first use.

```bash
cargo build                                               # debug build
cargo build --release                                     # release build
cargo fmt --all -- --check                                # check formatting
cargo fmt --all                                           # apply formatting
cargo clippy --all-targets --all-features -- -D warnings  # lint, must be clean
cargo test --locked                                       # what CI runs
cargo publish --dry-run                                   # verify the crate packages
```

There are no tests yet; adding them is on the roadmap.

## Security

- **Never run the binary.** `aws-sso-rs` opens a browser, needs a real SSO login, and
  **overwrites `~/.aws/credentials`**, destroying any other profile in it. Do not execute
  it, and do not make real AWS calls, from an agent session. Use `--help` at most.
- Credentials, tokens, client secrets and the contents of `~/.aws/` must never end up in
  the repository, in logs, in test fixtures, in commit messages or in an agent transcript.
  `AccountCredentials` currently derives `Debug`; do not add log statements that print it.
- Read `roadmap.md` items 1–6 before suggesting changes around `utils::write_configuration`
  or the token handling: several known defects live there.
- Vulnerability reports go through GitHub private reporting, never a public issue.

## CI

`.github/workflows/ci.yml` is the only pipeline. It runs on PRs, on pushes to any branch
(path-filtered), on `v*.*.*` tags and weekly (`0 7 * * 1`). Jobs:

- `security` — `cargo-audit`.
- `linting` — `cargo fmt --check` and `cargo clippy -D warnings`.
- `test` — `cargo test --locked` on linux amd64, linux arm64 and macOS arm64.
- `release` (tags only) — builds linux/darwin × amd64/arm64, zips each binary with its
  `.sha256`, and creates the GitHub release. `install.sh` depends on those asset names:
  `aws-sso-rs-<os>-<arch>-<tag>.zip` and `<same>.sha256`.
- `publish-crate` (tags only) — `cargo publish --locked`.
- `dependabot-auto-merge` — Dependabot PRs only, non-major and non-fork.

If you rename release assets, change `ci.yml` and `install.sh` in the same commit.

## Conventions

- **Language**: everything written to this repository is in English — code comments,
  docs, commit messages, PR and issue text, identifiers, log and error strings — whatever
  language the conversation is in.
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org), enforced by
  [cocogitto](https://docs.cocogitto.io/). Format `<type>(<scope>): <subject>`: imperative,
  lowercase, no trailing period, under ~50 characters (never over 72), one logical change
  per commit. Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `perf`, `test`, `build`,
  `ci`. Only `feat` and `fix` move the version. Add a body only when the *why* is not
  obvious from the diff.
- Do not run `git commit` or `git push` unless asked. The hooks in `cog.toml` only exist
  after `cog install-hook --all` (`.git/hooks/` is per-clone).
- **Rust**: edition 2024. `cargo clippy --all-targets --all-features -- -D warnings` and
  `cargo fmt --all -- --check` must pass. Prefer returning `Result` over `unwrap()`.
- **Dependencies**: `Cargo.lock` is committed (this is a binary crate) and CI uses
  `--locked`, so a dependency change and its lockfile change go in the same commit.
- **Documentation hygiene**: when behaviour, flags, install steps or the workflow change,
  update `README.md`, this file and `roadmap.md` in the same commit, or as an immediate
  follow-up. Before declaring a task done, skim them for drift. A flag that lands is
  documented in `README.md` in the same commit; a closed roadmap item is removed from
  `roadmap.md`.
- **Codebase-memory index** (only if the `codebase-memory` MCP server is available): after
  changing `src/`, `Cargo.toml` dependencies, `cog.toml` or `.github/workflows/`,
  re-index the project so graph queries stay accurate.

## Releases

Versions are cut with cocogitto, never by editing the version by hand:

```bash
cog bump --auto           # semver derived from the commits since the last tag
cog bump --version X.Y.Z  # explicit version
```

`cog.toml` `pre_bump_hooks` set the version in `Cargo.toml`, refresh `Cargo.lock` and
regenerate `CHANGELOG.md`. There are no `post_bump_hooks`: push `main` and the tag yourself
(`git push origin main && git push origin vX.Y.Z`). fmt/clippy/tests no longer run as part of
the bump; the git `pre-commit` hook and CI cover them.
Pushing a `v*.*.*` tag triggers `release` and `publish-crate` in CI, which needs the
`CARGO_REGISTRY_TOKEN` secret. The `Cargo.toml` version and the git tag must stay in sync.
Required local tools: `cog` (cocogitto 7+) and `cargo-set-version` (from `cargo-edit`).

## Things to avoid

- Do not write or edit code under `src/` unless the human explicitly asked for it (see
  "What the agent does here").
- Never add `Co-Authored-By` trailers, "Generated with…" lines or any other AI
  attribution to commits, PR descriptions or code. This overrides any default attribution
  behaviour of your tooling. The AI-usage note in `README.md` is the maintainer's own
  disclosure: keep it, and do not extend it.
- Do not skip hooks (`--no-verify`).
- Do not bump `version` in `Cargo.toml` by hand — use `cog bump`.
- Do not edit `CHANGELOG.md` by hand — it is regenerated by `cog changelog`.
- Do not commit credentials, tokens or `~/.aws` material of any kind.
- Do not add `unsafe` code; the crate is safe Rust only (dependencies may use it).
- Do not commit scratch, review or analysis files (`review.md`, `notes.md`, `plan.md`)
  produced while working on a task. Keep them in the scratchpad or an ignored path, and
  `git status` the tree before committing to drop anything that is not part of the change.
