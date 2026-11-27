#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
DESTDIR="${DESTDIR:-}"
PRIVILEGE="${PRIVILEGE:-auto}"

cd "$(dirname "$0")"

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo is required." >&2
    exit 1
fi

case "$PRIVILEGE" in
    auto|none|sudo|doas) ;;
    *)
        echo "Error: invalid privilege command: $PRIVILEGE" >&2
        exit 1
        ;;
esac

TARGET_DIR="${CARGO_TARGET_DIR:-target}"
BINARY="$TARGET_DIR/release/hika"
DESTINATION="$DESTDIR$PREFIX/bin/hika"

echo ">> Building hika..."
cargo build --release --locked --target-dir "$TARGET_DIR"

if [[ ! -f "$BINARY" ]]; then
    echo "Error: executable not found: $BINARY" >&2
    exit 1
fi

# Staged installations should not require elevated privileges.
if [[ "$PRIVILEGE" == "auto" ]]; then
    if [[ $EUID -eq 0 || -n "$DESTDIR" || "$PREFIX" == "$HOME" || "$PREFIX" == "$HOME/"* ]]; then
        PRIVILEGE="none"
    elif command -v sudo >/dev/null 2>&1; then
        PRIVILEGE="sudo"
    elif command -v doas >/dev/null 2>&1; then
        PRIVILEGE="doas"
    else
        echo "Error: neither sudo nor doas is available." >&2
        exit 1
    fi
fi

echo ">> Installing hika..."

case "$PRIVILEGE" in
    none)
        install -Dm755 "$BINARY" "$DESTINATION"
        ;;
    sudo|doas)
        if ! command -v "$PRIVILEGE" >/dev/null 2>&1; then
            echo "Error: $PRIVILEGE is not installed." >&2
            exit 1
        fi

        "$PRIVILEGE" install -Dm755 "$BINARY" "$DESTINATION"
        ;;
esac

echo ">> Installed to $DESTINATION"
