# nlsh

Natural language shell command generator.

## Quick Start

### Installation

Download the installer or package for your OS:
- **Arch Linux**: Download `nlsh.pkg.tar.zst` and install with `sudo pacman -U nlsh.pkg.tar.zst`
- **Debian/Ubuntu**: Download `nlsh.deb` and install with `sudo dpkg -i nlsh.deb`
- **Fedora/RHEL**: Download `nlsh.rpm` and install with `sudo rpm -i nlsh.rpm`
- **macOS (Homebrew)**: Use the Homebrew formula generated in the release (or download the binary)
- **Windows**: Use the `.zip` containing the binary and powershell install script

Or,
Download the latest binary for your OS from the [Releases](https://github.com/zpeteman/nlsh/releases) page.

Alternatively, build from source:
```bash
cargo install --path .
```

### Setup

```bash
# Set your API key
nlsh config set-key anthropic

# Configure your shell
# For bash, add this to your ~/.bashrc
source /path/to/shell/bash.sh

# For zsh, add this to your ~/.zshrc
source /path/to/shell/zsh.sh
```

## Status
Tasks completed:
1. Hidden input for `config set-key`
2. Default sane config and error messages
3. Command-safety linting
4. `--explain` / `-e` flag on `gen`
5. Wire up `delete_key` to a CLI command
6. Shell integration test harness
7. Packaging (GitHub Actions release workflow)
8. Task 8 (`--dry-run` mode for regenerate flow polish) is deferred for now as an optional nice-to-have.
