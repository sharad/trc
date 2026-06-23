# DESIGN.md

# trc — Simple LRU Cache

Operations:

* put(key,value)
* get(key)

Expected complexity:

* get → O(1)
* put → O(1)

## Architecture

Two structures:

### HashMap

Stores:

key → node_index

Used for fast lookup.

### Node Vector

Stores nodes and linked-list metadata.

Each node:

* key
* value
* prev
* next

Indexes are 1-based.

## Ordering

head = most recently used

tail = least recently used

Access:

get(key)
→ move node to head

Insert:

put(key,value)
→ move node to head

Eviction:

capacity exceeded
→ remove tail

## Constraints

Current implementation:

K: Clone + Eq + Hash
V: Clone



## Synchronization strategy (e.g. mutexes, read-write locks)

For thread LRU

we can use

```rust
Arc<Mutex<Lru<K,V>>

```

