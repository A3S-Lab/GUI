#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 ]]; then
    echo "usage: $0 <file>" >&2
    exit 2
fi

file="$1"
[[ -f "$file" ]] || { echo "missing file: $file" >&2; exit 1; }

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

basename="$(basename "$file")"
checksum="$(sha256_file "$file")"

printf '%s  %s\n' "$checksum" "$basename"
