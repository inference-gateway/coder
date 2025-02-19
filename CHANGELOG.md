# Changelog

All notable changes to this project will be documented in this file.

## [0.1.4-rc.23](https://github.com/inference-gateway/coder/compare/0.1.4-rc.22...0.1.4-rc.23) (2025-02-19)

### 👷 CI

* Simplify Dockerfile path in Kaniko executor configuration - it's at the root of the repo ([9739fc2](https://github.com/inference-gateway/coder/commit/9739fc2773a34e3279ff71c0cd1ea8a1247df610))

## [0.1.4-rc.22](https://github.com/inference-gateway/coder/compare/0.1.4-rc.21...0.1.4-rc.22) (2025-02-19)

### 👷 CI

* Test something - kaniko suppose to fetch the repo and build ([7e65385](https://github.com/inference-gateway/coder/commit/7e653852461f7de06485efb3d28a6cb8f7fd5b86))

## [0.1.4-rc.21](https://github.com/inference-gateway/coder/compare/0.1.4-rc.20...0.1.4-rc.21) (2025-02-19)

### 👷 CI

* Remove explicit verbosity, use default ([5028cff](https://github.com/inference-gateway/coder/commit/5028cffb5049826ef1a873297a80c166e614ae73))
* Update Kaniko executor image to use debug version for troubleshooting ([fee3e2d](https://github.com/inference-gateway/coder/commit/fee3e2db285a48f3deb87ee3a46ef36726ca2daf))

## [0.1.4-rc.20](https://github.com/inference-gateway/coder/compare/0.1.4-rc.19...0.1.4-rc.20) (2025-02-19)

### 👷 CI

* Update container image in release workflow to use Kaniko executor ([d10ac77](https://github.com/inference-gateway/coder/commit/d10ac77fb4f62e04cfcda03aa98b69bea71bbf01))
* Use directly the container image kaniko and execute the kaniko binary to build an OCI and push it to the container registry ([68e6474](https://github.com/inference-gateway/coder/commit/68e64745475a57fa85e44699470a1ea7b1310086))

## [0.1.4-rc.19](https://github.com/inference-gateway/coder/compare/0.1.4-rc.18...0.1.4-rc.19) (2025-02-19)

### 👷 CI

* Reduce the reliance on docker ([1a1ee02](https://github.com/inference-gateway/coder/commit/1a1ee029516eefdda4b6d39d233483b9b4df0d56))

## [0.1.4-rc.18](https://github.com/inference-gateway/coder/compare/0.1.4-rc.17...0.1.4-rc.18) (2025-02-19)

### 👷 CI

* Small fix - update container syntax in release workflow to use proper image format ([5c44837](https://github.com/inference-gateway/coder/commit/5c448376d45b35f080345c5ac26dcafc665aa407))

## [0.1.4-rc.17](https://github.com/inference-gateway/coder/compare/0.1.4-rc.16...0.1.4-rc.17) (2025-02-19)

### 👷 CI

* Update release workflow to use Ubuntu 24.04 container and adjust matrix configuration ([20ffe22](https://github.com/inference-gateway/coder/commit/20ffe2257497873935f8e685515cb0ec54f869dd))

## [0.1.4-rc.16](https://github.com/inference-gateway/coder/compare/0.1.4-rc.15...0.1.4-rc.16) (2025-02-19)

### 👷 CI

* Test the same job in Kubernetes ([ac38ae3](https://github.com/inference-gateway/coder/commit/ac38ae3e1cfe8e903be8da82e88599cf38d86dea))

### 🔧 Miscellaneous

* **cleanup:** Cleanup, those labels are redundant because they now fetched from action metadata directly from github repo ([1c2415f](https://github.com/inference-gateway/coder/commit/1c2415fca8ce03b30b1cbf2909147839fdd0f417))

## [0.1.4-rc.15](https://github.com/inference-gateway/coder/compare/0.1.4-rc.14...0.1.4-rc.15) (2025-02-19)

### 👷 CI

* Update release workflow to use a different action also for kaniko, maybe this one is better maintained ([829616f](https://github.com/inference-gateway/coder/commit/829616fe0619a66f13b5dbeb7db5bf48d63373b3))

## [0.1.4-rc.14](https://github.com/inference-gateway/coder/compare/0.1.4-rc.13...0.1.4-rc.14) (2025-02-19)

### 👷 CI

* Fix syntax for build-arg in release workflow ([0dd8dbf](https://github.com/inference-gateway/coder/commit/0dd8dbf095386cd2da1374af0e3e2b13348965b9))

## [0.1.4-rc.13](https://github.com/inference-gateway/coder/compare/0.1.4-rc.12...0.1.4-rc.13) (2025-02-19)

### 👷 CI

* Fix it was targeting the wrong architecture, the option build-args doesn't seems to be available in this Github Action ([b236b67](https://github.com/inference-gateway/coder/commit/b236b6757f17fb368e463deb4b394c2b55b94e3d))

### 🔧 Miscellaneous

* Add GitHub Actions extension to devcontainer configuration ([68eb13b](https://github.com/inference-gateway/coder/commit/68eb13bf3ca6f05128585ac4ab4285404df64d17))

## [0.1.4-rc.12](https://github.com/inference-gateway/coder/compare/0.1.4-rc.11...0.1.4-rc.12) (2025-02-19)

### 👷 CI

* Update release workflow to use Kaniko action for building and pushing images ([0a9787b](https://github.com/inference-gateway/coder/commit/0a9787bbdb5f1bc3ff50ee071d791a2b9bd3cd21))

### 🔧 Miscellaneous

* Resort dev containers dependencies ([af95d59](https://github.com/inference-gateway/coder/commit/af95d592f219eae690f065723f2e56f0a8039c5b))

## [0.1.4-rc.11](https://github.com/inference-gateway/coder/compare/0.1.4-rc.10...0.1.4-rc.11) (2025-02-19)

### 👷 CI

* Comment out temporarily the other runner, just to see if it works as expected ([a01a790](https://github.com/inference-gateway/coder/commit/a01a790fa3152e29cfd67075bd3ac01439ebd077))
* Update release workflow to use Kaniko for building and pushing OCI images ([6a2b930](https://github.com/inference-gateway/coder/commit/6a2b930918b30b353f552e63c75c8424d5e2cef6))

## [0.1.4-rc.10](https://github.com/inference-gateway/coder/compare/0.1.4-rc.9...0.1.4-rc.10) (2025-02-19)

### 👷 CI

* Optimize Dockerfile by removing redundant apk update commands ([3756ab1](https://github.com/inference-gateway/coder/commit/3756ab1cb406fb901662bfad70a121660e9eaa0b))

## [0.1.4-rc.9](https://github.com/inference-gateway/coder/compare/0.1.4-rc.8...0.1.4-rc.9) (2025-02-19)

### 👷 CI

* Update Dockerfile to use rust:alpine as base image and streamline build process ([8f02226](https://github.com/inference-gateway/coder/commit/8f022262d03d8e677f451e239056314e26b6cd5c))

## [0.1.4-rc.8](https://github.com/inference-gateway/coder/compare/0.1.4-rc.7...0.1.4-rc.8) (2025-02-19)

### 👷 CI

* Increase timeout for B&P Containers job in release workflow ([8568c67](https://github.com/inference-gateway/coder/commit/8568c6744340b52f46543298a973c04e0f945e26))

## [0.1.4-rc.7](https://github.com/inference-gateway/coder/compare/0.1.4-rc.6...0.1.4-rc.7) (2025-02-19)

### 👷 CI

* Add k8s to the list of OS options in release workflow ([3422a9f](https://github.com/inference-gateway/coder/commit/3422a9fcb82382aa578395006e443a6aa57a5fe6))

## [0.1.4-rc.6](https://github.com/inference-gateway/coder/compare/0.1.4-rc.5...0.1.4-rc.6) (2025-02-17)

### 👷 CI

* Comment out QEMU setup step in release workflow ([4a09c1a](https://github.com/inference-gateway/coder/commit/4a09c1a2d5c3a7d441c65d2ba7bff8ecc184a0ce))
* Comment out temporarily build_binaries job in release workflow to speed things up ([4d00ad3](https://github.com/inference-gateway/coder/commit/4d00ad3ecd9644b57a2218030681c99a87b3c416))
* Remove conditional check for minimal variant in B&P Minimal Container job ([ca6f85f](https://github.com/inference-gateway/coder/commit/ca6f85fc5ba7d85c13f113c15709b832ded25615))
* Rename build_container job to build_containers and update commented job name ([1bdf885](https://github.com/inference-gateway/coder/commit/1bdf885cd539045a0cdf816d5672b021bb575c3b))
* Split language specific containers with tools to a separate job and comment it out ([fd20e5b](https://github.com/inference-gateway/coder/commit/fd20e5b0c5e6781706222b59bd99c268ea441a35))

## [0.1.4-rc.5](https://github.com/inference-gateway/coder/compare/0.1.4-rc.4...0.1.4-rc.5) (2025-02-17)

### 👷 CI

* Comment out Build and Push Rust Container job in release workflow ([8e8af76](https://github.com/inference-gateway/coder/commit/8e8af765f3d6506891a4482eafbef0bd557e419b))

## [0.1.4-rc.4](https://github.com/inference-gateway/coder/compare/0.1.4-rc.3...0.1.4-rc.4) (2025-02-17)

### 👷 CI

* Update release workflow to handle macOS builds separately ([d242dce](https://github.com/inference-gateway/coder/commit/d242dce5f5032754271e85fa9f487a4efe7b8037))
* Update release workflow to include ubuntu-22.04-arm64 in self-hosted jobs ([74429be](https://github.com/inference-gateway/coder/commit/74429be18ee3401e6e23920c70c985d388ff9a7e))

## [0.1.4-rc.3](https://github.com/inference-gateway/coder/compare/0.1.4-rc.2...0.1.4-rc.3) (2025-02-17)

### 👷 CI

* Change ARM64 jobs to use self-hosted runners in release workflow ([274d375](https://github.com/inference-gateway/coder/commit/274d375bdb2d888de200706001e50986600e4919))

## [0.1.4-rc.2](https://github.com/inference-gateway/coder/compare/0.1.4-rc.1...0.1.4-rc.2) (2025-02-16)

### ♻️ Improvements

* Update Dockerfile and CI workflow to use clang and llvm for cross-compilation ([c0cf8d3](https://github.com/inference-gateway/coder/commit/c0cf8d3fa99a87f09758232f7b99c094f95e3abb))

### 👷 CI

* Increase timeout for B&P Containers job to 65 minutes ([c90a1f4](https://github.com/inference-gateway/coder/commit/c90a1f46005328045e6020237969608bf3cc04de))
* Make the Build and Push job name shorter ([a41ed31](https://github.com/inference-gateway/coder/commit/a41ed31b16c3848082859127c1f1f955442120d8))
* Reduce timeout for B&P Containers job from 65 to 15 minutes ([f610944](https://github.com/inference-gateway/coder/commit/f6109443895e59249226061aa62ea5645b99677c))
* Update Ubuntu version in release workflow to 24.04 and add arm64 runner ([41e2c5f](https://github.com/inference-gateway/coder/commit/41e2c5fa2baa3eb4042697cd036a302d07fa3f84))

## [0.1.4-rc.1](https://github.com/inference-gateway/coder/compare/0.1.3...0.1.4-rc.1) (2025-02-16)

### ♻️ Improvements

* **docker:** Simplify Dockerfile and update build process with Alpine base image ([ccf6275](https://github.com/inference-gateway/coder/commit/ccf627589983850cd1937ba33266b742fff41afa))

### 🐛 Bug Fixes

* **docker-compose:** Update coder service image version to 0.1.3 with alpine base image ([c2d26b0](https://github.com/inference-gateway/coder/commit/c2d26b0af2a1c29fbcb616baa0162d5efc2e2662))
* **docker:** Add git and curl to the container image for enhanced functionality ([b21b49b](https://github.com/inference-gateway/coder/commit/b21b49bf98497fe52729bb6f834d9bf1cd44596f))

### 👷 CI

* Enhance GitHub Actions workflow for building and pushing Docker containers with Rust tools support and minimal ones ([3a50346](https://github.com/inference-gateway/coder/commit/3a5034662619017c5bb7a3fa9419b23ae4081320))

## [0.1.3](https://github.com/inference-gateway/coder/compare/0.1.2...0.1.3) (2025-02-15)

### 🐛 Bug Fixes

* **docker-compose:** Clean up entrypoint and update service names in docker-compose.yml ([8c3fa07](https://github.com/inference-gateway/coder/commit/8c3fa070698ffc36a96033c12c0a7116f00aa38a))
* **docker:** Replace distroless final base image with Alpine and update user permissions ([123c932](https://github.com/inference-gateway/coder/commit/123c932afd5fbf416d583dbc37b42e2ac99e3893))

### 📚 Documentation

* Create example for docker compose ([#22](https://github.com/inference-gateway/coder/issues/22)) ([f3466c9](https://github.com/inference-gateway/coder/commit/f3466c9ca96f5fd2c50643be0ac36aff99e4e497))

### 🔧 Miscellaneous

* **docker-compose:** Resort the services - put the important ones at the top ([a6338c0](https://github.com/inference-gateway/coder/commit/a6338c0205e884955a8b801f5ff663ee02469a80))
* **docker-compose:** Update inference gateway URL and adjust user permissions in docker-compose ([e356343](https://github.com/inference-gateway/coder/commit/e356343add4dc19c3b2f6f869e3b2579630174e1))
* **docker:** Add TODOs for base image replacement and rethink distroless choice ([91b9fe3](https://github.com/inference-gateway/coder/commit/91b9fe346fab2bf06974c49d24f65e0ca9b1d7bf))

## [0.1.2](https://github.com/inference-gateway/coder/compare/0.1.1...0.1.2) (2025-02-14)

### 📚 Documentation

* Add Docker section to table of contents in README.md ([67195c1](https://github.com/inference-gateway/coder/commit/67195c1528c19967ef8c99507bb5ea1fc0213d0a))
* Add Docker usage instructions and update configuration section in README.md ([102e371](https://github.com/inference-gateway/coder/commit/102e371fb15e6f7d6a7a4cc28611f376a7f444c4))
* Add example configuration for AI Coder in README.md ([1725372](https://github.com/inference-gateway/coder/commit/1725372119214c50e30ae58006a2797b844ebeba))
* Correct container weight description in README.md ([ce68686](https://github.com/inference-gateway/coder/commit/ce68686860bb69c3151a3bacd01e1369b970b644))
* Simplify configuration section in README.md for clarity and consistency ([8b834ba](https://github.com/inference-gateway/coder/commit/8b834baee410ce0d0cfb1ccb9160f14cf8a9c4c9))

### 🔧 Miscellaneous

* Add environment variables ([#20](https://github.com/inference-gateway/coder/issues/20)) ([ef292f1](https://github.com/inference-gateway/coder/commit/ef292f1f33d06eb0cc3fdcf8d2daa56d0c61929a))
* Add version information to CLI command ([#21](https://github.com/inference-gateway/coder/issues/21)) ([f86f0cc](https://github.com/inference-gateway/coder/commit/f86f0cc5bc38db1534ae1435cf4418da5af6ce77))

### ✅ Miscellaneous

* Add serial test attribute to ensure test execution order ([c6ac947](https://github.com/inference-gateway/coder/commit/c6ac947f08a61732e6e29bf812e8083247d9566f))

## [0.1.1](https://github.com/inference-gateway/coder/compare/0.1.0...0.1.1) (2025-02-14)

### 🐛 Bug Fixes

* Small fix ([fcede59](https://github.com/inference-gateway/coder/commit/fcede59f85a351e4baf6b23a32163c80e4766629))

### 👷 CI

* Add architecture platform support for Docker builds in release workflow ([7fe71aa](https://github.com/inference-gateway/coder/commit/7fe71aa1c5b4fa14c8bde6d8d0df730138e5c5f6))
* Add Cargo configuration for musl targets with static linking ([6b647e9](https://github.com/inference-gateway/coder/commit/6b647e9c3989dde9b27f2571df0fcc4396d82e95))
* Add conditional setup for QEMU and Docker Buildx only for Ubuntu ([6d29255](https://github.com/inference-gateway/coder/commit/6d292551d0b9eafeb30b812d383f93c025e70004))
* Adjust timeouts for cross-compilation jobs in release workflow ([4f5d9c8](https://github.com/inference-gateway/coder/commit/4f5d9c87405ffd081096d392fdbca2bfa1646770))
* Construct cache key to include with target in release workflow ([f74ac8f](https://github.com/inference-gateway/coder/commit/f74ac8f8d031b62bc747558a08106eb1f7ec8c8f))
* Improve release process publish binaries and containers for all common platforms ([1c7637b](https://github.com/inference-gateway/coder/commit/1c7637be91bfe2f4704971af0777b9fc2e1d3cdc))
* Increase timeout for Build and Upload Artifacts job to 25 minutes ([0b328b6](https://github.com/inference-gateway/coder/commit/0b328b6b9b7d1eb8ebf6f2690e50b0d64fcd7671))
* Ok this setup works if I try to cross compile from macos arm64 to x86 so it supposed to also work the other way from the runner ([c7d05cc](https://github.com/inference-gateway/coder/commit/c7d05cc6f2313f2a6e327fe68e699cd657da43c0))
* Remove commented-out debug flag from release workflow ([d9c9605](https://github.com/inference-gateway/coder/commit/d9c9605bc8d348849a78538145b0c36303293b2e))
* Set timeout for Build and Upload Artifacts job to 15 minutes ([5623c32](https://github.com/inference-gateway/coder/commit/5623c321d88ad6d6ac303e577d054cd0f22a0685))
* Set up QEMU and Docker Buildx for using emulations ([96b9815](https://github.com/inference-gateway/coder/commit/96b981537cc71866197163efc9e5eb21a1439d02))
* Simplify it cache the current pwd and enter it after download ([919427d](https://github.com/inference-gateway/coder/commit/919427dec879990e705f8da8b6f05cfbeb323856))
* Some cleanups ([6fdb00f](https://github.com/inference-gateway/coder/commit/6fdb00f817207588679376a029bb4590646e8dba))
* Try to specify explicitly buildx ([91b4ee9](https://github.com/inference-gateway/coder/commit/91b4ee92f589c1773844956b52ca00ea71227216))
* Update cross-compilation setup for musl targets and improve build tools installation ([2ea56e4](https://github.com/inference-gateway/coder/commit/2ea56e45db9e06713049d10d7d4f00d9d4cd5996))
* Update docker/build-push-action to version 6 in release workflow ([50e2c81](https://github.com/inference-gateway/coder/commit/50e2c8125973e1327b9e13558621d63b1c6c3ec8))
* Update Dockerfile to use Ubuntu base and install necessary dependencies for Rust cross-compilation ([969aab0](https://github.com/inference-gateway/coder/commit/969aab0a1c3b4e8517a4e01b831508cb908e94c8))
* Update PATH for cross-compilation to include necessary binaries ([a43b632](https://github.com/inference-gateway/coder/commit/a43b632d11eb30e7a10644bfca1cf08689cac6f0))
* Update release workflow to use docker/build-push-action for building and pushing containers ([091c454](https://github.com/inference-gateway/coder/commit/091c454f1491afc5f1636f45cfdcf9ed968a242c))
* Update target from x86_64-unknown-linux-gnu to x86_64-unknown-linux-musl in release workflow ([f351ce2](https://github.com/inference-gateway/coder/commit/f351ce27a0a617837032cc2d22c247016950519f))
* Use sudo for apt-get commands in release workflow for musl target ([904f418](https://github.com/inference-gateway/coder/commit/904f418fb8c701e298f92d483060609acd5fa4bd))
* Use sudo for cleanup and moving musl cross-compilation tools in release workflow ([3dd974f](https://github.com/inference-gateway/coder/commit/3dd974fb8f33f13baecb79dbd7d145a10532d74d))

### 🔧 Miscellaneous

* **release:** 🔖 0.1.1-rc.1 [skip ci] ([c19ec65](https://github.com/inference-gateway/coder/commit/c19ec657aa74d9cf80938fe36c10fe7f24d0cf0a))
* **release:** 🔖 0.1.1-rc.10 [skip ci] ([fda3dd3](https://github.com/inference-gateway/coder/commit/fda3dd305d7ee37e0a673289ae4fa7f7767d3605))
* **release:** 🔖 0.1.1-rc.11 [skip ci] ([71b52d9](https://github.com/inference-gateway/coder/commit/71b52d9b9dc09bf8274c62ee854b81645ce47054))
* **release:** 🔖 0.1.1-rc.12 [skip ci] ([3064dad](https://github.com/inference-gateway/coder/commit/3064dad729aef94219b5558c9455ca439c0e11de))
* **release:** 🔖 0.1.1-rc.13 [skip ci] ([0821ce5](https://github.com/inference-gateway/coder/commit/0821ce5cda3318050ea9225215e10be8b6c611ff))
* **release:** 🔖 0.1.1-rc.14 [skip ci] ([1a840a0](https://github.com/inference-gateway/coder/commit/1a840a07c47181e3d4a886ccd184125791569e19))
* **release:** 🔖 0.1.1-rc.15 [skip ci] ([eae8ff3](https://github.com/inference-gateway/coder/commit/eae8ff33249740bd752abc67a7b6cf4a59969209))
* **release:** 🔖 0.1.1-rc.16 [skip ci] ([876c48d](https://github.com/inference-gateway/coder/commit/876c48dbfb1b69d45bc769c81ad817152a54cd58))
* **release:** 🔖 0.1.1-rc.2 [skip ci] ([d749cac](https://github.com/inference-gateway/coder/commit/d749cac352feb0d78695acd6da562c2ac6d0a92c))
* **release:** 🔖 0.1.1-rc.3 [skip ci] ([2de1fca](https://github.com/inference-gateway/coder/commit/2de1fcaa425e50730882c2914cb475b78d4b73ae))
* **release:** 🔖 0.1.1-rc.4 [skip ci] ([3a17396](https://github.com/inference-gateway/coder/commit/3a173960f2b85da8d4299b5b6e8ac7bc4a03b058))
* **release:** 🔖 0.1.1-rc.5 [skip ci] ([7b4821b](https://github.com/inference-gateway/coder/commit/7b4821ba6a9d488227d30beaa6bf81707f83b8c1))
* **release:** 🔖 0.1.1-rc.6 [skip ci] ([2370f0f](https://github.com/inference-gateway/coder/commit/2370f0f95cab7ebfa17b38114db5feaded673a77))
* **release:** 🔖 0.1.1-rc.7 [skip ci] ([33948a3](https://github.com/inference-gateway/coder/commit/33948a35c69315f9709c3d70a89a9339455d5cdd))
* **release:** 🔖 0.1.1-rc.8 [skip ci] ([31598f4](https://github.com/inference-gateway/coder/commit/31598f414828479efc323046cccff3fd6a8d1dda))
* **release:** 🔖 0.1.1-rc.9 [skip ci] ([df458d9](https://github.com/inference-gateway/coder/commit/df458d9b14dff528f7d45306b2d14dd299a591fb))
* Use a separate command for cargo to ensure workspace directory is restored ([22234d6](https://github.com/inference-gateway/coder/commit/22234d6a89d89bcc153bdb00c3632f6fe2873dc0))

## [0.1.1-rc.16](https://github.com/inference-gateway/coder/compare/0.1.1-rc.15...0.1.1-rc.16) (2025-02-14)

### 👷 CI

* Adjust timeouts for cross-compilation jobs in release workflow ([4f5d9c8](https://github.com/inference-gateway/coder/commit/4f5d9c87405ffd081096d392fdbca2bfa1646770))

## [0.1.1-rc.15](https://github.com/inference-gateway/coder/compare/0.1.1-rc.14...0.1.1-rc.15) (2025-02-14)

### 👷 CI

* Add Cargo configuration for musl targets with static linking ([6b647e9](https://github.com/inference-gateway/coder/commit/6b647e9c3989dde9b27f2571df0fcc4396d82e95))

## [0.1.1-rc.14](https://github.com/inference-gateway/coder/compare/0.1.1-rc.13...0.1.1-rc.14) (2025-02-13)

### 👷 CI

* Use sudo for cleanup and moving musl cross-compilation tools in release workflow ([3dd974f](https://github.com/inference-gateway/coder/commit/3dd974fb8f33f13baecb79dbd7d145a10532d74d))

## [0.1.1-rc.13](https://github.com/inference-gateway/coder/compare/0.1.1-rc.12...0.1.1-rc.13) (2025-02-13)

### 👷 CI

* Use sudo for apt-get commands in release workflow for musl target ([904f418](https://github.com/inference-gateway/coder/commit/904f418fb8c701e298f92d483060609acd5fa4bd))

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
