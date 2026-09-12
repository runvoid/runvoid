# RFC 0001: Project Manifests and Project Structure

- Feature Name: `project_manifests_and_modules`
- Start Date: 2026-09-12
- Status: Implemented
- RFC PR: Integrated in Runvoid 1.3.0

## Summary

This RFC establishes a standard project layout and declarative manifest (`runvoid.toml`) for Runvoid projects. It provides first-class CLI commands (`runvoid init`, `runvoid run`, `runvoid build`, `runvoid clean`) that detect project manifests and streamline compilation and testing without requiring manual file path arguments.

## Motivation

Earlier versions of Runvoid operated strictly on single source files (e.g., `runvoid run main.rv`). As codebases grow:
1. Developers need structured multi-file projects with clear source, test, and binary separation.
2. Build commands should be ergonomic (`runvoid run` should execute the project root without specifying paths).
3. Metadata such as application name, target ABI, and optimization levels should reside in configuration rather than CLI flags.
4. Clean artifacts removal should be automated.

## Guide-level Explanation

### Initializing a Project

To create a new Runvoid application:

```bash
runvoid init my_project
cd my_project
```

Or initialize an existing directory:

```bash
runvoid init
```

This creates:
```text
my_project/
├── runvoid.toml        # Declarative manifest
├── src/
│   └── main.rv         # Application entry point
├── tests/
│   └── main_test.rv    # Verification test suite
├── .gitignore          # Ignores build artifacts and binaries
└── README.md           # Project documentation
```

### Manifest Format (`runvoid.toml`)

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2026"
authors = ["Runvoid Developer <developer@runvoid.org>"]
description = "A standard Runvoid project"

[build]
entry = "src/main.rv"
output_dir = "bin"
target = "x86_64-linux"
opt_level = 2

[dependencies]
```

### Running and Building

When invoked within a directory containing `runvoid.toml`:
- `runvoid run`: Automatically parses `runvoid.toml`, locates the entry point (`src/main.rv`), compiles to memory or temporary executable, and executes.
- `runvoid build`: Compiles `src/main.rv` and outputs the executable to `bin/my_app` (or `bin/my_app.exe` on Windows).
- `runvoid clean`: Removes `bin/`, object files (`*.o`), generated assembly (`*.asm`), and build logs.
- `runvoid test`: Executes tests within `tests/` and `src/`.

## Reference-level Explanation

### Manifest Resolution Order
When running `runvoid run` or `runvoid build` without explicit arguments:
1. Check for `./runvoid.toml` in the current working directory.
2. If found, parse `[build].entry`. If `entry` exists, compile that target.
3. If no manifest is found, check for `./src/main.rv`.
4. If `./src/main.rv` does not exist, check for `./main.rv`.
5. If none exist, output an error message prompting the user to provide a source file or initialize a project via `runvoid init`.

### Manifest Parsing
The manifest is parsed using standard TOML specification. Unknown sections are preserved for future module and dependency management expansion.

## Alternatives Considered

1. **JSON Manifests (`runvoid.json`)**: Less human-readable, lacks inline comments.
2. **YAML Manifests**: Whitespace sensitivity can lead to subtle configuration errors.
3. **No Manifests (CLI-only flags)**: Leads to brittle build scripts and poor developer ergonomics across platforms.

## Future Possibilities

- Support for external Git and registry package dependencies in `[dependencies]`.
- Multi-target workspace definitions (`[workspace]` with multiple member packages).
- Dynamic build scripts (`build.rv`).
