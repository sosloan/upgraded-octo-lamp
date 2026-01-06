#!/bin/bash
# Demo script to showcase BET Architecture features

echo "BET Architecture Demo"
echo "===================="
echo ""
echo "Building the project..."
cargo build --release

echo ""
echo "Running tests..."
cargo test

echo ""
echo "Starting BET Architecture System..."
echo ""
echo "The application features:"
echo "  ✓ Terminal GUI with ANSI colors"
echo "  ✓ Vim-style modal keyboard (Normal/Command/Insert modes)"
echo "  ✓ Menu navigation with j/k or arrow keys"
echo "  ✓ Search functionality with / key"
echo "  ✓ 6 major subsystems:"
echo "    - Trading System (Biotech, P&L, CURE Foundation)"
echo "    - Storm Topologies (Word Count, Sum, Edison⚡, Polymath🌐, Key Bounce, Randomize Keys🎹)"
echo "    - Monad λ System (Invariant Monads, Law Verification, Plumber)"
echo "    - A-DAG (OCTOTREÉ, Topological Sort, Critical Path)"
echo "    - SWIN Transformer (16 Heads, Grey Eyes, 600 Shades)"
echo "    - Elixir Check (Erlang/OTP Guarantees)"
echo ""
echo "To run interactively: cargo run --release"
echo "Controls: j/k (navigate) | / (search) | : (command) | q (quit)"
