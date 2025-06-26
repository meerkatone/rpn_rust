# HP-16C RPN Calculator Emulator

A faithful emulator of the classic HP-16C programmable calculator written in Rust. This emulator recreates the RPN (Reverse Polish Notation) functionality and computer science features of the original HP-16C calculator.

## Features

- **Complete RPN Stack**: Standard X, Y, Z, T register implementation
- **Multiple Number Bases**: Binary (2), Octal (8), Decimal (10), and Hexadecimal (16)
- **Configurable Word Size**: 1-128 bits for precise bit manipulation
- **Full Arithmetic Operations**: Addition, subtraction, multiplication, division
- **Bitwise Operations**: AND, OR, XOR, NOT, bit shifts
- **Memory Registers**: 16 storage registers (STO/RCL 0-15)
- **Interactive CLI**: Command-line interface with tab completion and history

## Installation

Make sure you have Rust installed, then clone and build:

```bash
git clone https://github.com/meerkatone/rpn_rust
cd rpn_rust
cargo build --release
```

## Usage

Start the calculator:

```bash
cargo run
```

### Basic Operations

The calculator uses standard RPN notation:

```
HP-16C> 10 ENTER 5 +
     15

HP-16C> 100 ENTER 20 -
     80

HP-16C> FF ENTER AA &
     AA
```

### Number Base Switching

```
HP-16C> DEC          # Switch to decimal mode
HP-16C> 255
HP-16C> HEX          # Switch to hexadecimal mode
     FF

HP-16C> BIN          # Switch to binary mode
     11111111
```

### Memory Operations

```
HP-16C> 42 STO 5     # Store 42 in register 5
HP-16C> 0 RCL 5      # Recall from register 5
     42
```

### Word Size Configuration

```
HP-16C> WS 8         # Set word size to 8 bits
HP-16C> 300          # Enter 300 (will be masked to 8 bits)
     44              # Result: 300 & 0xFF = 44
```

### Available Commands

- **Numbers**: Enter values in current base
- **RPN Stack**: `ENTER`, `DROP`, `SWAP`, roll operations
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Bitwise**: `&` (AND), `|` (OR), `^` (XOR), `~` (NOT)
- **Memory**: `STO n`, `RCL n` (n = 0-15)
- **Base Control**: `HEX`, `DEC`, `OCT`, `BIN`
- **Configuration**: `WS n` (word size)
- **Help**: `HELP`, `H`, or `?`
- **Exit**: `QUIT`, `Q`, or `EXIT`

## Development

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run specific test
cargo test test_basic_arithmetic

# Format code
cargo fmt

# Run linter (if available)
cargo clippy
```

### Architecture

The project consists of three main modules:

- **`src/rom.rs`**: Handles loading and parsing of HP-16C ROM data
- **`src/cpu.rs`**: Core calculator engine with RPN stack implementation
- **`src/main.rs`**: Interactive command-line interface with tab completion

## Dependencies

- `rustyline`: Provides readline functionality for the interactive CLI

## License

This project is for educational and hobbyist purposes. The HP-16C is a trademark of Hewlett-Packard.
