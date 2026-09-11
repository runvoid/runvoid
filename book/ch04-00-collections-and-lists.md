# 4. Natural Word Lists & Collections

Most programming languages feature arrays, vectors, or lists. In Runvoid, collections are designed to read like conversational English phrases.

## Creating Word Lists

Create a collection simply by separating elements with commas:

```runvoid
remember backpack = "Torch", "Shield", "Health Potion"
```

## Adding and Removing Elements

```runvoid
# Appending to a list:
add "Magic Ring" to backpack

# Removing from a list:
remove "Shield" from backpack
```

## Checking Membership: `has`

```runvoid
if backpack has "Torch" {
    say "You have light in dark caverns!"
}
```

## Counting Elements

```runvoid
say "Total gear items: {count backpack}"
```

## Iterating with `for every ... in`

Iterate through every element without managing indices or boundaries:

```runvoid
for every item in backpack {
    say " - {item}"
}
```
