# Code Tree Generator

A Rust command-line tool that generates a directory tree structure and concatenates all code files into a single output file. This is particularly useful for sharing codebase context with Large Language Models (LLMs).

## Features

- 📁 Generates directory tree structure
- 📝 Concatenates code files with their paths
- 🔍 Supports multiple programming languages
- ⚡ Fast and efficient processing
- 🚫 Ignores common non-source directories (node_modules, .git, etc.)

## Installation

### Prerequisites

- Rust and Cargo (Install from [rustup.rs](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/code_tree.git
cd code_tree

# Build the project
cargo build --release

# Optional: Install globally
cargo install --path .

# Using cargo run
cargo run -- /path/to/directory

# If installed globally
code_tree /path/to/directory

# Analyze current directory
code_tree .


