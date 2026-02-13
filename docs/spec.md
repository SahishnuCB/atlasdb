# AtlasDB v0.1 Specification

## 1. Overview

AtlasDB v0.1 is a single-threaded, in-memory key-value storage engine written in Rust.

It provides a CLI interface for interacting with a String → String data store.

This version does not include persistence, disk storage, concurrency, or networking.

---

## 2. Architecture

CLI → Parser → Engine → Storage

### Layer Responsibilities

- **CLI**
  - Handles user input and output.
  - Does not contain business logic.

- **Parser**
  - Converts raw input strings into structured `Command` values.
  - Performs key validation.
  - Detects unknown commands.
  - Returns structured parse errors.

- **Engine**
  - Executes business logic.
  - Converts storage results into `Response`.
  - Applies overwrite rules.

- **Storage**
  - Provides raw data access.
  - Uses an in-memory `HashMap<String, String>`.
  - Returns neutral types (e.g., `Option<String>`).
  - Does not return formatted output like `"NULL"`.

---

## 3. Data Model

### Keys

- UTF-8 strings
- Must not contain spaces
- Allowed characters:
  - `a–z`
  - `A–Z`
  - `0–9`
  - `_`, `:`, `-`
- Each key maps to exactly one value

### Values

- UTF-8 strings
- May contain spaces if wrapped in double quotes

### Rules

- Setting an existing key overwrites the previous value.
- Deleting removes the key from the store.
- Accessing a nonexistent key does not cause a crash.
- The system must never panic due to malformed CLI input.

---

## 4. Command Interface

### 4.1 set

Insert or update a key-value pair.

Syntax:
```
set <key> <value>
set <key> "value with spaces"
```

Response:
```
OK
```

---

### 4.2 get

Retrieve the value for a key.

Syntax:
```
get <key>
```

Response:

If key exists:
```
<value>
```

If key does not exist:
```
NULL
```

---

### 4.3 del

Delete a key.

Syntax:
```
del <key>
```

Response:

If key existed:
```
1
```

If key did not exist:
```
0
```

---

### 4.4 keys

List all keys in the database.

Syntax:
```
keys
```

Response:

Each key printed on a new line.

Order is implementation-dependent.

---

### 4.5 exit

Terminate the program.

Syntax:
```
exit
```

Behavior:

Program terminates immediately.

---

## 5. Error Handling

If syntax is invalid or arguments are missing:

Response format:
```
ERR <message>
```

Examples:
```
ERR invalid command
ERR missing key
ERR malformed input
```

Malformed input must not cause a panic.

---

## 6. Scope (v0.1)

- In-memory storage only
- Uses `HashMap` internally
- Single-threaded
- No persistence
- No Write-Ahead Log (WAL)
- No SSTables
- No compaction
- No networking
- No background workers
- No indexing optimizations
- No search functionality

---

## 7. Non-Goals (v0.1)

The following are explicitly out of scope:

- Disk-based storage
- Transaction support
- Concurrency control
- Replication
- Authentication
- Performance optimizations

---

## 8. Planned Evolution

Future versions may introduce:

- Write-Ahead Log (WAL)
- MemTable (`BTreeMap`)
- Immutable SSTables
- Compaction
- Bloom filters
- Background workers
- Persistent storage
- LSM-tree architecture
