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

For more information, see `guppyc --help`.
```sh
Usage: guppyc [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input guppy file

Options:
  -e, --entrypoint <ENTRYPOINT>  The function name to use as entrypoint
  -o, --opt <OPT>                Optimisation level [default: 2]
  -v, --verbose...               Increase logging verbosity
  -q, --quiet...                 Decrease logging verbosity
  -h, --help                     Print help
  -V, --version                  Print version

Guppy version:
      --guppy-version <GUPPY_VERSION>  The guppy version to use. Defaults to the latest published version. Incompatible with `guppy_git` and `guppy_ref`
      --guppy-git <GUPPY_GIT>          The git repository to fetch guppy from. Incompatible with `guppy_version`
      --guppy-ref <GUPPY_REF>          The git commit or branch to use. Incompatible with `guppy_version`

Output artifacts:
      --hugr <HUGR>        Optional output path for the HUGR json
      --sexpr <SEXPR>      Optional output path for the S-expression representation of the HUGR
  -m, --mermaid <MERMAID>  Optional output path for the mermaid rendering of the HUGR
  -l, --llvm <LLVM>        Output llvm text file
  -b, --bitcode <BITCODE>  Output the llvm bitcode file
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
