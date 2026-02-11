# AtlasDB v0.1 Specification

AtlasDB is a minimal key-value storage engine written in Rust.
Version 0.1 focuses on an in-memory key-value store with a CLI interface.

---

## Data Model

- Keys are strings.
- Keys must not contain spaces.
- Keys may contain letters, numbers, `_`, `:`, `-`.
- Values are strings.
- Values may contain spaces if wrapped in double quotes.
- Each key maps to exactly one value.
- Setting an existing key overwrites the previous value.
- Deleting a key removes it from the store.

---

## Commands

### set

Insert or update a key-value pair.

Syntax:
set <key> <value>
set <key> "value with spaces"

Examples:
set name Atlas
set bio "Building a key-value engine in Rust"

Output:
OK

---

### get

Retrieve the value for a key.

Syntax:
get <key>

Output:
If key exists:
<value>

If key does not exist:
NULL

---

### del

Delete a key.

Syntax:
del <key>

Output:
If key existed:
1

If key did not exist:
0

---

### keys

List all keys in the database.

Syntax:
keys

Output:
Each key printed on a new line.
Order is not guaranteed.

---

### exit

Exit the program.

Syntax:
exit

Output:
Program terminates.

---

## Error Handling

If invalid syntax or missing arguments:

Output:
ERR <message>

Examples:
ERR missing key
ERR invalid command
ERR malformed input

---

## Scope (v0.1)

- In-memory storage only.
- No persistence yet.
- No networking.
- No concurrency.
- No indexing optimizations.

Future versions may introduce:
- Append-only log (WAL)
- Disk persistence
- Compaction
- SSTables
- Bloom filters
- Background workers
