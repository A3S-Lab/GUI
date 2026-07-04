#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 5 ]]; then
    echo "usage: $0 <archive-path> <sha256-path> <metadata-path> <platform> <artifact-name>" >&2
    exit 2
fi

archive_path="$1"
sha256_path="$2"
metadata_path="$3"
platform="$4"
artifact_name="$5"

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

metadata_value() {
    local key="$1"
    local value

    value="$(awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) exit 1 }' "$metadata_path")" || {
        echo "archive metadata is missing key: $key" >&2
        exit 1
    }
    printf '%s\n' "$value"
}

[[ -f "$archive_path" ]] || { echo "missing archive: $archive_path" >&2; exit 1; }
[[ -f "$sha256_path" ]] || { echo "missing archive checksum: $sha256_path" >&2; exit 1; }
[[ -f "$metadata_path" ]] || { echo "missing archive metadata: $metadata_path" >&2; exit 1; }

archive_name="$(basename "$archive_path")"
sha256_name="$(basename "$sha256_path")"
metadata_name="$(basename "$metadata_path")"
archive_checksum="$(sha256_file "$archive_path")"
archive_bytes="$(file_size "$archive_path")"

grep -qx 'A3S GUI Dogfood Archive Metadata' "$metadata_path" || {
    echo "archive metadata has the wrong title" >&2
    exit 1
}
grep -qx 'format: key: value' "$metadata_path" || {
    echo "archive metadata format line is missing" >&2
    exit 1
}
grep -qx "platform: $platform" "$metadata_path" || {
    echo "archive metadata platform does not match $platform" >&2
    exit 1
}
grep -qx "artifact: $artifact_name" "$metadata_path" || {
    echo "archive metadata artifact does not match $artifact_name" >&2
    exit 1
}
grep -qx "archive: $archive_name" "$metadata_path" || {
    echo "archive metadata archive does not match $archive_name" >&2
    exit 1
}
grep -Eq '^sourceCommit: ([0-9a-f]{12,64}|unknown)$' "$metadata_path" || {
    echo "archive metadata source commit is missing or invalid" >&2
    exit 1
}
grep -Eq '^generatedAtUtc: [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$' "$metadata_path" || {
    echo "archive metadata timestamp is missing or invalid" >&2
    exit 1
}

metadata_checksum="$(metadata_value archiveSha256)"
metadata_bytes="$(metadata_value archiveBytes)"
[[ "$metadata_checksum" == "$archive_checksum" ]] || {
    echo "archive metadata checksum mismatch" >&2
    exit 1
}
[[ "$metadata_bytes" == "$archive_bytes" ]] || {
    echo "archive metadata byte count mismatch" >&2
    exit 1
}

sha256_line="$(tr -d '\r' < "$sha256_path")"
expected_sha256_line="$archive_checksum  $archive_name"
[[ "$sha256_line" == "$expected_sha256_line" ]] || {
    echo "archive checksum file does not match $archive_name" >&2
    exit 1
}

grep -qx -- "contents:" "$metadata_path" || {
    echo "archive metadata contents section is missing" >&2
    exit 1
}
grep -qx -- "- $archive_name" "$metadata_path" || {
    echo "archive metadata contents is missing $archive_name" >&2
    exit 1
}
grep -qx -- "- $sha256_name" "$metadata_path" || {
    echo "archive metadata contents is missing $sha256_name" >&2
    exit 1
}
grep -qx -- "- $metadata_name" "$metadata_path" || {
    echo "archive metadata contents is missing $metadata_name" >&2
    exit 1
}

echo "validated $metadata_path"
