# committer-rs 📝

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![ci](https://github.com/koenverburg/committer/actions/workflows/ci.yml/badge.svg)](https://github.com/koenverburg/committer/actions/workflows/ci.yml)

A Rust CLI tool that helps you create consistent and standardized commit messages with interactive prompts. Maintain clean and well-structured Git history with minimal effort.

## Features ✨

- 🔍 **Interactive Prompts**: Guided process to create structured commits
- 🏷️ **Standardized Format**: Enforces conventional commit standards with emojis
- 🎭 **Commit Presets**: Quick commits with pre-defined templates
- 🛠️ **Command-line Options**: Override defaults with various flags
- 🚀 **Auto-Push Support**: Optionally push after committing
- 📋 **Rich Emoji Selection**: Visual categorization of commit types

## Installation 📦

### From crates.io

```bash
cargo install committer
```

### From Source

```bash
# Clone the repository
git clone https://github.com/koenverburg/committer.git
cd committer-rs

# Build and install
cargo install --path .
```

## Usage 🚀

### Basic Usage

```bash
# Start the interactive commit process
committer commit
```

### Using Presets

```bash
# Use formatting preset (minimal input required)
committer commit --preset formatting

# Use demo preset
committer commit --preset demo
```

### Command-line Options

```bash
# Specify ticket number directly
committer commit --ticket ABC-123

# Skip description editor
committer commit --no-description

# Commit and automatically push
committer commit --push

# Skip pre-commit hooks
committer commit --no-verify

# Force push after commit
committer commit --push --force-push

# Combine options
committer commit --ticket DEF-456 --preset formatting --push
```

## Commit Message Format 📋

The tool generates commit messages in the following format:

```
[TICKET] EMOJI TYPE: SUBJECT (MESSAGE)

DESCRIPTION
```

Example:
```
[ABC-123] ✨ feat: user authentication (add login system)

Implemented user authentication with OAuth2.
Added login and registration pages.
Set up session management.
```

## Scope Types with Emojis 🎨

| Type | Emoji | Description |
|------|-------|-------------|
| feat | ✨ | New features |
| fix | 🐛 | Bug fixes |
| docs | 📚 | Documentation changes |
| style | 💎 | Code style/formatting |
| refactor | ♻️ | Code refactoring |
| perf | 🚀 | Performance improvements |
| test | 🧪 | Adding or fixing tests |
| build | 🔨 | Build system changes |
| ci | 👷 | CI/CD changes |
| chore | 🧹 | General maintenance |
| revert | ⏪ | Reverting changes |
| package | 📦 | Package updates |
| draft | 📝 | Work in progress |
| crash | 💥 | Fixing crashes |
| caution | ⚠️ | Risky changes |
| danger | 🔥 | High-impact changes |
| hazard | ☢️ | Breaking changes |
| config | ⚙️ | Configuration changes |
| hack | 🔧 | Temporary fixes |
| bug | 🐞 | Identified bugs |
| fix | 🩹 | Simple fixes |
| wip | 🚧 | Work in progress |
| trash | 🗑️ | Code removal |
| deleting | 🧨 | Significant removal |
| removal | 🔥 | Feature removal |

## Contributing 🤝

Contributions are welcome! Here's how you can help improve committer:

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes and commit them using committer itself**:
   ```bash
   cargo run -- commit
   ```
4. **Push to your branch**:
   ```bash
   git push origin feature/amazing-feature
   ```
5. **Open a Pull Request**

### Development Setup

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with specific feature
cargo run -- commit --preset formatting
```

## Code of Conduct 📜

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Inspired by conventional commit standards
- Thanks to all contributors

---

Created with ❤️ by [Koen Verburg](https://github.com/koenverburg)
