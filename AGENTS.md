# AI Agent Guidelines for Rust Browser Handler

## Architecture Overview
This is a Windows-specific CLI application that acts as a default browser handler. It intercepts HTTP/HTTPS links and routes them based on user-defined rules stored in JSON. The application is designed to compile on multiple platforms but only functions fully on Windows.

**Core Components:**
- `src/main.rs`: CLI entry point with clap-based command parsing and interactive mode
- `src/rules.rs`: JSON-based rule management (pattern matching, browser paths)
- `src/registry_handler.rs`: Windows registry operations for default handler registration (Windows-only)
- `src/browser_discovery.rs`: Browser detection via filesystem paths (cross-platform) and registry (Windows-only)

**Data Flow:** URL interception → Rule matching (substring/regex) → Browser launch or interactive prompt → Optional rule saving

**Design Decisions:** Windows-only functionality due to registry/winapi dependencies; cross-compiled from Linux devcontainer; CLI-first with interactive fallback; multiplatform compilation with graceful degradation.

## Development Workflows
- **Build:** `cargo build` (compiles on any platform; Windows features gracefully fail on non-Windows)
- **Windows Build:** `cargo build --target x86_64-pc-windows-gnu` (full functionality cross-compilation)
- **Release Build:** `cargo build --release --target x86_64-pc-windows-gnu`
- **Run:** `cargo run --target x86_64-pc-windows-gnu` (requires Windows or WSL)
- **Test:** `cargo test` (runs on Linux, but Windows-specific code is conditionally compiled)
- **Debug:** Use VS Code LLDB extension; breakpoints in main.rs command handlers
- **Dev Setup:** Devcontainer auto-runs `cargo setup-dev` (installs hooks, tools); manual for local dev

## Project Conventions
- **CLI Patterns:** Use clap derive macros; subcommands like `add`, `list`, `register` (see main.rs examples)
- **Rule Storage:** JSON array in `%APPDATA%\RustBrowserHandler\rules.json`; patterns as strings or regex
- **Error Handling:** Use `anyhow` for errors; print to stderr with `eprintln!`
- **Windows Integration:** Registry keys under `HKEY_CLASSES_ROOT\http\shell\open\command`; winapi for user interactions
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
- `.devcontainer/devcontainer.json`: Cross-compilation setup
- `.github/workflows/release.yml`: CI release process
- `README.md`: Usage examples and rule format