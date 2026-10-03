# Roadmap

Planned improvements for `aws-sso-rs`, roughly ordered by priority.

1. **Do not overwrite `~/.aws/credentials`.** Read the existing file, merge the SSO profiles into it and keep every other profile (e.g. `default`). Create a backup before writing, and write the file once instead of once per account.
2. **Secure file permissions.** Create `~/.aws/credentials` with `0600` when it does not exist.
3. **Fix `--role-overrides` semantics.** A non-empty override currently replaces the whole profile name, so several accounts with the same role collide. It should produce `AccountName@new-role`, as documented.
4. **Pagination.** Follow `next_token` in `list_accounts` and `list_account_roles` so organizations with many accounts or roles are fully fetched.
5. **Remove panics.** Replace the `unwrap()` calls on SDK `Option` fields with proper error handling and a clear exit code.
6. **Honest error reporting.** Print a summary of the accounts that failed and exit non-zero when any of them did. Drop the misleading "Retrying..." message (retries are done by the SDK).
7. **Do not leak secrets in logs.** Implement `Debug` for `AccountCredentials` manually, redacting keys and tokens.
8. **Filtering options.** Add `--account`/`--role` filters (names or ids) to fetch only a subset of accounts.
9. **Tests.** Cover profile name resolution and `key=value` parsing, and use `mockall` (already a dev-dependency) for the AWS calls.
10. **Refactor.** Share the retry/client builder between `sso` and `sso_oidc`, take `&str` instead of `&String`, and drop `async` where nothing is awaited.
11. **Release hardening.** Publish SHA256 checksums for every release asset (done for the zips, extend to a single `checksums.txt`) and consider signing artifacts.
12. **Declare a minimum supported Rust version.** Add `rust-version` back to `Cargo.toml` once the lowest compatible version is known (e.g. with `cargo msrv`).
13. **Windows support.** Build and test a `x86_64-pc-windows-msvc` target.
