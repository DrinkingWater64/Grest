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
```
## Usage
```bash
# Run the CLI tool with optional arguments:
code_tree --[OPTIONS]
### Options
| Option              | Short | Default Value | Description |
|---------------------|-------|--------------|-------------|
| `--root-path`      | `-r`  | `.`          | Root directory to analyze |
| `--output`         | `-o`  | `code_output.txt` | Output file path |
| `--ignored-dirs`   | `-i`  | `.git,node_modules,target,.idea,venv,bin,obj,Debug,Release` | Comma-separated directories to ignore |
| `--extensions`     | `-e`  | `rs,js,py,cpp,c,java,go,ts,cs,csproj,sln,cshtml,razor,json,xml,config,yml,yaml` | File extensions to include |
| `--verbose`        | `-v`  | `false`      | Enable verbose output |

### Example Usage
Scan the current directory and save output to `result.txt`:
```sh
./cli_tool -o result.txt
```

Scan a specific directory while ignoring additional folders:
```sh
./cli_tool -r /path/to/project -i .git,node_modules,dist
```

The result will be stored in "Code_output.txt" in root project.

## Contributing
Feel free to open issues and submit pull requests!

## License
This project is licensed under the MIT License.