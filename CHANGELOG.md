# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1-rc.12](https://github.com/inference-gateway/coder/compare/0.1.1-rc.11...0.1.1-rc.12) (2025-02-13)

### 👷 CI

* Ok this setup works if I try to cross compile from macos arm64 to x86 so it supposed to also work the other way from the runner ([c7d05cc](https://github.com/inference-gateway/coder/commit/c7d05cc6f2313f2a6e327fe68e699cd657da43c0))
* Update cross-compilation setup for musl targets and improve build tools installation ([2ea56e4](https://github.com/inference-gateway/coder/commit/2ea56e45db9e06713049d10d7d4f00d9d4cd5996))
* Update PATH for cross-compilation to include necessary binaries ([a43b632](https://github.com/inference-gateway/coder/commit/a43b632d11eb30e7a10644bfca1cf08689cac6f0))
* Update target from x86_64-unknown-linux-gnu to x86_64-unknown-linux-musl in release workflow ([f351ce2](https://github.com/inference-gateway/coder/commit/f351ce27a0a617837032cc2d22c247016950519f))

## [0.1.1-rc.11](https://github.com/inference-gateway/coder/compare/0.1.1-rc.10...0.1.1-rc.11) (2025-02-13)

### 👷 CI

* Increase timeout for Build and Upload Artifacts job to 25 minutes ([0b328b6](https://github.com/inference-gateway/coder/commit/0b328b6b9b7d1eb8ebf6f2690e50b0d64fcd7671))

## [0.1.1-rc.10](https://github.com/inference-gateway/coder/compare/0.1.1-rc.9...0.1.1-rc.10) (2025-02-13)

### 👷 CI

* Some cleanups ([6fdb00f](https://github.com/inference-gateway/coder/commit/6fdb00f817207588679376a029bb4590646e8dba))
* Update docker/build-push-action to version 6 in release workflow ([50e2c81](https://github.com/inference-gateway/coder/commit/50e2c8125973e1327b9e13558621d63b1c6c3ec8))

## [0.1.1-rc.9](https://github.com/inference-gateway/coder/compare/0.1.1-rc.8...0.1.1-rc.9) (2025-02-13)

### 👷 CI

* Set timeout for Build and Upload Artifacts job to 15 minutes ([5623c32](https://github.com/inference-gateway/coder/commit/5623c321d88ad6d6ac303e577d054cd0f22a0685))
* Update release workflow to use docker/build-push-action for building and pushing containers ([091c454](https://github.com/inference-gateway/coder/commit/091c454f1491afc5f1636f45cfdcf9ed968a242c))

## [0.1.1-rc.8](https://github.com/inference-gateway/coder/compare/0.1.1-rc.7...0.1.1-rc.8) (2025-02-13)

### 🐛 Bug Fixes

* Small fix ([fcede59](https://github.com/inference-gateway/coder/commit/fcede59f85a351e4baf6b23a32163c80e4766629))

## [0.1.1-rc.7](https://github.com/inference-gateway/coder/compare/0.1.1-rc.6...0.1.1-rc.7) (2025-02-13)

### 👷 CI

* Construct cache key to include with target in release workflow ([f74ac8f](https://github.com/inference-gateway/coder/commit/f74ac8f8d031b62bc747558a08106eb1f7ec8c8f))
* Try to specify explicitly buildx ([91b4ee9](https://github.com/inference-gateway/coder/commit/91b4ee92f589c1773844956b52ca00ea71227216))

## [0.1.1-rc.6](https://github.com/inference-gateway/coder/compare/0.1.1-rc.5...0.1.1-rc.6) (2025-02-13)

### 👷 CI

* Add conditional setup for QEMU and Docker Buildx only for Ubuntu ([6d29255](https://github.com/inference-gateway/coder/commit/6d292551d0b9eafeb30b812d383f93c025e70004))

## [0.1.1-rc.5](https://github.com/inference-gateway/coder/compare/0.1.1-rc.4...0.1.1-rc.5) (2025-02-13)

### 👷 CI

* Set up QEMU and Docker Buildx for using emulations ([96b9815](https://github.com/inference-gateway/coder/commit/96b981537cc71866197163efc9e5eb21a1439d02))

## [0.1.1-rc.4](https://github.com/inference-gateway/coder/compare/0.1.1-rc.3...0.1.1-rc.4) (2025-02-13)

### 👷 CI

* Add architecture platform support for Docker builds in release workflow ([7fe71aa](https://github.com/inference-gateway/coder/commit/7fe71aa1c5b4fa14c8bde6d8d0df730138e5c5f6))

## [0.1.1-rc.3](https://github.com/inference-gateway/coder/compare/0.1.1-rc.2...0.1.1-rc.3) (2025-02-13)

### 👷 CI

* Update Dockerfile to use Ubuntu base and install necessary dependencies for Rust cross-compilation ([969aab0](https://github.com/inference-gateway/coder/commit/969aab0a1c3b4e8517a4e01b831508cb908e94c8))

## [0.1.1-rc.2](https://github.com/inference-gateway/coder/compare/0.1.1-rc.1...0.1.1-rc.2) (2025-02-13)

### 👷 CI

* Simplify it cache the current pwd and enter it after download ([919427d](https://github.com/inference-gateway/coder/commit/919427dec879990e705f8da8b6f05cfbeb323856))

### 🔧 Miscellaneous

* Use a separate command for cargo to ensure workspace directory is restored ([22234d6](https://github.com/inference-gateway/coder/commit/22234d6a89d89bcc153bdb00c3632f6fe2873dc0))

## [0.1.1-rc.1](https://github.com/inference-gateway/coder/compare/0.1.0...0.1.1-rc.1) (2025-02-13)

### 👷 CI

* Remove commented-out debug flag from release workflow ([d9c9605](https://github.com/inference-gateway/coder/commit/d9c9605bc8d348849a78538145b0c36303293b2e))
