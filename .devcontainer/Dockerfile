FROM mcr.microsoft.com/devcontainers/rust:1.0.20-bookworm 

ENV ZSH_CUSTOM=/home/vscode/.oh-my-zsh/custom \
    TASK_VERSION=v3.41.0 \
    DIVE_VERSION=0.12.0 \
    TRIVY_VERSION=0.59.1

RUN apt-get update && \
    # Install nodejs and npm
    curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    # Install Task
    curl -s https://taskfile.dev/install.sh | sh -s -- -b /usr/local/bin ${TASK_VERSION} && \
    # Install Dive 
    curl -fsSL https://github.com/wagoodman/dive/releases/download/v${DIVE_VERSION}/dive_${DIVE_VERSION}_linux_arm64.tar.gz -o /tmp/dive_${DIVE_VERSION}_linux_arm64.tar.gz && \
    tar -xzf /tmp/dive_${DIVE_VERSION}_linux_arm64.tar.gz -C /usr/local/bin && \
    chmod +x /usr/local/bin/dive && \
    # Install Trivy
    curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v${TRIVY_VERSION} && \
    # Cleanup
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/*

RUN npm install -g semantic-release @semantic-release/changelog @semantic-release/exec @semantic-release/git @semantic-release/github conventional-changelog-conventionalcommits

RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM}/themes/powerlevel10k

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    musl-tools && \
    rustup target add \
    aarch64-unknown-linux-musl \
    x86_64-unknown-linux-musl && \
    cd /tmp && \
    wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
    tar -xzf aarch64-linux-musl-cross.tgz && \
    mv aarch64-linux-musl-cross /opt/ && \
    rm -rf aarch64-linux-musl-cross.tgz && \
    wget https://musl.cc/x86_64-linux-musl-cross.tgz && \
    tar -xzf x86_64-linux-musl-cross.tgz && \
    mv x86_64-linux-musl-cross /opt/ && \
    rm -rf x86_64-linux-musl-cross.tgz

USER vscode