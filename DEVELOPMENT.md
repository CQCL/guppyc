# Welcome to the guppyc development guide <!-- omit in toc -->

This guide is intended to help you get started with developing guppyc.

If you find any errors or omissions in this document, please [open an issue](https://github.com/CQCL/guppyc/issues/new)!

## #️⃣ Setting up the development environment

You can setup the development environment you will need:

- Rust `>=1.85`: [rust-lang.org](https://www.rust-lang.org/tools/install)
- uv `>=0.6`: [docs.astral.sh](https://docs.astral.sh/uv/getting-started/installation/)
    * If you have an older manually installed `uv` version you can upgrade it
      with `uv self update`, or by following the instructions for your package
      manager.

## 🏃 Running the tests

To compile and test the rust code, run:

```bash
cargo build
cargo test
```

## 💅 Coding Style

The rustfmt tool is used to enforce a consistent rust coding style. The CI will fail if the code is not formatted correctly.

To format your code, run:

```bash
# Format rust code
cargo fmt
```

We also check for clippy warnings, which are a set of linting rules for rust. To run clippy, run:

```bash
cargo clippy --all-targets
```

## 🌐 Contributing to guppyc

We welcome contributions to guppyc! Please open [an issue](https://github.com/CQCL/guppyc/issues/new) or [pull request](https://github.com/CQCL/guppyc/compare) if you have any questions or suggestions.

PRs should be made against the `main` branch, and should pass all CI checks before being merged. This includes using the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) format for the PR title.

The general format of a contribution title should be:

```
<type>(<scope>)!: <description>
```

Where the scope is optional, and the `!` is only included if this is a semver breaking change that requires a major version bump.

We accept the following contribution types:

- feat: New features.
- fix: Bug fixes.
- docs: Improvements to the documentation.
- style: Formatting, missing semi colons, etc; no code change.
- refactor: Refactoring code without changing behaviour.
- perf: Code refactoring focused on improving performance.
- test: Adding missing tests, refactoring tests; no production code change.
- ci: CI related changes. These changes are not published in the changelog.
- chore: Updating build tasks, package manager configs, etc. These changes are not published in the changelog.
- revert: Reverting previous commits.