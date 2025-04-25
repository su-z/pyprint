# pyprint

[![Crates.io](https://img.shields.io/crates/v/pyprint.svg)](https://crates.io/crates/pyprint)
[![Documentation](https://docs.rs/pyprint/badge.svg)](https://docs.rs/pyprint)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Getting tired of writing printing statements with format strings in Rust? This is a library to enable Python-style printing in Rust. It is implemented using Rust macros. Anything with the `Display` trait implemented can be printed.

## Features

- **Python-style print syntax** - Simple, intuitive printing without format strings
- **Customizable separators and endings** - Control how items are separated and how output ends
- **Debug printing** - Easy printing of complex data structures that implement `Debug`
- **Error printing** - Redirect output to stderr when needed
- **File redirection** - Write output to files or other destinations
- **Flush control** - Control output buffer flushing

## Installation

Install with:

```bash
cargo add pyprint
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
pyprint = "1.0.1"
```

## Usage

Simply write like Python in Rust:

```rust
use pyprint::pprn;

// Basic printing
let a = 5;
pprn!("Progress:", a, sep=" ", end="\r");  // Prints: Progress: 5 (with carriage return)

// Multiple values
pprn!("Hello", "World", 42);  // Prints: Hello World 42

// Custom separator
pprn!("a", "b", "c", sep=", ");  // Prints: a, b, c

// Debug printing for complex types
use pyprint::dprn;
let data = vec![1, 2, 3];
dprn!(data);  // Prints: [1, 2, 3]
```

### Printing to stderr

```rust
use pyprint::eprn;
use pyprint::deprn;

// Regular error printing
eprn!("Error:", "File not found");  // Prints to stderr

// Debug error printing
deprn!(std::io::Error::last_os_error());  // Prints debug representation to stderr
```

### File Output

```rust
use pyprint::pprint;
use std::fs::File;

// Print to a file (returns Result)
let file = File::create("output.txt").unwrap();
pprint!(file=file, "This goes to a file");
```

## Available Macros

| Macro        | Description                                 |
| ------------ | ------------------------------------------- |
| `pprint!`  | Basic print, returns Result                 |
| `pprn!`    | Basic print, unwraps Result                 |
| `dprint!`  | Debug print, returns Result                 |
| `dprn!`    | Debug print, unwraps Result                 |
| `eprint!`  | Error print to stderr, returns Result       |
| `eprn!`    | Error print to stderr, unwraps Result       |
| `deprint!` | Debug error print to stderr, returns Result |
| `deprn!`   | Debug error print to stderr, unwraps Result |

## Options

All macros support these options:

- `sep=VALUE`: Set separator between items (default: space)
- `end=VALUE`: Set ending string (default: newline)
- `file=VALUE`: Set output destination (default: stdout or stderr)
- `flush=BOOL`: Control immediate flushing (default: false). Note that when printing to the terminal, upon entering a new line, often flush will happen anyway.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
