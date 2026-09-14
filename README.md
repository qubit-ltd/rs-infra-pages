# rs-infra-pages

[![Rust CI](https://github.com/qubit-ltd/rs-infra-pages/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-infra-pages/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-infra-pages/coverage-badge.json)](https://qubit-ltd.github.io/rs-infra-pages/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-infra-pages.svg?color=blue)](https://crates.io/crates/qubit-infra-pages)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Build a configurable project site and hand it to GitHub Pages through the
standard artifact/deploy interface. The binary does not require the legacy
`rs-ci` checkout or Node.js.

## Installation

```bash
cargo install --git https://github.com/qubit-ltd/rs-infra-pages.git --tag v0.1.0 qubit-infra-pages
```

## Quick Start

From a Rust project root:

```bash
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- --help
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- build
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- artifact
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- deploy
```

`build` renders configured README pages into `public`. `artifact` creates a
portable `pages-artifact.tar.gz`, and `deploy` creates the same artifact while
printing the handoff expected by a GitHub Actions `actions/deploy-pages` job.
The tool prepares the artifact; GitHub's Pages action performs authenticated
deployment.

The project's `.infra/ci/pages.json` is the primary configuration.
`.rs-ci-page.json` is also accepted for migration. With no configuration, the
tool builds `README.md` and an optional `README.zh_CN.md`, then copies these
standard reports when present: `assets/`, `target/llvm-cov/html/`,
`coverage-badge.json`, and `ci-summary.json`. Configuration can define
`site_title`, `default_language`, `languages`, `assets`, `coverage`,
`coverage_badge`, and `metadata`; copy entries use
`{ "source": "...", "output": "..." }`.

## Capabilities and limitations

Project-specific policy belongs in `.infra`; orchestration can invoke the
`artifact` or `deploy` command from any CI system.

## Learn More

See the command help and source tests for the supported interface. Switch to [中文文档](README.zh_CN.md).

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-infra-pages](https://github.com/qubit-ltd/rs-infra-pages)
