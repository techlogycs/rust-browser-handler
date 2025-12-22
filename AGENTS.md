# AI Agent Guidelines for Rust Browser Handler

## Architecture Overview
This is a cross-platform CLI application that acts as a browser handler with platform-specific integrations. It intercepts HTTP/HTTPS links and routes them based on user-defined rules stored in JSON. The application compiles and runs on multiple platforms with different levels of system integration.

**Core Components:**
- `src/main.rs`: CLI entry point with clap-based command parsing and interactive mode
- `src/rules.rs`: JSON-based rule management (pattern matching, browser paths)
- `src/platform/`: Platform abstraction layer with trait-based handlers
  - `mod.rs`: PlatformHandler trait and conditional compilation
  - `windows_impl.rs`: Windows registry operations and browser detection
  - `linux_impl.rs`: Linux/XDG integration and browser detection
- `src/browser_discovery.rs`: Cross-platform browser name utilities

**Data Flow:** URL interception → Rule matching (substring/regex) → Browser launch or interactive prompt → Optional rule saving

**Design Decisions:** Platform-agnostic core with trait-based abstraction for OS-specific features; cross-compiled from Linux devcontainer; CLI-first with interactive fallback; graceful degradation on unsupported platforms.

## Development Workflows
- **Build (Linux):** `cargo build` (native Linux build with XDG integration)
- **Build (Windows):** `cargo build --target x86_64-pc-windows-gnu` (full Windows functionality cross-compilation)
- **Release Build (Linux):** `cargo build --release`
- **Release Build (Windows):** `cargo build --release --target x86_64-pc-windows-gnu`
- **Run (Linux):** `cargo run` (native execution)
- **Run (Windows):** `cargo run --target x86_64-pc-windows-gnu` (requires Windows or WSL)
- **Test:** `cargo test` (runs on Linux, platform-specific code conditionally compiled)
- **Debug:** Use VS Code LLDB extension; breakpoints in main.rs command handlers
- **Dev Setup:** Devcontainer auto-runs `cargo setup-dev` (installs hooks, tools); manual for local dev

## Project Conventions
- **CLI Patterns:** Use clap derive macros; subcommands like `add`, `list`, `register` (see main.rs examples)
- **Rule Storage:** JSON array in `%APPDATA%\RustBrowserHandler\rules.json` (Windows) or `~/.config/rust_browser_handler/rules.json` (Linux); patterns as strings or regex
- **Error Handling:** Use `anyhow` for errors; print to stderr with `eprintln!`
- **Platform Integration:** 
  - Windows: Registry keys under `HKEY_CLASSES_ROOT\http\shell\open\command`; winapi for user interactions
  - Linux: xdg-mime and .desktop files for XDG desktop integration
- **Logging:** env_logger with `log::info!`/`log::error!`; configurable via RUST_LOG
- **Code Style:** Standard Rust fmt; clippy warnings as errors; commit messages follow Gitmoji conventional commits

## Integration Points
- **External Dependencies:** winreg (registry, Windows-only), winapi (system calls, Windows-only), serde_json (config), regex (patterns)
- **Cross-Component Communication:** Rules passed as structs between modules; browser list shared via Vec
- **File Paths:** Use `dirs::config_dir()` for config; hardcoded browser paths for detection
- **Build Targets:** Default build works on any platform; use `--target x86_64-pc-windows-gnu` for Windows binaries

## Key Files for Reference
- `src/main.rs`: Command structure and interactive loop
- `src/rules.rs`: JSON serialization/deserialization patterns
- `src/platform/`: Platform abstraction layer
  - `mod.rs`: PlatformHandler trait definition
  - `windows_impl.rs`: Windows registry and browser detection
  - `linux_impl.rs`: Linux/XDG integration and browser detection
- `src/browser_discovery.rs`: Cross-platform utilities
- `.devcontainer/devcontainer.json`: Cross-compilation setup
- `.github/workflows/release.yml`: CI release process
- `README.md`: Usage examples and rule format