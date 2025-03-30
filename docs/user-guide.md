# Wrale Agnostic Content Dependency Manager

## Installation Guide

### Prerequisites

- Rust 1.70 or higher
- Git 2.25 or higher (for sparse checkout features)

### Installation from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/wrale/wrale-acdm.git
   cd wrale-acdm
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   cargo install --path .
   ```

4. Verify the installation:
   ```bash
   acdm --version
   ```

## Usage Guide

### Initializing a Project

To start using `acdm` in your project, initialize a configuration file:

```bash
acdm init
```

This creates an `acdm.toml` file in your current directory. You can specify a default location for vendored content:

```bash
acdm init --location vendor/external
```

### Adding Dependencies

Add a dependency from an external Git repository:

```bash
acdm add git@github.com:example/repo.git --name example-dep --target vendor/example
```

Optional parameters:
- `--rev`: Specify a branch, tag, or commit (defaults to "main")

### Including Specific Paths

By default, the entire repository will be included. To select specific paths:

```bash
acdm include example-dep "docs/**" "src/lib/**"
```

The paths use glob patterns, supporting:
- `*`: Matches any sequence of non-separator characters
- `**`: Matches any sequence of characters including directory separators
- `?`: Matches any single non-separator character
- `[...]`: Matches any character in the brackets

### Updating Dependencies

Update all dependencies:

```bash
acdm update
```

Update specific dependencies:

```bash
acdm update dep1 dep2
```

Automatically commit changes:

```bash
acdm update --message "Update external dependencies"
```

## Configuration Reference

The `acdm.toml` file uses the following format:

```toml
# Default location for vendored content (optional)
location = "vendor/external"

# Define external dependencies
[[sources]]
# Repository URL (SSH or HTTPS)
repo = "git@github.com:example/repo.git"
# Name for the dependency
name = "example-dependency"
# Git revision (branch, tag, or commit)
rev = "main"
# Repository type (currently only 'git' is supported)
type = "git"
# Patterns for selective inclusion
sparse_paths = [
    "docs/specification/**",
    "schema/**"
]
# Target location in your project
target = "vendor/example"
```

## Best Practices

1. **Pin to specific commits**: Use commit hashes instead of branch names for stable dependencies.

2. **Organize vendored content**: Use a consistent directory structure for external content.

3. **Include only what you need**: Be specific with include patterns to minimize the footprint.

4. **Document dependencies**: Include a README explaining where vendored content comes from.

5. **Use atomic updates**: Update all dependencies at once to ensure consistency.

6. **Version lock**: Update dependencies deliberately, not automatically.

## Troubleshooting

### Common Issues

1. **Authentication Failures**:
   - For SSH: Ensure your SSH key is registered with the Git provider
   - For HTTPS: Check your credentials or tokens

2. **No files extracted**:
   - Verify your include patterns match files in the repository
   - Check that the revision exists in the remote repository

3. **Permission Denied**:
   - Ensure you have write permissions to the target directory

### Debugging

For detailed logging, set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug acdm update
```

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
