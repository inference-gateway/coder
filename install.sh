#!/bin/sh

set -e

install_coder() {
    local VERSION=$1
    local OS_LABEL=$2
    local ARCH_LABEL=$3
    local DOWNLOAD_URL=$4
    echo "Installing coder version $VERSION for $OS_LABEL $ARCH_LABEL from $DOWNLOAD_URL"
    mkdir -p "$HOME/.local/bin"
    curl -sSL "$DOWNLOAD_URL" -o "$HOME/.local/bin/coder"
    chmod +x "$HOME/.local/bin/coder"
    echo "coder installed successfully to $HOME/.local/bin/coder!\n"
}

DEP=curl
if ! command -v $DEP >/dev/null 2>&1; then
    echo "Error: $DEP is not installed. Please install $DEP and try again."
    exit 1
fi

if [ -z "$CODER_VERSION" ]; then
    VERSION=$(curl -sSL "https://api.github.com/repos/inference-gateway/coder/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    echo "Downloading latest version: $VERSION"
else
    VERSION=$CODER_VERSION
    echo "Downloading specified version: $VERSION"
fi
OS=$(uname -s)
ARCH=$(uname -m)
case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)
                TARGET="x86_64-unknown-linux-gnu"
                ARCH_LABEL="x86_64"
                ;;
            aarch64|arm64)
                TARGET="aarch64-unknown-linux-musl"
                ARCH_LABEL="ARM64"
                ;;
            *)
                echo "Unsupported architecture for Linux: $ARCH"
                exit 1
                ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64)
                TARGET="x86_64-apple-darwin"
                ARCH_LABEL="x86_64"
                ;;
            aarch64|arm64)
                TARGET="aarch64-apple-darwin"
                ARCH_LABEL="ARM64"
                ;;
            *)
                echo "Unsupported architecture for macOS: $ARCH"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/inference-gateway/coder/releases/download/$VERSION/coder_$TARGET"

install_coder "$VERSION" "$OS" "$ARCH_LABEL" "$DOWNLOAD_URL"

echo "   _____          _           "
echo "  / ____|        | |          "
echo " | |     ___   __| | ___ _ __ "
echo " | |    / _ \ / _\ |/ _ \ '__|"
echo " | |___| (_) | (_| |  __/ |   "
echo "  \_____\___/ \__,_|\___|_|   \n"

cat <<- EOF
Installation complete!

Make sure \$HOME/.local/bin is in your PATH.

To get started, follow these steps:

1. CD into your project directory (must be a git repository)
2. Run: coder init
3. Configure your environment in .coder/config.yaml
4. Run: coder index
5. Finally, run: coder fix --issue=<issue_number>
EOF