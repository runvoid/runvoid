# 17. Database Engineering & Persistent Storage

Modern software systems depend on persistent data storage: saving user state, logging system telemetry, indexing records, and ensuring transactional durability across unexpected power losses and system reboots.

In this chapter, you will master database engineering in Runvoid:
1. Understanding the storage hierarchy and memory-to-disk persistence pipelines.
2. Implementing a Write-Ahead Log (WAL) for crash-resilient append-only storage.
3. Building an in-memory hash-indexed Key-Value store with binary serialization.
4. Implementing an on-disk Page Index for constant-time $O(1)$ random lookups.
5. Binding directly to **SQLite3** via C FFI for enterprise ACID relational queries.

---

## 1. Storage Architecture: The Hardware Hierarchy

To engineer high-performance database engines, you must understand how data moves between hardware layers:

```
+-------------------------------------------------------------------------+
| Level 1: CPU Registers (0.5 ns, ~1 KB per core)                         |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Level 2: L1/L2/L3 Hardware Cache (1 - 10 ns, 32 KB - 64 MB)            |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Level 3: Main Memory / RAM (50 - 80 ns, 16 GB - 128 GB)                 |
+-------------------------------------------------------------------------+
                                     |
                                     v (Explicit Flush: fsync / msync)
+-------------------------------------------------------------------------+
| Level 4: NVMe PCIe SSD (10 - 50 µs, 1 TB - 8 TB)                        |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Level 5: Magnetic Hard Disk (HDD) (5 - 15 ms, Mechanical Seek)          |
+-------------------------------------------------------------------------+
```

When your Runvoid application calls `write data into filename`, the bytes do not immediately reach physical flash cells. Instead:
1. Runvoid issues the `sys_write` POSIX system call.
2. The Linux/Windows kernel writes the buffers into the **Page Cache** in RAM.
3. The kernel returns immediately, marking dirty pages.
4. A background flusher thread (`pdflush`/`kswapd`) writes dirty pages to physical disk every 5–30 seconds.
5. If the machine loses power before this flush, unwritten data in the page cache is lost!

To guarantee **durability** (the 'D' in ACID), database engines force physical synchronization using `fsync` or explicit journal records.

---

## 2. Write-Ahead Logging (WAL) Architecture

A Write-Ahead Log ensures that no mutation occurs in the primary database files until the mutation is safely recorded on persistent sequential storage:

```
[ Client Mutation: SET user:42 = "Elena" ]
                 |
                 v
   +-----------------------------+
   |  1. Append to WAL on Disk   |  <-- Fast sequential append (O(1))
   +-----------------------------+
                 |
                 v
   +-----------------------------+
   | 2. Update RAM Hash Index    |  <-- Instantaneous query access
   +-----------------------------+
                 |
                 v (Background Checkpoint)
   +-----------------------------+
   | 3. Flush to Main Data Store |  <-- Coalesced bulk write
   +-----------------------------+
```

### The Mini-WAL Implementation in Runvoid

Let's implement a complete crash-resilient write-ahead logger:

```runvoid
say cyan "=================================================="
say cyan "       RUNVOID WRITE-AHEAD LOG (WAL) ENGINE       "
say cyan "=================================================="

remember wal_log = "journal.wal"
remember data_store = "primary_data.db"

// Action: Append a transaction to the log
action wal_append(operation, key, value) {
    remember timestamp = 1718000000 // UNIX epoch
    remember entry = "{timestamp}|{operation}|{key}|{value}\n"
    write entry into wal_log
    say green "WAL Committed: [{operation}] {key} = {value}"
}

// Action: Replay WAL during recovery
action wal_recover(wal_file, target_cache) {
    if file wal_file exists {
        say yellow "WAL detected. Initiating crash recovery playback..."
        remember log_content = read wal_file
        remember lines = log_content.split("\n")
        
        remember count_recovered = 0
        for every line in lines {
            if line.length > 0 {
                remember parts = line.split("|")
                remember op = parts[1]
                remember k = parts[2]
                remember v = parts[3]
                
                if op == "SET" {
                    add k: v to target_cache
                    count_recovered = count_recovered + 1
                }
            }
        }
        say green "Recovery complete! Replayed {count_recovered} transactions."
    } otherwise {
        say "No active WAL journal found. Clean state."
    }
}

// Memory Cache Table
remember db_cache = {}

// Execute Recovery on Startup
wal_recover(wal_log, db_cache)

// Perform New Mutations
say yellow "Processing live client transactions..."
wal_append("SET", "account:1001", "5400.50")
add "account:1001": "5400.50" to db_cache

wal_append("SET", "account:1002", "12500.00")
add "account:1002": "12500.00" to db_cache

say cyan "Current Active Records in Memory: {count db_cache}"
```

---

## 3. High-Performance Relational SQL with SQLite3

When your project requires complex relations, secondary indexes, transactions, and SQL queries, Runvoid bridges natively to **SQLite3** through C Foreign Function Interface (FFI).

### SQLite3 C Header Bindings

SQLite exports an elegant C interface:
- `sqlite3_open(filename, **db)`: Opens or creates a database file.
- `sqlite3_exec(db, sql, callback, arg, **errmsg)`: Executes one or more SQL statements.
- `sqlite3_close(db)`: Releases memory and unlocks database files.

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use lib "sqlite3"

extern "C" {
    action sqlite3_open(filename: String, db_handle: Ptr) -> Int
    action sqlite3_exec(db: Ptr, sql: String, callback: Ptr, arg: Ptr, errmsg: Ptr) -> Int
    action sqlite3_close(db: Ptr) -> Int
}

action run_sql_migration() {
    remember db: Ptr = 0
    remember rc: Int = sqlite3_open("enterprise.db", addr db)
    
    if rc != 0 {
        say red "Database initialization failed with code: {rc}"
        give 1
    }
    
    say green "Connected to SQLite3 storage engine successfully."
    
    // Create Users Table
    remember schema_sql: String = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, username TEXT NOT NULL UNIQUE, balance REAL DEFAULT 0.0);"
    sqlite3_exec(db, schema_sql, 0, 0, 0)
    say green "Applied schema migration: table 'users' is ready."
    
    // Insert Records
    remember insert_sql: String = "INSERT OR IGNORE INTO users (id, username, balance) VALUES (1, 'alice', 950.00), (2, 'bob', 1200.50);"
    sqlite3_exec(db, insert_sql, 0, 0, 0)
    say green "Seeded sample transactional records."
    
    sqlite3_close(db)
    say cyan "Database handle closed safely."
    give 0
}

run_sql_migration()
```

---

## 4. Key Performance Insights

| Storage Strategy | Read Latency | Write Latency | Durability Level | Memory Footprint |
| :--- | :--- | :--- | :--- | :--- |
| **RAM Hash Map** | $< 50\text{ ns}$ | $< 50\text{ ns}$ | Zero (Volatile) | High (Full Dataset) |
| **Write-Ahead Log (WAL)** | N/A (Append-Only) | $10\text{ µs} - 1\text{ ms}$ | Very High | Minimal ($O(1)$ stream) |
| **Page-Indexed DB** | $< 100\text{ µs}$ | $< 500\text{ µs}$ | Full ACID | Low (Page Cache) |
| **SQLite3 Native FFI** | $< 20\text{ µs}$ | $< 1\text{ ms}$ | Strict Full ACID | Medium (~4 MB) |

By combining Runvoid's dictionary syntax for in-memory caching and zero-overhead C FFI for SQLite3 durability, you can build enterprise-grade data microservices capable of serving millions of records with minimal CPU and RAM consumption!
