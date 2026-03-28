#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?Usage: update-packaging.sh <version> <artifacts-dir>}"
ARTIFACTS_DIR="${2:?Usage: update-packaging.sh <version> <artifacts-dir>}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Strip leading 'v' if present
VERSION="${VERSION#v}"

echo "Updating packaging templates for version ${VERSION}"
echo "Artifacts directory: ${ARTIFACTS_DIR}"

compute_sha256() {
    local file="$1"
    if command -v sha256sum &>/dev/null; then
        sha256sum "$file" | awk '{print $1}'
    else
        shasum -a 256 "$file" | awk '{print $1}'
    fi
}

# Compute hashes for all artifacts
WIN_X64_SHA256=$(compute_sha256 "${ARTIFACTS_DIR}/mdterm-win-x64.zip")
LINUX_X64_SHA256=$(compute_sha256 "${ARTIFACTS_DIR}/mdterm-linux-x64.tar.gz")
OSX_X64_SHA256=$(compute_sha256 "${ARTIFACTS_DIR}/mdterm-osx-x64.tar.gz")
OSX_ARM64_SHA256=$(compute_sha256 "${ARTIFACTS_DIR}/mdterm-osx-arm64.tar.gz")

echo ""
echo "Computed hashes:"
echo "  win-x64:   ${WIN_X64_SHA256}"
echo "  linux-x64: ${LINUX_X64_SHA256}"
echo "  osx-x64:   ${OSX_X64_SHA256}"
echo "  osx-arm64: ${OSX_ARM64_SHA256}"
echo ""

# Create output directory for rendered templates
OUTPUT_DIR="${ARTIFACTS_DIR}/packaging"
mkdir -p "${OUTPUT_DIR}"/{winget,scoop,chocolatey/tools,homebrew,snap,aur}

# Fill in templates
for dir in winget scoop chocolatey homebrew snap aur; do
    for template in "${SCRIPT_DIR}/${dir}"/**/* "${SCRIPT_DIR}/${dir}"/*; do
        [ -f "$template" ] || continue
        filename=$(basename "$template")
        output="${OUTPUT_DIR}/${dir}/${filename}"

        sed \
            -e "s/VERSION_PLACEHOLDER/${VERSION}/g" \
            -e "s/SHA256_PLACEHOLDER/${WIN_X64_SHA256}/g" \
            -e "s/SHA256_ARM64_PLACEHOLDER/${OSX_ARM64_SHA256}/g" \
            -e "s/SHA256_X64_PLACEHOLDER/${OSX_X64_SHA256}/g" \
            -e "s/SHA256_LINUX_PLACEHOLDER/${LINUX_X64_SHA256}/g" \
            "$template" > "$output"
    done
done

echo "Rendered packaging templates to ${OUTPUT_DIR}"
