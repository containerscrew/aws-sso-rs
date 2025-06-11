<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Changelog](#changelog)
  - [[1.0.0] - 2025-06-11](#100---2025-06-11)
  - [[0.4.1] - 2023-11-19](#041---2023-11-19)
  - [[0.4.0] - 2023-11-19](#040---2023-11-19)
  - [[0.3.0] - 2023-11-18](#030---2023-11-18)
  - [[0.2.0] - 2023-11-18](#020---2023-11-18)
  - [[0.1.0] - 2023-11-18](#010---2023-11-18)
<!-- END OF TOC -->
# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-06-11

- Fix global variable in fish function

- Massive refactor

- Refactor(logger): logger format

- Refactor(utils): log info

- Refactor(main): log info

- Add lint pipeline

- Wip: pipelines

- Pipeline cache

- Pre-commit

- Merge pull request #8 from containerscrew/feat/v1.0.0

Massive refactor, new clean version v1.0.0

- Update aws-sdk-sso requirement from 0.39.0 to 1.72.0

---
updated-dependencies:
- dependency-name: aws-sdk-sso
  dependency-version: 1.72.0
  dependency-type: direct:production
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #9 from containerscrew/dependabot/cargo/aws-sdk-sso-1.72.0

Update aws-sdk-sso requirement from 0.39.0 to 1.72.0

- Update webbrowser requirement from 0.8.15 to 1.0.4

Updates the requirements on [webbrowser](https://github.com/amodm/webbrowser-rs) to permit the latest version.
- [Release notes](https://github.com/amodm/webbrowser-rs/releases)
- [Changelog](https://github.com/amodm/webbrowser-rs/blob/main/CHANGELOG.md)
- [Commits](https://github.com/amodm/webbrowser-rs/compare/v0.8.15...v1.0.4)

---
updated-dependencies:
- dependency-name: webbrowser
  dependency-version: 1.0.4
  dependency-type: direct:production
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #11 from containerscrew/dependabot/cargo/webbrowser-1.0.4

Update webbrowser requirement from 0.8.15 to 1.0.4

- Update aws-sdk-ssooidc requirement from 0.39.0 to 1.73.0

---
updated-dependencies:
- dependency-name: aws-sdk-ssooidc
  dependency-version: 1.73.0
  dependency-type: direct:production
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #10 from containerscrew/dependabot/cargo/aws-sdk-ssooidc-1.73.0

Update aws-sdk-ssooidc requirement from 0.39.0 to 1.73.0

- Fix: panic unwrap option

- Add example

- Add example-2

## [0.4.1] - 2023-11-19

- Update README

- Edit help message for --log-level flag

- Update version of Cargo.toml

## [0.4.0] - 2023-11-19

- Update README

- Change default workers&retries & info logging message

## [0.3.0] - 2023-11-18

- Fix cli --version flag, rename pipeline release

- Create new tag v0.3.0

## [0.2.0] - 2023-11-18

- Update README

- Fix install script

- Remove release-please in release pipeline

- Fix error messages

- Generate release notes in release pipeline

- Merge pull request #7 from containerscrew/fix/error-handling

Fix error messages

## [0.1.0] - 2023-11-18

- Initial commit

- Iniital commit, first BETA version v0.1.0

- Update README.md

- Update docs

- Update README.md and pre-commit config

- Create pipeline for Rust

- Add pipelines for test and release

- Fix rust version for release pipeline

- Set rust version + disable release pipeline

- Fix rust version

- Add platform information

- Implement concurrent async threads

- Feature: split main and async_main functions

- Add new console messages & testing release github action pipeline

- Merge pull request #1 from containerscrew/feature/multi-threadding

Multi threadding & refactor some console prints

- Fix release pipeline

- Testing only darwin release

- Refactoring pipeline for automatic release github deploy

- Trigger pipeline release

- Merge pull request #2 from containerscrew/feature/github-release

Feature/GitHub release

- Remove --locked flag in mac osx build

- Testing: add pipeline release when tags

- Implement semaphore to control number of concurrent threads (spawned tasks)

- Implementing config subcommand & refactor code

- Implement start subcommand

- Merge pull request #3 from containerscrew/implementing-config-subcommand

Implementing config&start subcommand

- Refactoring

- Merge pull request #4 from containerscrew/implementing-config-subcommand

Refactoring

- Adding new pipeline for coverage

- Testing coverage pipeline

- Testing rust release pipeline

- Testing

- Fix coverage pipeline

- Updating README & refactor release pipeline

- Testing release pipeline

- Setup release for linux

- Fix pipeline release

- Remove --locked flag when build

- Improve release pipeline & cargo fmt code

- Pre-commit update

- Add target for linux aarch64

- Fix warnings

- Testing with cross

- Wip

- Remove --offline flag when build

- Fix musl-tools

- Release pipeline step

- Package zip && upgrade cargo dependencies version

- Packaging binaries

- Merging to master

- Merge pull request #5 from containerscrew/feature/new-pipelines

Refactoring pipelines

- Update README

- Doctoc

- Add release-please

- Coverage pipeline only for main branch

- Modify codecoverage pipeline

- Create installation script + fix release pipeline

- Update install.sh script

- Pre-commit

- Update documentation, code warnings and logging

- Merge pull request #6 from containerscrew/latest_refactors

Latest refactors

<!-- generated by git-cliff -->
