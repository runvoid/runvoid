# 1.3 The Compiler & CLI Tooling

The `runvoid` command-line tool provides everything you need to develop, format, benchmark, and deploy native applications.

## Key Subcommands

### 1. `runvoid run <file.rv>`
Compiles your code directly into memory and executes it immediately:
```bash
$ runvoid run hello.rv
```

### 2. `runvoid build <file.rv> [-o <name>]`
Builds an optimized standalone Linux ELF executable:
```bash
$ runvoid build hello.rv -o my_app
$ ./my_app
```
Add `--verbose` to inspect the compilation stages:
```bash
$ runvoid build hello.rv -o my_app --verbose
```

### 3. `runvoid emit-asm <file.rv>`
Inspect the generated x86_64 NASM assembly code directly:
```bash
$ runvoid emit-asm hello.rv
```

### 4. `runvoid fmt [-w] <file.rv>`
Automatically format and indent your code:
```bash
$ runvoid fmt hello.rv        # Print formatted code
$ runvoid fmt -w hello.rv     # Overwrite file in-place
```

### 5. `runvoid new <game|gui|script> <name.rv>`
Scaffold beginner-friendly starter projects:
```bash
$ runvoid new game arcade.rv    # 2D screen game template
$ runvoid new gui dashboard.rv  # Desktop GUI template
$ runvoid new script tool.rv    # Conversational script template
```

### 6. `runvoid cheat`
Displays an interactive color-coded terminal cheat sheet anytime you need a quick syntax reminder.
