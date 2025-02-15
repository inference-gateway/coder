## Docker Compose Quick Guide

This guide will help you get started with Docker Compose. It will walk you through the process of creating a simple Docker Compose file and running it.

1. Create .env file from example:

```bash
cp .env.example .env
```

2. Configure the environment variables in .env:

```bash
# Required settings
CODER_INFERENCE_GATEWAY_URL=http://inference-gateway:8080
CODER_SCM_NAME=github
CODER_SCM_TOKEN=<your-github-token>
CODER_SCM_USERNAME=<your-username>
CODER_SCM_REPOSITORY=<your-repo>
```

Also configure at least one provider, it works well with Groq, but you can use any other provider.

3. Adjust the `CODER_ISSUE` to reference the ID of the issue you created on Github or Gitlab.

## Usage

### Start the services

```bash
docker-compose up
```

Inspect the logs and notice that the services will be up and running in the following order:

1. The `inference-gateway` service will start.
2. At the same time the `repository-cloner` service will start - it will pull the repository from SCM.
3. Finally the coder will use the same mount volume of the repository-cloner and start working on the issue.

### Troubleshooting

If you see the error `repository-cloner-1  | fatal: destination path '.' already exists and is not an empty directory.`

That means the repository is already cloned, so you need to remove the volume and start again:

```bash
docker compose down
docker volume prune -a
```
