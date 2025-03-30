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

2. Using Make (recommended):
   ```bash
   make install
   ```

3. Or manual installation:
   ```bash
   cargo build --release
   cargo install --path .
   ```

4. Verify the installation:
   ```bash
   acdm --version
   ```

## Usage Guide

### Global Flags

The following flags can be used with any command:

- `--quiet`: Suppress verbose logging, showing only warnings and errors
- `--force`: Skip confirmation prompts and proceed with potentially destructive operations
- `--config <path>`: Specify a custom path to the configuration file (default: `acdm.toml`)

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

Notes:
- This command will fail if your git repository has uncommitted changes
- Use `--force` to bypass git status checks (not recommended)

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

Notes:
- This command will fail if your git repository has uncommitted changes
- Use `--force` to bypass git status checks (not recommended)

### Updating Dependencies

Update all dependencies:

```bash
acdm update
```

Update specific dependencies:

```bash
acdm update dep1 dep2
```

After updating dependencies, you'll need to review and commit the changes manually:

```bash
git add .
git commit -m "Update external dependencies"
```

Notes:
- By default, you will be prompted to confirm before mount points are purged
- Use `--force` to skip the confirmation prompt
- This command will fail if your git repository has uncommitted changes

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

## Logging and Debugging

`acdm` provides detailed logging to help debug issues. By default, verbose output is enabled.

- Use `--quiet` to suppress verbose logging
- Alternatively, set the `RUST_LOG` environment variable to control the log level:

```bash
# Show only warnings and errors
RUST_LOG=warn acdm update

# Show detailed debug information
RUST_LOG=debug acdm update

# Show trace-level information (very verbose)
RUST_LOG=trace acdm update
```

## Safety Features

`acdm` implements several safety features to protect your repository:

1. **Git Repository Requirement**: Operations will only run within a Git repository, ensuring you can track changes.

2. **Git Status Check**: Operations will fail if your Git repository has uncommitted changes, ensuring a clean state before making modifications.

3. **Mount Point Confirmation**: Before purging mount points during updates, you'll be prompted to confirm the operation.

4. **Verbose Logging**: Detailed logs help troubleshoot issues and understand what's happening.

The mount point confirmation can be bypassed with the `--force` flag when necessary, but this should be used with caution.

Note: Unlike previous versions, `acdm` will not automatically stage or commit any changes. After running operations that modify files, you'll need to stage and commit the changes manually.

## Best Practices

1. **Pin to specific commits**: Use commit hashes instead of branch names for stable dependencies.

2. **Organize vendored content**: Use a consistent directory structure for external content.

3. **Include only what you need**: Be specific with include patterns to minimize the footprint.

4. **Document dependencies**: Include a README explaining where vendored content comes from.

5. **Keep a clean git repository**: Commit your changes before running `acdm` operations.

6. **Batch updates**: Update all dependencies at once to ensure consistency.

7. **Version lock**: Update dependencies deliberately, not automatically.

8. **Review and commit manually**: After updates, review all changes and commit them manually.

9. **Create atomic commits**: For cleaner history, commit dependency updates separately from your own code changes.

## Troubleshooting

### Common Issues

1. **Operation fails with "Git repository has uncommitted changes"**:
   - Commit or stash your changes before running `acdm` operations
   - Use `--force` to bypass this check (not recommended)

2. **Authentication Failures**:
   - For SSH: Ensure your SSH key is registered with the Git provider
   - For HTTPS: Check your credentials or tokens

3. **No files extracted**:
   - Verify your include patterns match files in the repository
   - Check that the revision exists in the remote repository
   - Use verbose logging to see what's happening

4. **Permission Denied**:
   - Ensure you have write permissions to the target directory

5. **Update Errors**:
   - Check if the target directory exists and has correct permissions
   - Verify that the source repository is accessible
   - Run with verbose logging to see detailed error information

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
