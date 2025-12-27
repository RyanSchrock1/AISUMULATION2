# AI Simulation

A simulation of AI entities with different behaviors and interactions, built with the Bevy game engine.

## Features

- Multiple AI types with distinct behaviors (Aggressor, Defender, Ethical, etc.)
- Complex interaction system for AI-to-AI interactions
- Detailed simulation statistics and visualization
- Pausable and configurable simulation
- Modular architecture for easy extension

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager, installed with Rust)
- Git (for version control)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ai-simulation.git
   cd ai-simulation
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Running the Simulation

To run the simulation in debug mode:
```bash
cargo run
```

For better performance, run in release mode:
```bash
cargo run --release
```

## Controls

- **Space**: Pause/resume the simulation
- **R**: Reset the simulation
- **Q**: Quit the application

## Project Structure

- `src/main.rs`: Entry point and UI setup
- `src/lib.rs`: Library crate with public API
- `src/ai.rs`: AI types and behavior logic
- `src/common.rs`: Common components and data structures
- `src/simulation.rs`: Core simulation logic
- `src/interaction.rs`: AI interaction systems

## Configuration

You can configure various aspects of the simulation by modifying the constants in `src/lib.rs`:

```rust
// Maximum number of simulation cycles
pub const MAX_CYCLES: u64 = 1_000_000;

// Threshold for monoculture dominance (0.0 to 1.0)
pub const MONOCULTURE_DOMINANCE_THRESHOLD: f64 = 0.9;

// Minimum number of AIs required for monoculture check
pub const MONOCULTURE_MIN_COUNT: usize = 10;

// How often to log simulation stats (in cycles)
pub const LOG_INTERVAL: u64 = 10_000;
```

## License

This project is licensed under either of:

 * MIT license (see [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0 (see [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
