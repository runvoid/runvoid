# 14. Compiler Internals: From Tokens to Silicon

How does human-readable conversational text like `remember count = 5 + 10` transform into electronic pulses switching billions of transistors on an Intel or AMD microprocessor?

This chapter takes you inside the engine room of the **Runvoid Compiler** (`runvoid`), walking through the complete journey from character stream to executable silicon machine code.

---

## 1. The Compilation Journey: End-to-End Flow

```
                      +-----------------------------+
                      | Source File: main.rv        |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 1. Lexer (src/lexer.rs)     |
                      | Output: Vec<Token>          |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 2. Parser (src/parser.rs)   |
                      | Output: AST (Program)       |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 3. Typechecker              |
                      | (src/typechecker.rs)        |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 4. Optimizer                |
                      | (src/optimizer.rs)          |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 5. Codegen (src/codegen.rs) |
                      | Output: main.asm (NASM)     |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | 6. Assembler + Linker       |
                      | nasm -f elf64 + gcc         |
                      +-----------------------------+
                                     |
                                     v
                      +-----------------------------+
                      | Final Executable: main      |
                      +-----------------------------+
```

---

## 2. Phase 1: Lexical Analysis (Scanning)

The lexer (`src/lexer.rs`) reads the raw UTF-8 source file character by character. Its responsibility is to group characters into meaningful semantic units called **Tokens**:

```rust
pub enum TokenKind {
    Remember,      // "remember" keyword
    Say,           // "say" keyword
    Ident(String), // e.g. "player_score"
    Number(i64),   // e.g. 42
    Str(String),   // e.g. "Hello"
    Plus,          // '+'
    Equal,         // '='
    LBrace,        // '{'
    RBrace,        // '}'
    Eof,
}
```

### String Interpolation Lexing
When the lexer encounters a string literal containing `{...}`, such as `"Total: {x + 1}"`, it splits the token into structured template parts:
1. `TokenKind::Str("Total: ")`
2. `TokenKind::InterpolateStart`
3. `TokenKind::Ident("x")`, `TokenKind::Plus`, `TokenKind::Number(1)`
4. `TokenKind::InterpolateEnd`

This allows the parser to construct expression trees directly inside string templates!

---

## 3. Phase 2: Recursive Descent Parsing & Pratt Expressions

The parser (`src/parser.rs`) converts the linear stream of tokens into a hierarchical tree structure: the **Abstract Syntax Tree (AST)**.

Consider this statement:
```runvoid
remember result = (x + 10) * 2
```

The parser constructs the following AST tree:

```
                      Assignment Statement
                               |
                   +-----------+-----------+
                   |                       |
               Name: "result"         Expression: BinaryExpr (*)
                                           |
                               +-----------+-----------+
                               |                       |
                         BinaryExpr (+)           Literal: 2
                               |
                       +-------+-------+
                       |               |
                   Var: "x"       Literal: 10
```

### Operator Precedence (Pratt Parsing)
To ensure mathematical expressions evaluate correctly (e.g. `*` binds tighter than `+`), Runvoid implements standard binding power levels:

| Precedence Level | Operators | Description |
| :--- | :--- | :--- |
| 1 (Lowest) | `==`, `!=`, `<`, `>`, `is` | Relational comparisons |
| 2 | `bit or`, `bit xor` | Bitwise logical operators |
| 3 | `bit and` | Bitwise conjunction |
| 4 | `shift left`, `shift right` | Bit shifts |
| 5 | `+`, `-` | Addition and subtraction |
| 6 (Highest) | `*`, `/`, `%` | Multiplication, division, remainder |

```
Pratt Binding Algorithm:
1. Parse prefix expression (Literal, Identifier, Parenthesized Group).
2. While next operator binding power > current precedence:
   - Consume infix operator
   - Parse right-hand expression with operator's binding power
   - Wrap left and right in BinaryExpr AST node
```

---

## 4. Phase 3: Semantic Analysis & Typechecking

The typechecker (`src/typechecker.rs`) traverses the AST to verify program correctness before generating a single line of machine code:
- **Symbol Table:** Tracks all declared variables in the active scope, their stack frame offsets, and their types.
- **Arity Validation:** Ensures functions are called with the exact number of declared arguments.
- **Directive Enforcement:** If `remove Basic` is declared, it verifies that no untyped variables exist.

---

## 5. Phase 4: AST Optimization & Peephole Passes

Before emitting assembly, the AST optimizer (`src/optimizer.rs`) performs passes across the tree:
1. **Constant Folding:** Replaces `10 * 60` with `600`.
2. **Identity Simplification:** Replaces `x + 0` with `x`.
3. **Dead Code Elimination (DCE):** Prunes code blocks following unconditional `give` returns.

### Assembly Peephole Optimization
In `src/codegen.rs`, redundant machine instruction sequences are substituted with optimized forms:
- `mov rax, 0` $\to$ `xor eax, eax` (Saves 3 bytes of instruction cache and clears register with zero latency via register renaming).
- `imul rax, 8` $\to$ `shl rax, 3` (Replaces costly hardware multiplier with 1-cycle barrel shifter).
- `cmp rax, 0` $\to$ `test rax, rax` (Performs non-destructive bitwise test, faster flag evaluation).

---

## 6. Phase 5: x86_64 NASM Code Generation

The code generator (`src/codegen.rs`) translates AST nodes into standard x86_64 assembly code.

### Managing Local Variables on the Stack Frame
The compiler allocates 8 bytes on the stack for each local variable:
- Variable 1: `[rbp - 8]`
- Variable 2: `[rbp - 16]`
- Variable 3: `[rbp - 24]`

```
Stack Frame Anatomy:
High Memory
  [ Caller's Frame ]
  [ Return %rip   ] <-- Pushed by call instruction
  [ Saved %rbp    ] <-- Pushed by fn prologue (push rbp)
  ----------------- <--- %rbp points here
  [ Local Var 1   ] [rbp - 8]
  [ Local Var 2   ] [rbp - 16]
  [ Local Var 3   ] [rbp - 24]
  [ Spill Space   ] ...
  ----------------- <--- %rsp points here (Must be 16-byte aligned before call!)
Low Memory
```

When evaluating expressions, the compiler utilizes the primary `%rax` accumulator and `%rbx` secondary registers.

---

## 7. Phase 6: Native Assembly and Linking

Once `codegen.rs` writes the `.asm` file to disk, the compiler orchestrates the external toolchain:

```bash
# 1. Assemble NASM into ELF64 object file:
nasm -f elf64 -o app.o app.asm

# 2. Compile native C runtime:
gcc -c -O3 runtime/gui.c -o gui.o

# 3. Link executable:
gcc -no-pie app.o gui.o -o app -lX11 -lpthread
```

The resulting binary is a completely standalone ELF64 executable ready to run at microsecond speeds.
