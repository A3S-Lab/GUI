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
[[ -f "$manifest_path" ]] || { echo "missing bundle manifest: $manifest_path" >&2; exit 1; }
grep -qx 'A3S GUI Dogfood Bundle Manifest' "$manifest_path" || { echo "bundle manifest has the wrong title" >&2; exit 1; }
grep -qx "platform: $platform" "$manifest_path" || { echo "bundle manifest platform does not match $platform" >&2; exit 1; }
grep -qx "artifact: $artifact_name" "$manifest_path" || { echo "bundle manifest artifact does not match $artifact_name" >&2; exit 1; }
grep -Eq '^sourceCommit: ([0-9a-f]{12,64}|unknown)$' "$manifest_path" || { echo "bundle manifest source commit is missing or invalid" >&2; exit 1; }
grep -Eq '^generatedAtUtc: [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$' "$manifest_path" || { echo "bundle manifest timestamp is missing or invalid" >&2; exit 1; }
grep -qx 'format: sha256  bytes  path' "$manifest_path" || { echo "bundle manifest format line is missing" >&2; exit 1; }

manifest_rel="$(relative_manifest_path)"
expected_paths="$(mktemp)"
actual_paths="$(mktemp)"
expected_sorted="$(mktemp)"
actual_sorted="$(mktemp)"
trap 'rm -f "$expected_paths" "$actual_paths" "$expected_sorted" "$actual_sorted"' EXIT

file_count=0
in_files=false
while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" == "files:" ]]; then
        in_files=true
        continue
    fi

    [[ "$in_files" == true ]] || continue
    [[ -n "$line" ]] || continue

    if [[ ! "$line" =~ ^([0-9a-f]{64})[[:space:]][[:space:]]([0-9]+)[[:space:]][[:space:]](.+)$ ]]; then
        echo "invalid bundle manifest file entry: $line" >&2
        exit 1
    fi

    checksum="${BASH_REMATCH[1]}"
    bytes="${BASH_REMATCH[2]}"
    rel="${BASH_REMATCH[3]}"

    if [[ "$rel" == /* || "$rel" == *"/../"* || "$rel" == "../"* || "$rel" == *"/.." ]]; then
        echo "unsafe bundle manifest path: $rel" >&2
        exit 1
    fi

    file="$bundle_dir/$rel"
    [[ -f "$file" ]] || { echo "bundle manifest references missing file: $rel" >&2; exit 1; }

    actual_checksum="$(sha256_file "$file")"
    actual_bytes="$(file_size "$file")"
    [[ "$actual_checksum" == "$checksum" ]] || { echo "bundle manifest checksum mismatch for $rel" >&2; exit 1; }
    [[ "$actual_bytes" == "$bytes" ]] || { echo "bundle manifest byte count mismatch for $rel" >&2; exit 1; }

    printf '%s\n' "$rel" >> "$expected_paths"
    file_count=$((file_count + 1))
done < "$manifest_path"

[[ "$file_count" -gt 0 ]] || { echo "bundle manifest does not list any files" >&2; exit 1; }

(
    cd "$bundle_dir"
    find . -type f -print | sed 's#^\./##' | awk -v manifest="$manifest_rel" '$0 != manifest' | LC_ALL=C sort
) > "$actual_paths"

LC_ALL=C sort "$expected_paths" > "$expected_sorted"
LC_ALL=C sort "$actual_paths" > "$actual_sorted"
diff -u "$expected_sorted" "$actual_sorted" >/dev/null || {
    echo "bundle manifest file list does not match staged bundle contents" >&2
    diff -u "$expected_sorted" "$actual_sorted" >&2 || true
    exit 1
}

echo "validated $manifest_path"
