# Rust KV

A persistent, concurrent key-value server built from scratch in Rust.

The project implements an in-memory key-value store backed by a Write-Ahead Log (WAL), crash recovery, log compaction, a custom TCP command protocol, persistent client connections, and concurrent access using Rust's synchronization primitives.

The goal of the project is to explore systems programming concepts in Rust by building the core components of a database without relying on external database or networking frameworks.

> **Status:** v0.1.0 — single-node persistent KV server  
> **Planned:** replication, multi-node clustering, and Raft consensus.

---

## Features

- In-memory key-value storage
- `SET`, `GET`, and `DELETE` operations
- Write-Ahead Log (WAL)
- State recovery from WAL after restart
- Durable writes using `sync_data()`
- WAL compaction
- TCP server
- Custom text-based protocol
- Persistent TCP connections
- Multiple concurrent clients
- Thread-per-client concurrency model
- Shared state using `Arc<RwLock<KvStore>>`
- Concurrent readers with exclusive writers
- Command parser with unit tests
- No external runtime or database dependencies

---

## Architecture

```text
                         TCP Client
                             │
                    SET / GET / DELETE
                             │
                             ▼
                    ┌─────────────────┐
                    │   TCP Server    │
                    │  TcpListener    │
                    └────────┬────────┘
                             │
                       client thread
                             │
                             ▼
                    ┌─────────────────┐
                    │ Command Parser  │
                    └────────┬────────┘
                             │
                             ▼
                  Arc<RwLock<KvStore>>
                             │
                 ┌───────────┴───────────┐
                 │                       │
                 ▼                       ▼
        ┌─────────────────┐      ┌─────────────────┐
        │ In-Memory Store │      │       WAL       │
        │   HashMap<K,V>  │      │    data.log     │
        └─────────────────┘      └─────────────────┘
```

`KvStore` owns both the in-memory `HashMap` and the WAL abstraction.

Mutating operations are written to the WAL before the in-memory state is changed.

---

## Write-Ahead Log

The storage engine uses a simple append-only Write-Ahead Log.

Example:

```text
SET language Rust
SET database PostgreSQL
DELETE language
```

For a `SET` operation, the server performs:

```text
1. Append SET command to WAL
2. sync_data()
3. Update HashMap
4. Return success
```

For a `DELETE` operation:

```text
1. Append DELETE command to WAL
2. sync_data()
3. Remove key from HashMap
4. Return result
```

This ordering prevents the server from acknowledging an in-memory mutation before the corresponding WAL record has been persisted.

---

## Recovery

When the server starts, `KvStore` opens the WAL and replays its entries.

```text
data.log
   │
   ▼
Wal::replay()
   │
   ├── SET    → HashMap::insert()
   │
   └── DELETE → HashMap::remove()
   │
   ▼
Recovered in-memory state
```

This allows the key-value state to survive process restarts.

---

## WAL Compaction

Because the WAL is append-only, repeated updates can create obsolete records.

For example:

```text
SET name Alice
SET name Bob
SET name Charlie
DELETE name
SET language Rust
```

The current state only requires:

```text
SET language Rust
```

The project implements WAL compaction that writes the current in-memory state into a temporary compacted log and replaces the old WAL.

Automatic compaction policy is not implemented in v0.1.0; compaction is currently exposed by the storage layer.

---

## Concurrency Model

The store is shared between client threads using:

```rust
Arc<RwLock<KvStore>>
```

`Arc` provides shared ownership of the store between threads.

`RwLock` provides synchronization:

```text
GET     → read lock
SET     → write lock
DELETE  → write lock
```

Multiple clients can therefore hold read locks concurrently:

```text
Client A ── GET ── read ──┐
                           ├── KvStore
Client B ── GET ── read ──┘
```

Mutating operations require exclusive access:

```text
Client A ── SET ── write ───── KvStore

Client B ── GET ── waits
Client C ── DELETE ── waits
```

Critical sections are kept short. Network responses are written after the store lock is released whenever possible.

---

## TCP Protocol

The server listens on:

```text
127.0.0.1:4000
```

The protocol is line-based.

### SET

Request:

```text
SET language Rust
```

Response:

```text
OK
```

Values may contain spaces:

```text
SET name Ivan Portnov
```

### GET

Request:

```text
GET language
```

Response:

```text
VALUE Rust
```

If the key does not exist:

```text
NOT_FOUND
```

### DELETE

Request:

```text
DELETE language
```

Response:

```text
DELETED Rust
```

If the key does not exist:

```text
NOT_FOUND
```

### Invalid command

Request:

```text
HELLO world
```

Response:

```text
ERROR: Invalid command
```

---

## Running the Project

### Requirements

- Rust toolchain
- Cargo
- `nc` / Netcat for manual TCP testing

Clone the repository:

```bash
git clone https://github.com/phonicss/rust-distributed-kv.git
cd rust-distributed-kv
```

Run the server:

```bash
cargo run
```

The server starts on:

```text
KV server listening on 127.0.0.1:4000
```

Connect from another terminal:

```bash
nc 127.0.0.1 4000
```

Example session:

```text
SET language Rust
OK

GET language
VALUE Rust

DELETE language
DELETED Rust

GET language
NOT_FOUND
```

The TCP connection remains open, allowing multiple commands to be executed during the same client session.

---

## Testing

Run the test suite:

```bash
cargo test
```

The current unit tests cover command parsing for:

```text
GET
SET
DELETE
invalid commands
```

Example:

```text
running 4 tests

test command::tests::parse_get_command ... ok
test command::tests::parse_set_command ... ok
test command::tests::parse_delete_command ... ok
test command::tests::parse_invalid_command ... ok

test result: ok. 4 passed; 0 failed
```

Code formatting can be checked with:

```bash
cargo fmt --check
```

Static analysis:

```bash
cargo clippy
```

---

## Project Structure

```text
src/
├── main.rs       # TCP server and client handling
├── command.rs    # Command representation and parser
├── store.rs      # In-memory KV storage
└── wal.rs        # Write-Ahead Log and recovery
```

### `main.rs`

Responsible for:

- TCP listener
- accepting connections
- spawning client threads
- persistent client sessions
- executing parsed commands
- writing protocol responses

### `command.rs`

Defines the command protocol:

```rust
enum Command {
    Set { key: String, value: String },
    Get(String),
    Delete(String),
}
```

It also contains command parser unit tests.

### `store.rs`

Contains `KvStore`, which owns:

```text
HashMap<String, String>
Wal
```

It provides the storage operations:

```text
set()
get()
delete()
compact()
```

### `wal.rs`

Responsible for:

- opening the WAL
- appending `SET`
- appending `DELETE`
- synchronizing writes
- replaying the WAL
- compacting the WAL

---

## Design Decisions

### WAL before memory mutation

Mutating operations are persisted before changing the in-memory state.

This establishes the basic durability invariant:

```text
WAL write succeeds
        ↓
memory mutation
        ↓
success response
```

### `RwLock` instead of `Mutex`

A `Mutex<KvStore>` would serialize all operations, including independent reads.

`RwLock<KvStore>` allows multiple readers while preserving exclusive access for mutations.

### Owned values outside critical sections

`KvStore::get()` returns a borrowed value:

```rust
Option<&String>
```

The server clones the returned value while holding the read lock and then releases the guard before performing network I/O.

This prevents slow clients from unnecessarily holding the store lock.

### Standard library networking

The current implementation intentionally uses:

```text
std::net
std::thread
std::sync
```

rather than an asynchronous runtime.

This keeps the implementation small and exposes the underlying networking and synchronization model directly.

---

## Current Limitations

v0.1.0 is intentionally a small single-node storage system.

Current limitations include:

- single server node
- fixed server address
- no replication
- no consensus protocol
- no authentication
- no TLS
- no automatic WAL compaction policy
- no checksums or corruption detection for WAL records
- no snapshot protocol
- thread-per-client model
- limited integration testing

This project is educational and is not intended for production use.

---

## Roadmap

### v0.1 — Single-node storage

- [x] In-memory key-value store
- [x] WAL
- [x] Crash recovery
- [x] Durable WAL writes
- [x] WAL compaction
- [x] TCP protocol
- [x] Persistent connections
- [x] Concurrent clients
- [x] `Arc<RwLock<KvStore>>`
- [x] Command parser tests

### v0.2 — Replication

- [ ] Configurable node addresses
- [ ] Three-node cluster
- [ ] Leader / follower roles
- [ ] Replicate mutations to followers
- [ ] Replication acknowledgements
- [ ] Quorum writes
- [ ] Node failure handling
- [ ] Follower recovery

### v0.3 — Consensus

- [ ] Raft terms
- [ ] Leader election
- [ ] Heartbeats
- [ ] Replicated log
- [ ] Commit index
- [ ] Majority consensus
- [ ] Leader failover

---

## What This Project Explores

The project is primarily an exercise in:

- Rust ownership and borrowing
- error handling with `Result` and `Option`
- file I/O
- persistence
- Write-Ahead Logging
- crash recovery
- TCP networking
- protocol design
- multithreading
- shared ownership with `Arc`
- synchronization with `RwLock`
- critical-section design
- database storage fundamentals

Future versions will extend the project into distributed systems concepts such as replication, quorum, failure recovery, and consensus.

---

## License

This project is currently provided for educational and portfolio purposes.