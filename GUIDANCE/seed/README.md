# wrale-acdm
Wrale Agnostic Content Dependency Manager

## Overview

`wrale-acdm` (Wrale Agnostic Content Dependency Manager) is a proposed open-source tool designed to solve the problem of granular, selective content inclusion from external repositories without the complexity and limitations of Git submodules. Built in Rust with clean architecture principles, it enables teams to pull specific folders or files from external repositories while maintaining a clean project history and minimizing dependency footprint.

**Proposed License:** MIT
**Executable:** `acdm`
**Primary Platform:** macOS (with cross-platform compatibility in mind)
**Repository:** Not yet created

## Key Features

- **Selective Content Inclusion**: Pull only the specific folders or files you need from external repositories
- **Zero Submodule Footprint**: Avoids Git submodules entirely, leaving no metadata in your repository
- **Declarative Configuration**: TOML-based specification of dependencies and targets
- **Version Locking**: Pin dependencies to specific commits, branches, or tags
- **Clean Git History**: Changes to vendored content appear as normal changes in your repository
- **Multiple Protocol Support**: Clone via SSH and HTTPS with appropriate authentication
- **Git-Aware Operations**: Operates only within clean Git repositories
- **Domain-Driven Design**: Clean architecture with proper separation of concerns for extensibility

## Core Concepts

### Dependencies

External sources that provide content to be included in your project:
- **Repository URL**: Git repository location (SSH or HTTPS)
- **Name**: Identifier for the dependency
- **Revision**: Git reference (branch, tag, commit) to checkout
- **Sparse Paths**: Patterns to include only specific files or directories
- **Target Location**: Where the content should be placed in your project

### Configuration

A TOML file (e.g., `acdm.toml`) that defines all external dependencies:

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

TOML was chosen over YAML or JSON for several reasons:

1. **Better fit for Rust ecosystem**: TOML is the standard configuration format in the Rust ecosystem (used by Cargo)
2. **Less ambiguity**: Stricter parsing rules with fewer surprises compared to YAML's indentation-based structure
3. **Explicit structure**: Section headers and explicit arrays make the configuration more readable
4. **Comment support**: Unlike JSON, TOML allows comments for documenting configuration choices
5. **Simplified tooling**: TOML parsers tend to be simpler and less error-prone than YAML parsers

### Operation Model

Unlike Git submodules or tools that establish ongoing connections to external repositories, `wrale-acdm` follows a "fetch-and-forget" model:

1. Clone external repository to a temporary location
2. Extract only the specified content based on sparse paths
3. Clean the target mount point by removing all existing content (not using Git but native filesystem operations)
4. Copy new content to the clean target location in your project
5. Clean up temporary data

After these operations, the user needs to manually stage and commit the changes if they wish to persist them.

This approach ensures your repository maintains a clean history without extra metadata or hidden connections to external repositories. The complete cleaning of the mount point (step 3) is crucial for maintaining the integrity of the dependency directory, ensuring a clean slate for each update without relying on git-specific commands.

## Differentiation from Existing Tools

### Compared to Git Submodules

- No `.gitmodules` file or submodule metadata in your repository
- No need to learn complex submodule commands
- Ability to include only specific directories without sparse checkout complexity
- Changes to vendored content appear as normal changes in your repository

### Compared to Gitman

While [Gitman](../git/gitman.md) offers similar functionality for selective directory inclusion, `wrale-acdm` differs in several key ways:

- No Python dependency (built in native Rust)
- Truly zero submodule footprint, with content appearing as normal files
- More focused feature set without symlinks or post-checkout scripts
- Clean architecture design for extensibility
- Git-aware operations that respect repository state

## Development Rationale

### Problem Statement

Current dependency management approaches for selective content inclusion suffer from:

1. **Complexity**: Git submodules have a steep learning curve and create ongoing maintenance burden
2. **Overhead**: Full repository clones waste space when only specific content is needed
3. **History Pollution**: Many solutions leave metadata or special references in repository history
4. **Language Coupling**: Existing tools often depend on specific language ecosystems

### Solution Approach

`wrale-acdm` addresses these issues by:

1. Providing a simple, focused tool that does one thing well
2. Implementing a clean architecture that separates concerns
3. Treating content inclusion as a one-time operation rather than an ongoing relationship
4. Building in Rust for performance, safety, and cross-platform compatibility

## Implementation Strategy

### Architecture

The tool will follow clean architecture principles with clear separation of:

1. **Domain Layer**: Core business logic and entities
2. **Use Cases**: Application-specific business rules
3. **Interface Adapters**: Controllers, presenters, and gateways
4. **Frameworks & Drivers**: External frameworks and tools

### Key Components

- **CLI Interface**: Command parsing and user interaction using `clap`
- **Configuration Parser**: TOML parsing and validation
- **Repository Manager**: Handles Git operations through abstraction
- **Content Extractor**: Extracts specific content based on patterns
- **Directory Cleaner**: Platform-specific implementation for removing directory contents
- **File System Manager**: Handles file operations with proper error handling
- **Git Integration Layer**: Abstracts Git operations behind interfaces for testability

### Content Management Strategy

A key aspect of the implementation is how content changes are handled:

1. **Complete Mount Point Cleaning**: For each update, the entire target directory is completely cleared using OS-native operations
2. **Filesystem Abstraction**: Directory removal operations are dependency-inverted to handle cross-OS differences
3. **Clean Slate Approach**: Rather than tracking diffs, each update creates a pristine copy of the current dependency state
4. **Conflict Detection**: Pre-operation checks ensure the Git repository is in a clean state
5. **Safe Operations**: The tool operates only when Git status is clean, preventing accidental overwrites of uncommitted changes

Example of dependency inversion for directory cleaning:

```rust
trait DirectoryCleaner {
    fn clean_directory(&self, path: &Path) -> Result<(), Error>;
}

struct UnixDirectoryCleaner;
impl DirectoryCleaner for UnixDirectoryCleaner {
    fn clean_directory(&self, path: &Path) -> Result<(), Error> {
        // Unix-specific implementation
    }
}

struct WindowsDirectoryCleaner;
impl DirectoryCleaner for WindowsDirectoryCleaner {
    fn clean_directory(&self, path: &Path) -> Result<(), Error> {
        // Windows-specific implementation
    }
}

// Factory to create the appropriate cleaner based on the OS
fn create_directory_cleaner() -> Box<dyn DirectoryCleaner> {
    if cfg!(unix) {
        Box::new(UnixDirectoryCleaner)
    } else if cfg!(windows) {
        Box::new(WindowsDirectoryCleaner)
    } else {
        // Default implementation
        Box::new(DefaultDirectoryCleaner)
    }
}
```

### Dependency Inversion

External dependencies like Git will be accessed through abstractions:

```rust
trait RepositoryFetcher {
    fn fetch(&self, url: &str, revision: &str, temporary_path: &Path) -> Result<(), Error>;
}

struct GitRepositoryFetcher {
    // Implementation that shells out to git with proper error handling
}
```

This design enables:
- Mocking for testing
- Potential addition of non-Git sources in the future
- Clear separation between business logic and external tools

## Expected Benefits

### For Developers

1. **Simplified Workflow**: No need to learn complex Git submodule commands
2. **Reduced Cognitive Load**: Treat external content as normal files
3. **Improved Performance**: Fetch only what you need, reducing repository size
4. **Better History**: Changes to vendored content appear as normal changes
5. **Cross-project Consistency**: Standardized approach to content dependency management

### For Organizations

1. **Reduced Onboarding Friction**: New team members don't need to learn submodule management
2. **Improved Compliance**: Better visibility into external dependencies
3. **Storage Efficiency**: Minimize repository bloat by including only necessary content
4. **Workflow Standardization**: Consistent approach across projects

## Roadmap

### MVP (Minimum Viable Product)

1. Basic CLI with init, update, and status commands
2. SSH and HTTPS repository support
3. Sparse path selection
4. Version locking (commit pinning)
5. Single dependency updating
6. Complete content purging during updates

### Future Enhancements

1. Parallel dependency fetching
2. Dependency groups
3. Incremental updates
4. Non-Git sources (HTTP, S3, etc.)
5. Conflict resolution strategies
6. Dependency integrity validation

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
