# 22. Package Architecture, Project Manifests & Production CI/CD

As software projects expand beyond individual prototype scripts, engineering teams require rigorous modular structures: package manifests, reproducible builds, semantic versioning, automated continuous integration (CI/CD), and automated documentation generation.

In this chapter, you will master the Runvoid production development workflow:
1. The anatomy of `runvoid.toml` project manifests.
2. Managing multi-module directory structures and internal imports.
3. Automated testing workflows with `runvoid test`.
4. Continuous Integration pipelines with GitHub Actions.
5. Packaging standalone desktop installers and cross-platform distribution releases.

---

## 1. Project Anatomy & `runvoid.toml`

A production Runvoid repository follows a clean, standardized layout:

```
my_enterprise_app/
├── runvoid.toml            # Project manifest & metadata
├── README.md               # Documentation & quickstart
├── LICENSE                 # Software license (MIT / Apache-2.0)
├── src/
│   ├── main.rv             # Main application entrypoint
│   ├── models/
│   │   └── user.rv         # Domain models
│   └── network/
│       └── server.rv       # Socket & HTTP routing handlers
├── tests/
│   ├── test_models.rv      # Unit test suites
│   └── test_server.rv      # Integration test suites
├── assets/
│   ├── icon.png            # Window & installer icon
│   └── sound.wav           # Audio assets
└── target/                 # Build artifacts & compiled binaries (gitignored)
```

### The `runvoid.toml` Specification

```toml
[package]
name = "enterprise_monitor"
version = "1.3.0"
authors = ["Alex Developer <alex@example.com>"]
edition = "2026"
license = "MIT"
description = "High-performance systems telemetry and monitoring daemon"

[dependencies]
# Future package registry dependencies
# sqlite_driver = "1.0.0"

[build]
entrypoint = "src/main.rv"
output_binary = "enterprise_monitor"
optimization_level = "3"
strip_debug_symbols = true

[pro_mode]
strict_types = true
memory_management = "manual"  # "gc" or "manual"
target_arch = "x86_64"
```

---

## 2. Multi-Module Project Architecture

Large codebases decompose into specialized modules. In Runvoid, modules are included cleanly using `include`:

### `src/models/user.rv`:
```runvoid
action create_user(username, role) {
    remember u = {
        "username": username,
        "role": role,
        "created_at": 1718000000
    }
    give u
}

action is_admin(user_obj) {
    if user_obj["role"] == "ADMIN" {
        give true
    }
    give false
}
```

### `src/main.rv`:
```runvoid
include "models/user.rv"

say cyan "=== ENTERPRISE DAEMON INGESTION ==="

remember root_user = create_user("superadmin", "ADMIN")
say "Created user: {root_user[\"username\"]}"

if is_admin(root_user) {
    say green "Administrative privileges confirmed."
} otherwise {
    say red "Permission denied!"
}
```

---

## 3. Automated Continuous Integration (CI/CD)

Modern development teams never merge code manually without passing automated validation. Below is a complete production GitHub Actions workflow (`.github/workflows/ci.yml`):

```yaml
name: Runvoid Continuous Integration

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build-and-test:
    name: Build & Verify on Ubuntu x86_64
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Source Code
        uses: actions/checkout@v4

      - name: Setup Rust & Cargo Toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache Cargo Dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run Format Check
        run: cargo fmt --check

      - name: Run Clippy Linter
        run: cargo clippy -- -D warnings

      - name: Execute Compiler Test Suites
        run: cargo test --verbose

      - name: Verify Documentation Build (mdBook)
        run: |
          cargo install mdbook || true
          mdbook build
```

---

## 4. Building Cross-Platform Release Packages

When distributing your finished Runvoid software to end users, provide native installers:

### 1. Standalone Binary Compilation
```bash
# Linux x86_64 stripped release binary:
runvoid build --release src/main.rv -o target/release/enterprise_monitor

# Windows x86_64 executable:
runvoid build --target x86_64-pc-windows-gnu --release src/main.rv -o target/release/enterprise_monitor.exe
```

### 2. Checksum Verification
Always generate SHA-256 cryptographic hashes for release artifacts:
```bash
sha256sum target/release/enterprise_monitor > target/release/enterprise_monitor.sha256
```

This guarantees users can verify that their downloaded executable has not been corrupted or tampered with during transit.
