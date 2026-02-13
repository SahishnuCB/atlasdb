# AtlasDB

AtlasDB is an embedded key-value storage engine written in Rust.

It is being built incrementally toward a production-style LSM-tree architecture, starting with a minimal in-memory core.

---

## Current Version — v0.1

AtlasDB v0.1 implements:

- Single-threaded execution
- In-memory storage (HashMap)
- String → String key-value model
- Linux-style CLI shell
- Layered architecture (CLI → Parser → Engine → Storage)

No persistence is included in this version.

---

## Architecture (v0.1)

CLI  
↓  
Parser  
↓  
Engine  
↓  
Storage (HashMap)

### Layer Responsibilities

- **CLI** — Handles input/output only
- **Parser** — Converts user input into `Command`
- **Engine** — Executes business logic and returns `Response`
- **Storage** — Provides raw data access (no formatting)

---

## Design Goals

- Clean separation of concerns
- Versioned feature progression
- Production-style architectural thinking
- Small, incremental Git commits

---

## Roadmap

Planned future features:

- Write-Ahead Log (WAL)
- MemTable (BTreeMap)
- Immutable SSTables
- Compaction
- Bloom filters
- Background workers
- Disk persistence

---

## Versioning Philosophy

Each version introduces one major architectural concept while preserving clean boundaries.
