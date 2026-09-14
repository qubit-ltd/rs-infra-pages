# rs-infra-pages

[![Rust CI](https://github.com/qubit-ltd/rs-infra-pages/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-infra-pages/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-infra-pages/coverage-badge.json)](https://qubit-ltd.github.io/rs-infra-pages/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-infra-pages.svg?color=blue)](https://crates.io/crates/qubit-infra-pages)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

构建配置驱动的项目站点，并通过标准 artifact/deploy 接口交给 GitHub
Pages。工具自身不依赖旧版 `rs-ci` 检出目录或 Node.js。

## 安装

```bash
cargo install --git https://github.com/qubit-ltd/rs-infra-pages.git --tag v0.1.0 qubit-infra-pages
```

## 快速开始

在 Rust 项目根目录构建、打包或准备部署：

```bash
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- --help
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- build
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- artifact
cargo run --manifest-path /path/to/rs-infra-pages/Cargo.toml -- deploy
```

`build` 将配置的 README 渲染到 `public`；`artifact` 生成可移植的
`pages-artifact.tar.gz`；`deploy` 生成相同 artifact 并输出供 GitHub Actions
`actions/deploy-pages` 使用的交接提示。认证部署仍由 GitHub Pages action
完成。

主要配置文件是 `.infra/ci/pages.json`，迁移时也接受旧名称
`.rs-ci-page.json`。没有配置时会构建 `README.md` 和存在时的
`README.zh_CN.md`，并复制存在的 `assets/`、`target/llvm-cov/html/`、
`coverage-badge.json`、`ci-summary.json`。配置支持 `site_title`、
`default_language`、`languages`、`assets`、`coverage`、`coverage_badge` 和
`metadata`；复制项格式为 `{ "source": "...", "output": "..." }`。

## 能力与限制

项目策略放在 `.infra`；任意 CI 系统都可以调用 `artifact` 或 `deploy`。

## 延伸阅读

可通过命令帮助和源码测试了解实际接口。切换到 [English README](README.md)。

## 测试

```bash
cargo test
cargo test --all-features
./ci-check.sh
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交 Pull Request 前运行 `./align-ci.sh` 格式化代码，运行 `./ci-check.sh` 满足 CI 要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-infra-pages](https://github.com/qubit-ltd/rs-infra-pages)
