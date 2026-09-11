# 4. Natural Word Lists & Collections

Nearly every real-world program needs to store, organize, and manipulate collections of related items—whether it's a list of network connections, game inventory items, or sensor readings.

In Runvoid, collections are designed to read like conversational English phrases while maintaining the performance of native heap-allocated dynamic arrays.

---

## 1. Creating Collections

You can declare a list using either standard bracket syntax or comma-separated natural word lists:

```runvoid
// Using brackets:
remember numbers = [10, 20, 30, 40, 50]

// Using natural word list syntax:
remember backpack = "Torch", "Iron Shield", "Health Potion"
```

Both forms initialize a dynamically sized array stored on the heap with an 8-byte length header and contiguous element memory.

---

## 2. Modifying Collections: Adding and Removing

Runvoid provides conversational statements for mutating collections:

### Appending Elements: `add ... to`
To append an element to the end of a list:

```runvoid
add "Magic Ring" to backpack
add 60 to numbers
```

### Removing Elements: `remove ... from`
To find and remove an element from a list:

```runvoid
remove "Iron Shield" from backpack
```

If the item exists in the collection, the elements after it are shifted down to fill the gap, and the collection's length is decremented.

---

## 3. Querying Collections

### Checking Membership with `has`
Runvoid provides the `has` operator to check whether an element is present in a collection:

```runvoid
if backpack has "Torch" {
    say green "You can navigate dark dungeons safely!"
} otherwise {
    say yellow "It is too dark to see anything!"
}
```

### Collection Size: `count` and `.length`
To inspect how many elements are currently stored in a collection:

```runvoid
say "Backpack contains: {count backpack} items."
say "Number list has: {numbers.length} elements."
```

### Accessing Elements by Index
You can access individual elements using zero-based index brackets:

```runvoid
remember first_item = backpack[0]
say "Primary equipped item: {first_item}"
```

---

## 4. Iterating with `for every ... in`

Walking through every element of a list is one of the most common operations in programming. Runvoid provides the readable `for every item in collection` loop:

```runvoid
say "=== Inventory Status ==="

for every item in backpack {
    say " * {item}"
}
```

Under the hood, the compiler tracks the collection's pointer and length, automatically advancing through each element until the boundary is reached. You never have to worry about off-by-one errors or manual index tracking.

---

## 5. Complete Example: Equipment Inventory System

Let's combine these concepts into a complete inventory management example:

```runvoid
say cyan "=== ADVENTURE RPG INVENTORY ==="

remember inventory = "Iron Sword", "Leather Armor", "Healing Salve", "Rations"

say "Current gear count: {count inventory}"

// Equip a newly discovered weapon:
say "Found an Ancient Crossbow!"
add "Ancient Crossbow" to inventory

// Use up an item:
if inventory has "Healing Salve" {
    say green "Consumed Healing Salve: Health restored +50 HP."
    remove "Healing Salve" from inventory
}

// Display final inventory report:
say yellow "Updated Gear List:"
for every gear in inventory {
    say " [x] {gear}"
}
```

Running the code:
```bash
$ runvoid run inventory.rv
=== ADVENTURE RPG INVENTORY ===
Current gear count: 4
Found an Ancient Crossbow!
Consumed Healing Salve: Health restored +50 HP.
Updated Gear List:
 [x] Iron Sword
 [x] Leather Armor
 [x] Rations
 [x] Ancient Crossbow
```

---

## 6. Under the Hood: Memory Layout

In Runvoid, dynamic collections are managed by the runtime's memory allocator:
- Each collection is represented by a pointer to a heap buffer.
- The buffer header contains an 8-byte length and an 8-byte capacity.
- Elements are stored contiguously as 64-bit pointers or scalar values.
- When the capacity is exceeded, the runtime reallocates the buffer with a 1.5x growth factor, minimizing reallocations while keeping memory waste low.

