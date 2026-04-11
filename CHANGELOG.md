# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2026-04-11

### Fixed

- **Windows Build Failure (Issue #3)**: Fixed `cargo install` failure on Windows caused by usearch v2.24.0 using `MAP_FAILED` (a POSIX-only identifier). Added `build.rs` script that automatically patches the usearch header on Windows builds.
- **Cross-Platform CI Coverage**: Added dedicated compile jobs for Windows, macOS, and Linux to catch platform-specific build issues before releases.
- **MCP Content Wrapper**: Wrapped `tools/call` responses in MCP-compliant `{"content": [{"type": "text", ...}]}` format, fixing "content is missing" errors in Windsurf and other MCP clients.
- **Capability Advertisement**: Removed false `resources` and `prompts` capability claims from `initialize` response; server now only advertises `tools`.

### Added

- **MCP Protocol Handlers**: Added explicit `resources/list`, `resources/read`, and `prompts/list` handlers as defensive fallbacks.
- **Comprehensive MCP Test Suite**: 59 tests covering all 20 tools, JSON-RPC protocol handling, content wrapper validation, schema completeness, error/edge cases, and full lifecycle tests for KG, diary, and drawer operations.

## [0.4.0] - 2026-04-09

### Added

- **Full Parity with Python Implementation**:
  - Implemented `repair` command for HNSW index recovery from SQLite metadata.
  - Implemented `instructions` command for AI agent system-prompt onboarding.
  - Added interactive entity confirmation in `mempalace init` using `dialoguer`.
  - Ported emotional-marker and speech-pattern regex parsing to `src/extractor.rs`.
- **Sync with Latest Upstream (April 2026)**:
  - **Deterministic MD5 IDs**: Replaced unstable `DefaultHasher` with stable MD5 hashing for `drawer_id` to ensure idempotent writes.
  - **Mtime-Based Mining Skip**: Implemented file modification time tracking to skip unchanged files during re-mining (significant performance boost).
  - **MCP Server Hardening**: Bounded metadata scans in `mempalace_status` and `mempalace_get_taxonomy` to prevent OOM on massive palaces.
- **Advanced CLI Features**:
  - Enhanced `mine` with `--limit`, `--dry-run`, and `--agent` overrides.
  - Enhanced `search` with `--wing`, `--room`, and `--results` filters.
  - Native `.gitignore` support via hierarchical filtering.
- **Production Infrastructure**:
  - Re-architected CI/CD using an **Artifact-Based Pipeline** (70% faster workflow).
  - Global migration to **Rust 2026** formatting and Clippy-clean standards.

### Changed

- **MCP Tool Standard**: Renamed all tools to use the `mempalace_` prefix for marketplace compatibility.
- **L1 Context Logic**: Migrated context generation to a density-aware, room-grouped engine in the `dialect` module.

## [0.3.0] - 2026-04-09

### Added

- **2026 Gold Standard Benchmarks**:
  - Replaced legacy suites with RULER, StructMem, BABILong, and BEAM for rigorous reasoning validation.
  - Achieved perfect 1.000 integrity scores across all suites.
- **First-Class Android/Termux Support**:
  - Added `scripts/setup_android.sh` for automated mobile environment bootstrapping.
  - Patched `ort-sys` for native Android support and optimized linking against system `onnxruntime`.
- **Network Resilience**:
  - Implemented exponential backoff retry logic for resilient model ingestion during CI and local setup.

## [0.2.0] - 2026-04-08

### Added

- **AAAK Dialect V:3.2 Upgrade**:
  - Versioning, Adaptive Density, Proposition Atomisation, and Temporal Decay.
  - Metadata Overlay, Delta Encoding, and Faithfulness Scoring.
- **Semantic Memory Pruning**: Automated deduplication and clustering.
- **Storage Engine Migration**: Unified VectorStorage (SQLite + usearch).

## [0.1.0] - 2026-04-08

### Added

- **Memory Stack (L0-L3)**: Initial release of the 4-layer hierarchical context system.
