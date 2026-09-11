# 10.1 Key-Value Dictionaries & Maps

Real-world applications frequently model data with named fields and dynamic attributes—user profiles, HTTP configuration headers, JSON payloads, and game entity state.

Runvoid 0.3.0 introduces first-class **Key-Value Dictionaries (Maps)**, combining human-friendly syntax with a high-performance native open-addressing hash table implementation.

---

## 1. Defining Dictionaries

Runvoid offers two expressive styles for declaring maps:

### Conversational Field Notation
Ideal for declaring domain objects and state models:

```runvoid
remember hero = name: "Aria", hp: 100, level: 5, class: "Ranger"
```

### JSON-Style Brace Notation
Familiar to developers working with REST APIs, configuration files, and serialized data:

```runvoid
remember server_config = {
    "host": "0.0.0.0",
    "port": 8080,
    "max_connections": 10000,
    "tls_enabled": true
}

// Initializing an empty map:
remember session_cache = {}
```

---

## 2. Accessing Map Properties

Runvoid allows you to retrieve values using either standard bracket indexing or conversational English possessive syntax:

### 1. Traditional Bracket Indexing
```runvoid
remember host_address = server_config["host"]
say "Binding to host: {host_address}"
```

### 2. Conversational Possessive Syntax (`entity's property`)
To write code that reads like natural English, use the possessive `'s` operator:

```runvoid
say "Character Name: {hero's name}"
say "Current Health: {hero's hp}"
say "Character Level: {hero's level}"
```

The possessive syntax translates directly to a dictionary lookup for that field name, with zero runtime performance difference.

---

## 3. Mutating Dictionaries

### Adding and Updating Entries: `add ... to`
You can insert a new key-value pair or overwrite an existing key's value using conversational syntax:

```runvoid
// Insert a new key:
add "mana": 75 to hero

// Update an existing key:
add "hp": 120 to hero

say "Updated Mana: {hero's mana}"
say "Restored HP: {hero's hp}"
```

### Removing Entries: `remove ... from`
To delete a key-value entry from a dictionary:

```runvoid
remove "class" from hero
```

---

## 4. Querying and Inspecting Maps

### Checking Key Existence: `has`
Verify whether a given key exists in a dictionary using the `has` operator:

```runvoid
if hero has "mana" {
    say green "Caster abilities unlocked."
} otherwise {
    say yellow "Physical fighter build."
}
```

### Measuring Size: `count`
Get the total number of active key-value pairs currently stored in the map:

```runvoid
say "Total attributes tracked: {count hero}"
```

### Extracting Keys and Values
You can extract all keys or values as standard Runvoid word lists for batch processing or iteration:

```runvoid
remember attributes = keys hero
say "Attribute keys: {count attributes}"

for every attr in attributes {
    say " - {attr}: {hero[attr]}"
}
```

---

## 5. Under the Hood: Native Hash Table Architecture

Behind the scenes in the Runvoid native runtime (`runtime/gui.c`), maps are implemented as compact, cache-conscious hash tables:

1. **Hash Algorithm:** Keys are hashed using the **FNV-1a 64-bit algorithm**, which provides optimal distribution across string and integer keys with minimal CPU clock cycle overhead.
2. **Collision Resolution:** Collisions are resolved using **open addressing with linear probing**, maximizing CPU L1/L2 cache locality by keeping entries contiguous in memory.
3. **Dynamic Resizing:** The hash table maintains a load factor threshold of 0.70. When exceeded, the capacity doubles and keys are re-indexed into a contiguous memory block.
4. **Zero Overhead Reads:** When key literals are known at compile time, their hashes are precomputed during compilation, skipping runtime hashing passes completely!

