# BET Architecture System

A comprehensive Rust-based system implementing trading, analytics, machine learning, and distributed processing components.

## Architecture Overview

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                         BET ARCHITECTURE MAP                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                           MAIN.RS (Entry Point)                         │  ║
║  │  • Terminal GUI with ANSI escape codes                                  │  ║
║  │  • Modal keyboard system (Vim-style)                                    │  ║
║  │  • Menu navigation with search/filter                                   │  ║
║  │  • Screen reader accessibility                                          │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Components

### 1. Trading System
- **Biotech Symbols**: Pre-configured universe of biotech stocks (BIIB, GILD, VRTX, REGN, AMGN)
- **Market Data**: Real-time quote management and OHLCV data structures
- **P&L Calculation**: Realized and unrealized profit/loss tracking
- **CURE Foundation**: Coalition for Unified Research and Education in Biotech

### 2. Storm Topologies
Distributed stream processing with multiple bolt types:
- **Word Count**: Text tokenization and frequency analysis
- **Sum**: Numerical aggregation
- **Edison ⚡**: Electric/energy processing (voltage × current = power)
- **Polymath 🌐**: Multi-domain knowledge processing
- **Key Bounce**: Keyboard debouncing and event filtering
- **Randomize Keys 🎹**: Random key generation (88 piano keys)

### 3. Monad λ System
Functional programming foundation:
- **Invariant Monads**: Type-safe computation chaining
- **Law Verification**: Left identity, right identity, and associativity
- **Plumber**: Utility for composing monadic operations

### 4. A-DAG (Acyclic Directed Acyclic Graph)
Workflow orchestration:
- **OCTOTREÉ**: Tree-based task structure
- **Topological Sort**: Kahn's algorithm for dependency resolution
- **Critical Path**: Identify bottlenecks in workflows
- **Trading Workflow**: Pre-configured trading pipeline

### 5. SWIN Transformer
Machine learning transformer architecture:
- **16 Attention Heads**: Multi-head self-attention mechanism
- **Grey Eyes**: Greyscale image processing
- **600 Shades**: High-resolution color/intensity mapping
- **Forward Pass**: End-to-end inference pipeline

### 6. Elixir Check
Runtime verification:
- **Erlang/OTP Detection**: Check for BEAM VM availability
- **Elixir Runtime**: Functional programming environment verification
- **Guarantees**: Fault tolerance, hot code reloading, immutability

## Building and Running

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

### Test
```bash
cargo test
```

## Terminal UI Usage

### Keyboard Controls

#### Normal Mode (default)
- `j` or `↓`: Navigate down in menu
- `k` or `↑`: Navigate up in menu
- `Enter`: Select menu item (view details)
- `/`: Enter search mode
- `:`: Enter command mode
- `q`: Quit application
- `Ctrl+C`: Force quit

#### Insert Mode (Search)
- Type to filter menu items
- `Backspace`: Delete character
- `Esc`: Return to normal mode
- `Enter`: Apply search and return to normal mode

#### Command Mode
- `q`: Quit
- `Esc`: Return to normal mode

### Menu Items

1. **Trading System**: View biotech symbols, portfolio, and CURE Foundation projects
2. **Storm Topologies**: Access distributed stream processing components
3. **Monad λ System**: Functional programming utilities and law verification
4. **A-DAG**: Task DAG visualization and workflow execution order
5. **SWIN Transformer**: Transformer architecture details
6. **Elixir Check**: Runtime verification status
7. **Quit**: Exit application

## Module Dependency Graph

```
adag ─────────────────┐
trading_models ───────┼──┐
market_data ──────────┼──┼──┐
momentum ─────────────┼──┼──┼──┐
signals ──────────────┼──┼──┼──┼──► trading_system ──► main.rs
trading ──────────────┼──┼──┼──┘         │
pnl ──────────────────┼──┼──┘            │
cure_foundation ──────┼──┘               │
capital_flow ─────────┘                  │
trading_dag ─────────────────────────────┘

monad_lambda ────────────────────────────────────► main.rs
storm ───────────────────────────────────────────► main.rs
swin_transformer ────────────────────────────────► main.rs
elixir_check ────────────────────────────────────► main.rs
```

## Accessibility

The terminal UI is designed with screen reader accessibility in mind:
- Clear visual hierarchy with ASCII art borders
- Numbered and labeled menu items
- Status indicators (mode, selection)
- Keyboard-only navigation
- High-contrast color scheme

## Development

### Project Structure
```
.
├── Cargo.toml              # Project manifest
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Binary entry point (Terminal GUI)
│   ├── adag.rs             # A-DAG implementation
│   ├── capital_flow.rs     # Capital flow analysis
│   ├── cure_foundation.rs  # CURE Foundation
│   ├── elixir_check.rs     # Elixir/Erlang runtime check
│   ├── market_data.rs      # Market data structures
│   ├── momentum.rs         # Technical indicators
│   ├── monad_lambda.rs     # Monad system
│   ├── pnl.rs              # P&L calculation
│   ├── signals.rs          # Trading signals
│   ├── storm.rs            # Storm topologies
│   ├── swin_transformer.rs # SWIN Transformer
│   ├── trading.rs          # Order execution
│   ├── trading_dag.rs      # Trading workflow DAG
│   ├── trading_models.rs   # Core trading models
│   └── trading_system.rs   # Unified trading interface
└── README.md               # This file
```

## License

See repository license.

## Contributing

This is a demonstration project showcasing modern Rust development practices, including:
- Terminal UI with crossterm
- Functional programming patterns
- Stream processing concepts
- Machine learning architectures
- Financial trading systems
