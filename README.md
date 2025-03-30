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
- **Atomic Operations**: All content updates happen in a single atomic commit

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
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
