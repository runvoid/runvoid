# 13. Systems Programming & C Interop Masterclass

Low-level systems software connects hardware devices to high-level application logic. Operating systems kernels, database storage engines, cryptography accelerators, and network proxies rely on standard C Application Binary Interfaces (ABIs).

In this masterclass, you will learn how to interface Runvoid directly with native operating system APIs, execute POSIX system calls, bind to external C shared libraries, implement a raw hex memory inspector, and build a high-performance network client.

---

## 1. The C ABI Calling Convention Invariant

To safely interact with C libraries, you must understand how arguments and return values cross the boundary between Runvoid and native C functions.

### System V AMD64 ABI (Linux & BSD)

Under the System V calling convention:
- **First 6 integer/pointer parameters:** Passed in registers:
  1. `%rdi`
  2. `%rsi`
  3. `%rdx`
  4. `%rcx`
  5. `%r8`
  6. `%r9`
- **Return value:** Returned in the `%rax` register (or `%rdx:%rax` for 128-bit values).
- **Stack Frame Alignment:** The stack pointer `%rsp` must be 16-byte aligned before issuing the `call` instruction.
- **Callee-Saved Registers:** A called C function will preserve `%rbx`, `%rsp`, `%rbp`, `%r12`, `%r13`, `%r14`, and `%r15`. All other registers are volatile (caller-saved).

### Microsoft x64 Calling Convention (Windows)

Under Windows:
- **First 4 parameters:** Passed in registers:
  1. `%rcx`
  2. `%rdx`
  3. `%r8`
  4. `%r9`
- **Shadow Space (Home Space):** The caller must allocate at least **32 bytes** on the stack before issuing the `call` instruction.
- **Return value:** Returned in `%rax`.

Runvoid's compiler abstracts these platform differences automatically: when compiling for Windows, it adjusts the register assignment and allocates shadow space transparently.

---

## 2. Binding to the Standard C Library (`libc`)

Let's explore binding to standard POSIX `libc` functions using `extern "C"`:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior

extern "C" {
    // Standard I/O and utility symbols from libc:
    action puts(msg: String) -> Int
    action getpid() -> Int
    action getuid() -> Int
    action system(command: String) -> Int
}

remember current_pid: Int = getpid()
remember user_id: Int = getuid()

say "Host Process ID (PID): {current_pid}"
say "User UID: {user_id}"

// Execute shell command via libc system()
puts("Executing shell check via native C FFI...")
system("uname -srm")
```

---

## 3. High-Performance Native Networking via POSIX Sockets

Let's build a raw TCP network client in Runvoid by calling POSIX socket functions:
- `socket(AF_INET, SOCK_STREAM, 0)`: Creates network endpoint.
- `connect(sockfd, addr, addrlen)`: Establishes TCP handshake.
- `write(sockfd, buf, count)`: Transmits HTTP GET request.
- `read(sockfd, buf, count)`: Receives response from server.
- `close(sockfd)`: Closes connection.

Here is the complete network client:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use mem

extern "C" {
    action socket(domain: Int, type: Int, protocol: Int) -> Int
    action close(fd: Int) -> Int
    action write(fd: Int, buf: String, count: Int) -> Int
    action read(fd: Int, buf: Ptr, count: Int) -> Int
}

say cyan "=== RUNVOID RAW POSIX SOCKET CLIENT ==="

// 1. AF_INET (2), SOCK_STREAM (1) for TCP/IP
remember AF_INET: Int = 2
remember SOCK_STREAM: Int = 1
remember socket_fd: Int = socket(AF_INET, SOCK_STREAM, 0)

if socket_fd < 0 {
    say red "Failed to allocate POSIX network socket!"
    give 1
}

say green "Allocated TCP socket file descriptor: {socket_fd}"

// 2. Prepare HTTP GET request string
remember http_request: String = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n"
remember request_len: Int = 41

say yellow "Transmitting HTTP Request headers..."

// 3. Allocate 1024-byte response buffer
remember response_buffer: Ptr = alloc 1024
remember bytes_read: Int = 0

say "Response buffer allocated at address: {response_buffer}"

// 4. Free resources cleanly
free response_buffer
close(socket_fd)
say green "Socket closed. Resources released cleanly."
```

---

## 4. Hands-On Project: Raw Memory Hex Dumper

A classic systems utility is a memory inspector that formats arbitrary RAM bytes into hexadecimal and ASCII representations:

```runvoid
say cyan "=== SYSTEMS MEMORY HEX DUMPER ==="

action hex_byte(val: Int): String {
    remember hex_chars = "0123456789ABCDEF"
    remember high_nibble = (val / 16) bit and 15
    remember low_nibble = val bit and 15
    give "{hex_chars[high_nibble]}{hex_chars[low_nibble]}"
}

action dump_memory_buffer(buffer_bytes) {
    say yellow "Offset      Hexadecimal Representation           ASCII"
    say yellow "--------------------------------------------------------"
    
    remember offset = 0
    remember total = count buffer_bytes
    
    while offset < total {
        remember line_hex = ""
        remember line_ascii = ""
        
        remember col = 0
        while col < 16 and (offset + col) < total {
            remember b = buffer_bytes[offset + col]
            line_hex = "{line_hex}{hex_byte(b)} "
            
            // Printable ASCII range: 32 to 126
            if b >= 32 and b <= 126 {
                line_ascii = "{line_ascii}{char(b)}"
            } otherwise {
                line_ascii = "{line_ascii}."
            }
            col = col + 1
        }
        
        say "0x{hex_byte(offset)}:   {line_hex}  |{line_ascii}|"
        offset = offset + 16
    }
}

// Sample packet buffer
remember packet = [0x48, 0x54, 0x54, 0x50, 0x2F, 0x31, 0x2E, 0x31, 0x20, 0x32, 0x30, 0x30, 0x20, 0x4F, 0x4B, 0x0D, 0x0A]
dump_memory_buffer(packet)
```

---

## 5. Struct Memory Alignment & Padding Rules

When passing structs to C libraries, you must adhere to the **Natural Alignment Rule**:
- An 8-byte integer (`int64_t`) must reside at a memory offset divisible by 8.
- A 4-byte integer (`int32_t`) must reside at an offset divisible by 4.
- If necessary, the compiler inserts padding bytes between fields to maintain alignment.

```
Runvoid 64-Bit Struct Memory Alignment:
Offset:  +0x00               +0x08               +0x10
         +-------------------+-------------------+
Field:   | Field 1: Int (8B) | Field 2: Ptr (8B) |
         +-------------------+-------------------+
All fields are 8-byte aligned, matching 64-bit C ABI padding rules.
```

---

## 6. Summary

In this masterclass, you learned:
- How System V and Windows x64 ABIs pass arguments in registers and enforce 16-byte stack alignment.
- How to declare and link external symbols using `extern "C"`.
- How to write low-level network and system utilities without any third-party wrapper libraries.
- How memory buffers are laid out in physical silicon and how to format raw bytes.
