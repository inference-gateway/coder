<h1 align="center">🦀 Coder 🦀</h1>

<p align="center">
    <a href="https://github.com/inference-gateway/coder/actions"><img src="https://github.com/inference-gateway/coder/actions/workflows/ci.yml/badge.svg" alt="CI Status"/></a>
    <a href="https://github.com/inference-gateway/coder/releases"><img src="https://img.shields.io/github/v/release/inference-gateway/coder?color=blue&style=flat-square" alt="Version"/></a>
    <a href="https://github.com/inference-gateway/coder/blob/main/LICENSE"><img src="https://img.shields.io/github/license/inference-gateway/coder?color=blue&style=flat-square" alt="License"/></a>
</p>

The tool you want to hide from your boss.

Table of content:

- [What is Coder?](#what-is-coder)
- [Why Coder?](#why-coder)
- [Key Features](#key-features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Getting Started](#getting-started)
- [Contributing](#contributing)
- [License](#license)

### What is Coder?

Coder is an AI-powered assistant that automatically reviews issues and creates pull requests/merge requests with suggested fixes for GitHub and GitLab repositories. It is built with Rust for performance and reliability, and it supports the latest reasoning LLMs (DeepSeek-r1) for code analysis and improvement.

### Why Coder?

Unlike many tools that are either open-source UIs bloated and loaded with dependencies or costly, proprietary solutions, Coder is simple, free, and runs directly from your terminal - giving you true freedom. You can also automate it in remote environments like Kubernetes to parallelize tasks, control resources, and scale on demand. With a lightweight footprint and zero extra dependencies, Coder seamlessly integrates on Kubernetes, Docker, or locally, adapts to your project structure, and supports configurable AI models.

Coder understands project structure rather than just a specific code snippet or a file. It can analyze the project and suggest fixes based on the context of the project. It can also fetch the latest data when activated, thanks to RAG-enabled reasoning. It supports the latest reasoning LLMs (DeepSeek-r1) for code analysis and improvement, and it can be configured to use only the AI models with tool-use support.

### Key Features

- 📜 Open-source and free to use
- 🚀 Automatic issue analysis and code review
- 🤖 AI-generated code fixes and improvements
- 🔗 Support for both GitHub PRs and GitLab MRs
- ⚡ Built with Rust for performance and reliability
- 🎛️ Configurable AI model selection (only the ones with tool-use support)
- 🤖 Works with the latest and greatest reasoning LLMs (DeepSeek-r1)
- 🔄 RAG-enabled reasoning: fetches the latest data when activated 🚀
- 🗂️ Project structure aware analysis
- 📦 Easy to install and use
- 💻 Run directly from your terminal – enjoy true freedom from vendor lock-in! 🔓
- 🌐 Cross-platform support
- 🚀 Runs on ☸️ Kubernetes, 🐳 Docker, and 💻 local environments
- 🤖 Automate remote tasks with Kubernetes: parallelize, scale, and control resources on demand
- 📦 Lightweight footprint - binary weight ~ 11MB container weight ~ 17MB
- 🚀 Easy to deploy

### Installation

Just run:

```bash
curl -sSL https://raw.githubusercontent.com/inference-gateway/coder/refs/heads/main/install.sh | sh
```

### Usage

1. Initialize the project:

```bash
coder init
```

This will generate configuration files in the .coder directory. Review them and configure the assistant.

2. Index the project:

```bash
coder index
```

This will index the project files and prepare all the information necessary for the assistant so it's easily digestible.

3. Send the assistant to fix an issue:

```bash
coder fix --issue=#1
```

### Using Docker

TODO - Add more environment variables

You can also use Docker to run the assistant:

```bash
docker run -v $(pwd):/app -w /app ghcr.io/inference-gateway/coder:latest init
docker run -v $(pwd):/app -w /app ghcr.io/inference-gateway/coder:latest index
docker run -v $(pwd):/app -w /app ghcr.io/inference-gateway/coder:latest fix --issue=#1
```

### Configuration

TODO - Add more environment variables

Configuration is stored in .coder/config.yaml file. You can customize the configuration by editing this file.

Running:

```bash
coder init
```

Will result in the following available configurations:

```yaml
---
# AI Coder Configuration
language:
  name: rust # The language of the project
  analyse: # The command you use for static analysis
    - cargo clippy
  linter: # The command you use for linting
    - cargo fmt
  test_commands: # The command you use for testing
    - cargo test
  docs_urls: # The URL for the documentations so the agent can reference them
    - https://docs.rs
scm:
  name: github # The SCM you are using (e.g. github, gitlab)
  owner: owner # The owner of the repository
  repository: repo # The repository name
  issue_template:
    |- # The issue template, the agent will run a tool to validate the template contain all necessary information before proceeding
    ## Description
    ## Steps to Reproduce
    ## Expected Behavior
    ## Actual Behavior
    ## Environment
agent:
  model: deepseek-r1-distill-llama-70b
  provider: groq
  max_tokens: 4000
api:
  endpoint: http://localhost:8080 # The endpoint of the deployed Inferece-Gateway API
```

### Development

#### Prerequisites

- Docker
- Devcontainers

#### Getting Started

For development tasks, you can use the following commands:

1. To start the backend gateway:

```bash
task gateway
```

2. To build the project:

```bash
task build
```

3. To execute the agent coder:

```bash
task run
```

4. To execute a specific command of the coder:

```bash
task run -- <command>
```

5. To run the tests:

```bash
task test
```

### Contributing

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Commit your changes
5. Push your changes
6. Create a new pull request

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
