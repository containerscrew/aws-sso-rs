#!/usr/bin/env sh

set -e

# Global vars
INSTALLATION_PATH="${INSTALLATION_PATH:-/usr/local/bin}" # Override: INSTALLATION_PATH=$HOME/.local/bin sh install.sh
BINARY_NAME="aws-sso-rs"
REPO="containerscrew/aws-sso-rs"

# Welcome message
echo "Welcome to the $BINARY_NAME installation script! 🚀"
echo "Author: github.com/containerscrew"

happyexit(){
  echo ""
  echo "$BINARY_NAME successfully installed! 🎉"
  echo ""
  echo "Now run: $ $BINARY_NAME --help"
  echo ""
  exit 0
}

info() {
  printf "\033[0;32m[info] - %s \033[0m\n" "$1"
}

# Detect OS and Architecture
OS_RAW=$(uname -s)
ARCH_RAW=$(uname -m)

case $OS_RAW in
  Linux)
    OS="linux"
    ;;
  Darwin)
    OS="darwin"
    ;;
  *)
    echo "❌ Error: There is no $BINARY_NAME support for OS: $OS_RAW"
    exit 1
    ;;
esac

case $ARCH_RAW in
  x86_64)
    CLI_ARCH="amd64"
    ;;
  armv8* | aarch64* | arm64)
    CLI_ARCH="arm64"
    ;;
  *)
    echo "❌ Error: There is no $BINARY_NAME support for architecture: $ARCH_RAW"
    exit 1
    ;;
esac

for tool in curl unzip; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "❌ Error: '$tool' is required but not installed"
    exit 1
  fi
done

# Everything is downloaded/extracted here and removed on exit
WORKDIR=$(mktemp -d)
trap 'rm -rf "$WORKDIR"' EXIT

download_release() {
  # Get latest version using grep/sed to avoid depending on jq
  LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

  if [ -z "$1" ]; then
    TAG_VERSION=$LATEST_TAG
  else
    TAG_VERSION=$1
  fi

  if [ -z "$TAG_VERSION" ]; then
    echo "❌ Error: could not determine the version to install"
    exit 1
  fi

  # Release tags have the 'v' prefix (v1.5.0). Accept both 1.5.0 and v1.5.0
  case $TAG_VERSION in
    v*) ;;
    *) TAG_VERSION="v$TAG_VERSION" ;;
  esac

  FILENAME="${BINARY_NAME}-${OS}-${CLI_ARCH}-${TAG_VERSION}.zip"
  BASE_URL="https://github.com/$REPO/releases/download/${TAG_VERSION}"

  info "OS: $OS | Arch: $CLI_ARCH | Version: $TAG_VERSION"
  info "Downloading $FILENAME..."

  DOWNLOADED_FILE="$WORKDIR/$FILENAME"
  CHECKSUM_FILE="$WORKDIR/$FILENAME.sha256"

  curl -L --fail "$BASE_URL/$FILENAME" -o "$DOWNLOADED_FILE"

  # Releases older than the checksum support do not have a .sha256 file
  info "Downloading $FILENAME.sha256..."
  if ! curl -L --fail -s "$BASE_URL/$FILENAME.sha256" -o "$CHECKSUM_FILE"; then
    rm -f "$CHECKSUM_FILE"
  fi
}

# Verify SHA256 checksum of the downloaded artifact against its .sha256 file.
# Tries sha256sum, shasum -a 256, then openssl dgst -sha256.
verify_checksum() {
  info "Verifying checksum..."

  if [ ! -f "$CHECKSUM_FILE" ]; then
    echo "⚠  Warning: checksum file not found — skipping verification"
    return 0
  fi

  EXPECTED=$(awk '{print $1}' "$CHECKSUM_FILE")

  if [ -z "$EXPECTED" ]; then
    echo "❌ Error: empty checksum entry for $FILENAME"
    exit 1
  fi

  # Pick available sha256 tool
  if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$DOWNLOADED_FILE" | awk '{print $1}')
  elif command -v shasum >/dev/null 2>&1; then
    ACTUAL=$(shasum -a 256 "$DOWNLOADED_FILE" | awk '{print $1}')
  elif command -v openssl >/dev/null 2>&1; then
    ACTUAL=$(openssl dgst -sha256 "$DOWNLOADED_FILE" | awk '{print $2}')
  else
    echo "⚠  Warning: no sha256 tool found (sha256sum, shasum, openssl) — skipping verification"
    return 0
  fi

  if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "❌ Error: checksum verification failed!"
    echo "  Expected: $EXPECTED"
    echo "  Got:      $ACTUAL"
    exit 1
  fi

  info "Checksum verified ✓"
}

execute_with_sudo() {
  if [ "$(id -u)" = 0 ]; then
    "$@"
  else
    sudo "$@"
  fi
}

install_binary(){
  info "Installing $BINARY_NAME in $INSTALLATION_PATH..."

  unzip -o "$DOWNLOADED_FILE" "$BINARY_NAME" -d "$WORKDIR"

  # Only escalate privileges when the target directory cannot be created/written as the current user
  mkdir -p "$INSTALLATION_PATH" 2>/dev/null || true
  if [ -d "$INSTALLATION_PATH" ] && [ -w "$INSTALLATION_PATH" ]; then
    mv "$WORKDIR/$BINARY_NAME" "$INSTALLATION_PATH/$BINARY_NAME"
    chmod +x "$INSTALLATION_PATH/$BINARY_NAME"
  else
    execute_with_sudo mkdir -p "$INSTALLATION_PATH"
    execute_with_sudo mv "$WORKDIR/$BINARY_NAME" "$INSTALLATION_PATH/$BINARY_NAME"
    execute_with_sudo chmod +x "$INSTALLATION_PATH/$BINARY_NAME"
  fi
}

# Function to display help text
usage() {
    echo "Usage: $0 [-v <version>] [-h]"
    echo "Options:"
    echo "  -v           Select which version do you want to install (e.g., 1.5.0 or v1.5.0)."
    echo "  -h           Display this help message."
    echo ""
    echo "Environment:"
    echo "  INSTALLATION_PATH   Directory where the binary is installed (default: /usr/local/bin)."
}

# Parse options using getopts
while getopts "v:h" option; do
    case "${option}" in
        v)
            VERSION_ARG=${OPTARG}
            download_release "$VERSION_ARG"
            verify_checksum
            install_binary
            happyexit
            ;;
        h)
            usage
            exit 0
            ;;
        \?)
            usage
            exit 1
            ;;
    esac
done

# If no flags, install latest version by default
if [ $# -eq 0 ]; then
    download_release
    verify_checksum
    install_binary
    happyexit
fi
