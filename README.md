guppyc
=========

[![build_status][]](https://github.com/CQCL/guppyc/actions)
[![crates][]](https://crates.io/crates/guppyc)
[![msrv][]](https://github.com/CQCL/guppyc)

Compiler tools for the [guppylang](https://github.com/cqcl/guppylang) Quantum Programming Language.

Please read the [API documentation here][].

## Usage

`guppyc` can be installed via `cargo`:

```sh
# Install latest version
cargo install guppyc
# Or install the local project
cargo install --path .
```

Then you can use it to compile a `.gpy`/`.py` file:

```sh
guppyc \
  test_files/even_odd.py  `# The input file` \
  --hugr even_odd.hugr    `# Output hugr JSON file` \
  --mermaid even_odd.mmd  `# Output hugr Mermaid file` \
  --llvm even_odd.ll      `# Output LLVM IR text file` \
  --bitcode even_odd.o    `# Output LLVM bitcode file` \
  --guppy-version 0.15.0  `# Fix the version of the guppylang compiler` \
```

For more information, run `guppyc --help`.

## Recent Changes

See [CHANGELOG][] for a list of changes. The minimum supported rust
version will only change on major releases.

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for instructions on setting up the development environment.

## License

This project is licensed under Apache License, Version 2.0 ([LICENSE][] or http://www.apache.org/licenses/LICENSE-2.0).

  [API documentation here]: https://docs.rs/guppyc/
  [build_status]: https://github.com/CQCL/guppyc/actions/workflows/ci.yml/badge.svg
  [crates]: https://img.shields.io/crates/v/guppyc
  [LICENSE]: LICENCE
  [msrv]: https://img.shields.io/badge/rust-1.75.0%2B-blue.svg?maxAge=3600
  [CHANGELOG]: CHANGELOG.md