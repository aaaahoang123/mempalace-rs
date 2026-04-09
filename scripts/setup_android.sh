#!/bin/bash
set -e

echo "🚀 Setting up MemPalace-RS for Android/Termux..."

# 1. Install system dependencies
echo "📦 Installing rust, onnxruntime, libsqlite3, and pkg-config..."
pkg install -y rust onnxruntime libsqlite3 pkg-config

# 2. Ensure ort-sys is in the cargo registry
echo "📥 Fetching dependencies..."
cargo fetch

# 3. Locate and patch ort-sys
# We patch it to recognize Android as a valid target for cache_dir
ORT_SYS_DIR=$(find ~/.cargo/registry/src -name "ort-sys-2.0.0-rc.9" -type d | head -n 1)

if [ -n "$ORT_SYS_DIR" ]; then
    echo "🔧 Patching ort-sys at $ORT_SYS_DIR..."
    # Update cache_dir to support android
    sed -i 's/\[cfg(target_os = "linux")\]/\[cfg(any(target_os = "linux", target_os = "android"))\]/g' "$ORT_SYS_DIR/src/internal/dirs.rs"
    echo "✅ Patch applied successfully."
else
    echo "❌ Failed to find ort-sys-2.0.0-rc.9 in registry. Run 'cargo build' once to download."
    exit 1
fi

echo ""
echo "✅ Android setup complete!"
echo "💡 To build, use:"
echo "   ORT_LIB_LOCATION=/data/data/com.termux/files/usr/lib cargo build --release"
