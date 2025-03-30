# wrale-acdm

Wrale Agnostic Content Dependency Manager

## Overview

`wrale-acdm` is an open-source tool designed to solve the problem of granular, selective content inclusion from external repositories without the complexity and limitations of Git submodules. Built in Rust with clean architecture principles, it enables teams to pull specific folders or files from external repositories while maintaining a clean project history and minimizing dependency footprint.

## Key Features

- **Selective Content Inclusion**: Pull only the specific folders or files you need from external repositories
- **Zero Submodule Footprint**: Avoids Git submodules entirely, leaving no metadata in your repository
- **Declarative Configuration**: TOML-based specification of dependencies and targets
- **Version Locking**: Pin dependencies to specific commits, branches, or tags
- **Clean Git History**: Changes to vendored content appear as normal changes in your repository
- **Multiple Protocol Support**: Clone via SSH and HTTPS with appropriate authentication
- **Git-Aware Operations**: Operates only within clean Git repositories
- **Safe Operations**: Verifies clean git status before making changes
- **Interactive Workflow**: Prompts before potentially destructive actions (can be bypassed with --force)
- **Verbose Logging**: Detailed logging for debugging (can be disabled with --quiet)

## Installation

```bash
# Not yet available on crates.io
cargo install --git https://github.com/wrale/wrale-acdm.git
```

or

```bash
git clone https://github.com/wrale/wrale-acdm.git
cd wrale-acdm
cargo install --path .
```

Alternatively, using the provided Makefile:

```bash
git clone https://github.com/wrale/wrale-acdm.git
cd wrale-acdm
make install
```

## Quick Start

Initialize a new configuration:

```bash
acdm init
```

Add an external dependency:

```bash
acdm add git@github.com:example/repo.git --name example-dep --rev main --target vendor/example
```

Define which files to include:

```bash
acdm include example-dep "docs/specification/**" "schema/**"
```

Update all dependencies:

```bash
acdm update
```

After running any command that modifies files, you'll need to review and commit the changes manually:

```bash
git add .
git commit -m "Update external dependencies"
```

Note: `acdm` requires a clean git repository to perform updates. It will not automatically commit changes.

## Global Flags

The following flags can be used with any command:

- `--quiet`: Suppress verbose logging, showing only warnings and errors
- `--force`: Skip confirmation prompts and proceed with potentially destructive operations
- `--config <path>`: Specify a custom path to the configuration file (default: `acdm.toml`)

## Configuration

`acdm.toml` example:

```toml
location = "vendor/external"

[[sources]]
repo = "git@github.com:example/repo.git"
name = "example-dependency"
rev = "main"
type = "git"
sparse_paths = [
    "docs/specification/**",
    "schema/**"
]
target = "vendor/example"
```

## Development

Requirements:
- Rust 1.70+
- Git 2.25+ (for sparse checkout features)

Building from source:

```bash
git clone https://github.com/wrale/wrale-acdm.git
cd wrale-acdm
make build
```

Running tests:

```bash
make test       # Run all tests
make unit       # Run unit tests only
make integration # Run integration tests only
```

Preparing for a commit:

```bash
make prepare    # Formats code, runs linter, checks, tests, and generates docs
```

See all available commands:

```bash
make help
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
