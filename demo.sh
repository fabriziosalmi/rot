#!/bin/bash
# LiveScope Demo Script - Shows off different themes

echo "🎨 LiveScope Demo - Real-time System Performance Art"
echo "====================================================="
echo ""

echo "🔥 Starting Fire Theme (10 seconds)..."
timeout 10s cargo run --release -- --theme fire --refresh 16 || true

clear
echo "🌊 Starting Ocean Theme (10 seconds)..."
timeout 10s cargo run --release -- --theme ocean --refresh 16 || true

clear
echo "🟢 Starting Matrix Theme (10 seconds)..."
timeout 10s cargo run --release -- --theme matrix --refresh 16 || true

clear
echo "🌈 Starting Rainbow Theme (10 seconds)..."
timeout 10s cargo run --release -- --theme rainbow --refresh 16 || true

clear
echo ""
echo "✨ LiveScope Demo Complete!"
echo "Run 'cargo run --release' to start your own session!"
echo "Press 'q' to quit when running LiveScope"
