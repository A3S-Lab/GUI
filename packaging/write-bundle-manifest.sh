#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 4 ]]; then
    echo "usage: $0 <bundle-dir> <manifest-path> <platform> <artifact-name>" >&2
    exit 2
fi

bundle_dir="$1"
manifest_path="$2"
platform="$3"
artifact_name="$4"

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

relative_manifest_path() {
    case "$manifest_path" in
        "$bundle_dir"/*)
            printf '%s\n' "${manifest_path#"$bundle_dir"/}"
            ;;
        *)
            echo "manifest path must be inside bundle dir: $manifest_path" >&2
            exit 1
            ;;
    esac
}

[[ -d "$bundle_dir" ]] || { echo "missing bundle directory: $bundle_dir" >&2; exit 1; }
mkdir -p "$(dirname "$manifest_path")"

manifest_rel="$(relative_manifest_path)"
commit="$(git rev-parse --short=12 HEAD 2>/dev/null || printf 'unknown')"
tmp_manifest="$(mktemp)"

{
    echo "A3S GUI Dogfood Bundle Manifest"
    echo "platform: $platform"
    echo "artifact: $artifact_name"
    echo "sourceCommit: $commit"
    echo "generatedAtUtc: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    echo "format: sha256  bytes  path"
    echo
    echo "files:"
} > "$tmp_manifest"

while IFS= read -r rel; do
    [[ -n "$rel" ]] || continue
    [[ "$rel" == "$manifest_rel" ]] && continue

    file="$bundle_dir/$rel"
    checksum="$(sha256_file "$file")"
    bytes="$(file_size "$file")"
    printf '%s  %s  %s\n' "$checksum" "$bytes" "$rel" >> "$tmp_manifest"
done < <(
    cd "$bundle_dir"
    find . -type f -print | sed 's#^\./##' | LC_ALL=C sort
)

mv "$tmp_manifest" "$manifest_path"
echo "wrote $manifest_path"
