# 10.1 Key-Value Dictionaries & Maps

Runvoid 0.3.0 brings native dictionary and hash map data structures, allowing developers to manage structured associative data without needing external dependencies.

## Defining Maps

Maps can be created using either conversational pairs or bracketed key-value notation:

```runvoid
// Conversational notation
remember player = name: "Alex", hp: 100, class: "Mage"

// JSON-style brace notation
remember config = { "host": "127.0.0.1", "port": 8080 }

// Empty map
remember cache = {}
```

## Reading Values

You can read map values in two ways:
1. **Bracket Indexing:** `player["name"]`
2. **Possessive Syntax:** `player's name`

```runvoid
say "Hero Name: {player['name']}"
say "Hit Points: {player's hp}"
```

## Mutating Maps

Adding and removing key-value pairs uses conversational English commands:

```runvoid
// Add new key-value pair or overwrite existing
add "shield": 50 to player

// Remove key
remove "class" from player
```

## Checking Keys & Counting Entries

```runvoid
// Check key existence
if player has "shield" {
    say green "Shield is equipped!"
}

// Count number of key-value pairs
say "Total attributes: {count player}"

// Retrieve keys and values as lists
remember all_keys = keys player
remember all_values = values player
```
