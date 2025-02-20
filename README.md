guppyc
=========

[![build_status][]](https://github.com/CQCL/guppyc/actions)
[![crates][]](https://crates.io/crates/guppyc)
[![msrv][]](https://github.com/CQCL/guppyc)

Compiler tools for the [guppylang](https://github.com/cqcl/guppylang) Quantum Programming Language.

Please read the [API documentation here][].

## Installation

`guppyc` can be installed via `cargo`:

```sh
# Install latest version
cargo install --git https://github.com/CQCL/guppyc.git
# Or install the local project
cargo install --path .
```

You will also need the following tools:
- uv `>=0.6`: [docs.astral.sh](https://docs.astral.sh/uv/getting-started/installation/)
- LLVM 14: [llvm.org](https://llvm.org/docs/GettingStarted.html)

## Usage

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

For more information, see `guppyc --help`.
```sh
Usage: guppyc [OPTIONS] <input|--hugr-input <HUGR_INPUT>>

Options:
  -e, --entrypoint <ENTRYPOINT>  Function name to use as entrypoint
  -o, --opt <OPT>                Optimisation level [default: 2] [possible values: 0, 1, 2, 3]
  -v, --verbose...               Increase logging verbosity
  -q, --quiet...                 Decrease logging verbosity
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version

Input format:
      --hugr-input <HUGR_INPUT>  A `.hugr` file
  [input]                    A guppy program definition

Output artifacts:
      --hugr <HUGR>        Store the intermediate HUGR as json
      --sexpr <SEXPR>      Store the intermediate HUGR as an S-expression
  -m, --mermaid <MERMAID>  Store the mermaid diagram for the HUGR
  -l, --llvm <LLVM>        LLVM IR (text) output
  -b, --bitcode <BITCODE>  LLVM Bitcode output

Guppy version:
      --guppy-version <GUPPY_VERSION>  The guppy version to use
      --guppy-git <GUPPY_GIT>          The git repository to fetch guppy from
      --guppy-ref <GUPPY_REF>          The git commit or branch to use
```

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
