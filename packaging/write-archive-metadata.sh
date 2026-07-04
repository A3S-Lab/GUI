#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 4 ]]; then
    echo "usage: $0 <archive-path> <platform> <artifact-name> <metadata-path>" >&2
    exit 2
fi

archive_path="$1"
platform="$2"
artifact_name="$3"
metadata_path="$4"

sha256_file() {
    local file="$1"

    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$file" | awk '{print $1}'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$file" | awk '{print $1}'
    elif command -v certutil >/dev/null 2>&1; then
        certutil -hashfile "$file" SHA256 | sed -n '2p' | tr -d ' \r' | tr 'A-F' 'a-f'
    else
        echo "missing SHA-256 tool; install sha256sum or shasum" >&2
        exit 1
    fi
}

file_size() {
    local file="$1"

    if stat -c '%s' "$file" >/dev/null 2>&1; then
        stat -c '%s' "$file"
    elif stat -f '%z' "$file" >/dev/null 2>&1; then
        stat -f '%z' "$file"
    else
        wc -c < "$file" | tr -d ' '
    fi
}

[[ -f "$archive_path" ]] || { echo "missing archive: $archive_path" >&2; exit 1; }
mkdir -p "$(dirname "$metadata_path")"

archive_name="$(basename "$archive_path")"
checksum="$(sha256_file "$archive_path")"
bytes="$(file_size "$archive_path")"
commit="$(git rev-parse --short=12 HEAD 2>/dev/null || printf 'unknown')"

{
    echo "A3S GUI Dogfood Archive Metadata"
    echo "platform: $platform"
    echo "artifact: $artifact_name"
    echo "archive: $archive_name"
    echo "archiveSha256: $checksum"
    echo "archiveBytes: $bytes"
    echo "sourceCommit: $commit"
    echo "generatedAtUtc: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    echo "format: key: value"
    echo
    echo "contents:"
    echo "- $archive_name"
    echo "- $archive_name.sha256"
    echo "- $archive_name.metadata.txt"
} > "$metadata_path"

echo "wrote $metadata_path"
