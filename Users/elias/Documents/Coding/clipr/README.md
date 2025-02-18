# clipr

A simple and efficient clipboard utility for the command line.

<img src="assets/demo.gif" alt="clipr demo" width="600"/>

## Features

- 📋 Copy text from files to clipboard
- 🔄 Pipe command outputs directly to clipboard
- ➕ Append mode for adding to existing content
- 🔍 View current clipboard content
- 🗑️ Clear clipboard with a single command
- 📝 Verbose output option for detailed feedback

## Installation

### Using Homebrew
```bash
brew install eliasnau/clipr/clipr
```

### Manual Installation
```bash
# Using cargo
cargo install --path .

# Or download the binary from releases
chmod +x ./clipr
sudo mv ./clipr /usr/local/bin/
```

## Usage

### Basic Commands
```bash
# Copy file contents to clipboard
clipr -f config.json

# Copy command output to clipboard
ls -la | clipr

# Append to existing clipboard content
echo "more text" | clipr --append

# Show current clipboard content
clipr show

# Clear clipboard
clipr clear

# Show verbose output
clipr -v
```

### Options
- `-f, --file <FILE>` - Read from file
- `-a, --append` - Append to existing clipboard content
- `-v, --verbose` - Show verbose output
- `--help` - Show help message
- `--version` - Show version information

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.