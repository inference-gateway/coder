## Coder

An AI-powered assistant that automatically reviews issues and creates pull requests/merge requests with suggested fixes for GitHub and GitLab repositories.

### Features

- Automatic issue analysis and code review
- AI-generated code fixes and improvements
- Support for both GitHub PRs and GitLab MRs
- Built with Rust for performance and reliability
- Configurable AI model selection
- Project structure aware analysis

### Prerequisites

- Docker
- Devcontainers

### Installation

1. Run:

```bash
cargo install coder
```

### Usage

1. Initialize the project:

```bash
coder init
```

2. Index the project:

```bash
coder index
```

3. Run the assistant:

```bash
coder start
```

### Development

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

### Configuration

Configuration is stored in .coder/config.toml file. You can customize the configuration by editing this file.

Creating a new configuration file:

```bash
coder init
```

### Contributing

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Commit your changes
5. Push your changes

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
