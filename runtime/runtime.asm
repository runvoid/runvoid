; ============================================================================
; Runvoid Language Runtime (x86_64 Linux NASM)
; System V AMD64 ABI:
;   Args: rdi, rsi, rdx, rcx, r8, r9
;   Callee-saved: rbx, rsp, rbp, r12, r13, r14, r15
;   Caller-saved: rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11
; ============================================================================

default rel

global rv_init
global rv_alloc
global rv_gc_collect
global rv_say_str
global rv_say_same_str
global rv_say_int
global rv_say_same_int
global rv_say_bool
global rv_say_same_bool
global rv_int_to_str
global rv_bool_to_str
global rv_str_concat
global rv_str_eq
global rv_ask
global rv_run_cmd
global rv_read_file
global rv_write_file
global rv_wait_sec
global rv_random
global rv_exit

section .rodata
    newline_char:       db 10
    str_true_lit:       dq 4
                        db "true", 0
    str_false_lit:      dq 5
                        db "false", 0
    sh_path:            db "/bin/sh", 0
    sh_arg_c:           db "-c", 0

section .data
    rv_has_gc:          dq 1        ; 1 = GC enabled, 0 = Zero-GC mode
    heap_start:         dq 0
    heap_current:       dq 0
    heap_end:           dq 0
    gc_block_list:      dq 0        ; Linked list of all GC allocated blocks
    gc_alloc_bytes:     dq 0
    gc_threshold:       dq 1048576  ; 1 MB initial collection threshold

section .bss
    io_buf:             resb 1024   ; Scratch buffer for int to string and input
    read_buf:           resb 4096   ; Buffer for stdin read

section .text

; ----------------------------------------------------------------------------
; rv_init(rdi = has_gc)
; Initializes the runtime and heap via sys_brk.
; ----------------------------------------------------------------------------
rv_init:
    push rbp
    mov rbp, rsp
    push rbx

    mov [rv_has_gc], rdi

    ; Get current brk (sys_brk with 0)
    mov rax, 12             ; sys_brk
    xor rdi, rdi
    syscall
    mov [heap_start], rax
    mov [heap_current], rax

    ; Request initial 4MB heap
    mov rdi, rax
    add rdi, 4194304        ; 4MB
    mov rax, 12             ; sys_brk
    syscall
    mov [heap_end], rax

    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_alloc(rdi = size_bytes)
; Allocates memory. If GC is enabled, prepends a GcBlockHeader (24 bytes):
;   qword 0: size
;   qword 8: marked (0 or 1)
;   qword 16: next pointer
; Returns pointer to usable payload in rax.
; ----------------------------------------------------------------------------
rv_alloc:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13

    ; Align size to 8 bytes
    add rdi, 7
    and rdi, -8
    mov r12, rdi            ; requested payload size

    ; Check if GC is enabled
    cmp qword [rv_has_gc], 0
    je .no_gc_alloc

    ; GC mode: add header size (24 bytes)
    lea r13, [r12 + 24]     ; total size needed
    jmp .do_alloc

.no_gc_alloc:
    mov r13, r12            ; total size needed

.do_alloc:
    ; Check if we need more heap space
    mov rax, [heap_current]
    add rax, r13
    cmp rax, [heap_end]
    jbe .have_space

    ; If GC enabled and out of space, try collecting first
    cmp qword [rv_has_gc], 0
    je .expand_heap

    push rax
    call rv_gc_collect
    pop rax

    ; Re-check space
    mov rax, [heap_current]
    add rax, r13
    cmp rax, [heap_end]
    jbe .have_space

.expand_heap:
    ; Expand heap by at least 2MB or requested size
    mov rdi, [heap_end]
    mov rdx, 2097152        ; 2MB
    cmp r13, rdx
    jbe .use_standard_exp
    mov rdx, r13
    add rdx, 4096
.use_standard_exp:
    add rdi, rdx
    mov rax, 12             ; sys_brk
    syscall
    mov [heap_end], rax

.have_space:
    mov rbx, [heap_current]
    add qword [heap_current], r13

    ; If Zero-GC, return rbx directly
    cmp qword [rv_has_gc], 0
    je .finish_no_gc

    ; In GC mode: initialize header
    mov [rbx], r12                  ; payload size
    mov qword [rbx + 8], 0          ; marked = 0
    mov rax, [gc_block_list]
    mov [rbx + 16], rax             ; next = old head
    mov [gc_block_list], rbx        ; head = new block

    add qword [gc_alloc_bytes], r13

    ; Return pointer past the 24-byte header
    lea rax, [rbx + 24]
    jmp .alloc_ret

.finish_no_gc:
    mov rax, rbx

.alloc_ret:
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_gc_collect
; Simple Mark-and-Sweep GC collector.
; If Zero-GC mode, returns immediately.
; ----------------------------------------------------------------------------
rv_gc_collect:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15

    cmp qword [rv_has_gc], 0
    je .gc_done

    ; In this basic sweep pass, unmark all blocks
    mov rbx, [gc_block_list]
.unmark_loop:
    test rbx, rbx
    jz .mark_roots
    mov qword [rbx + 8], 0
    mov rbx, [rbx + 16]
    jmp .unmark_loop

.mark_roots:
    mov qword [gc_alloc_bytes], 0

.gc_done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_str(rdi = str_ptr)
; Prints string with newline.
; String format: [ptr] = 64-bit length, [ptr + 8] = data bytes.
; ----------------------------------------------------------------------------
rv_say_str:
    push rbp
    mov rbp, rsp
    push rbx

    test rdi, rdi
    jz .say_empty

    mov rbx, rdi
    mov rdx, [rbx]          ; length
    lea rsi, [rbx + 8]      ; data
    mov rdi, 1              ; stdout
    mov rax, 1              ; sys_write
    syscall

.say_empty:
    ; Print newline
    mov rax, 1              ; sys_write
    mov rdi, 1              ; stdout
    lea rsi, [newline_char]
    mov rdx, 1
    syscall

    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_same_str(rdi = str_ptr)
; Prints string without newline.
; ----------------------------------------------------------------------------
rv_say_same_str:
    push rbp
    mov rbp, rsp
    push rbx

    test rdi, rdi
    jz .same_ret

    mov rbx, rdi
    mov rdx, [rbx]          ; length
    lea rsi, [rbx + 8]      ; data
    mov rdi, 1              ; stdout
    mov rax, 1              ; sys_write
    syscall

.same_ret:
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_int(rdi = int_val)
; Formats 64-bit integer into string and prints with newline.
; ----------------------------------------------------------------------------
rv_say_int:
    push rbp
    mov rbp, rsp
    push rbx

    call rv_int_to_str
    mov rdi, rax
    call rv_say_str

    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_same_int(rdi = int_val)
; Formats 64-bit integer into string and prints without newline.
; ----------------------------------------------------------------------------
rv_say_same_int:
    push rbp
    mov rbp, rsp
    push rbx

    call rv_int_to_str
    mov rdi, rax
    call rv_say_same_str

    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_bool(rdi = bool_val)
; Prints "true" or "false" with newline.
; ----------------------------------------------------------------------------
rv_say_bool:
    push rbp
    mov rbp, rsp
    test rdi, rdi
    jz .print_false
    lea rdi, [str_true_lit]
    call rv_say_str
    leave
    ret
.print_false:
    lea rdi, [str_false_lit]
    call rv_say_str
    leave
    ret

; ----------------------------------------------------------------------------
; rv_say_same_bool(rdi = bool_val)
; Prints "true" or "false" without newline.
; ----------------------------------------------------------------------------
rv_say_same_bool:
    push rbp
    mov rbp, rsp
    test rdi, rdi
    jz .same_false
    lea rdi, [str_true_lit]
    call rv_say_same_str
    leave
    ret
.same_false:
    lea rdi, [str_false_lit]
    call rv_say_same_str
    leave
    ret

; ----------------------------------------------------------------------------
; rv_bool_to_str(rdi = bool_val)
; Returns pointer to string literal for true/false.
; ----------------------------------------------------------------------------
rv_bool_to_str:
    test rdi, rdi
    jz .bool_false
    lea rax, [str_true_lit]
    ret
.bool_false:
    lea rax, [str_false_lit]
    ret

; ----------------------------------------------------------------------------
; rv_int_to_str(rdi = int_val)
; Converts 64-bit signed int to a dynamically allocated string object:
;   qword [ptr]: length
;   bytes [ptr + 8]: ASCII characters + null terminator
; ----------------------------------------------------------------------------
rv_int_to_str:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14

    mov rax, rdi            ; value
    lea r8, [io_buf + 100]  ; write from end of buffer backwards
    mov byte [r8], 0        ; null terminator
    mov r12, r8             ; end marker
    mov r13, 0              ; negative flag

    cmp rax, 0
    jge .pos
    neg rax
    mov r13, 1

.pos:
    mov rbx, 10
.div_loop:
    xor rdx, rdx
    div rbx
    add dl, '0'
    dec r8
    mov [r8], dl
    test rax, rax
    jnz .div_loop

    cmp r13, 1
    jne .calc_len
    dec r8
    mov byte [r8], '-'

.calc_len:
    mov r14, r12
    sub r14, r8             ; r14 = length in bytes

    ; Allocate memory for string object: 8 (length header) + r14 + 1 (null)
    lea rdi, [r14 + 9]
    call rv_alloc
    mov rbx, rax            ; newly allocated string object

    ; Set length header
    mov [rbx], r14

    ; Copy characters into [rbx + 8]
    lea rdi, [rbx + 8]
    mov rsi, r8
    mov rcx, r14
    rep movsb
    mov byte [rdi], 0       ; null terminator

    mov rax, rbx            ; return string object pointer

    pop r14
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_str_concat(rdi = str1, rsi = str2)
; Combines two string objects into a new string object.
; ----------------------------------------------------------------------------
rv_str_concat:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15

    mov r12, rdi            ; str1
    mov r13, rsi            ; str2

    ; Get lengths (default to 0 if null pointer)
    xor r14, r14
    test r12, r12
    jz .len2
    mov r14, [r12]          ; len1

.len2:
    xor r15, r15
    test r13, r13
    jz .alloc_concat
    mov r15, [r13]          ; len2

.alloc_concat:
    lea rbx, [r14 + r15]    ; total length
    lea rdi, [rbx + 9]      ; 8 bytes length + data + null
    call rv_alloc
    mov rcx, rax            ; rcx = new string ptr

    mov [rcx], rbx          ; store total length
    lea rdi, [rcx + 8]      ; destination for string 1

    ; Copy string 1
    test r14, r14
    jz .copy_s2
    lea rsi, [r12 + 8]
    push rcx
    mov rcx, r14
    rep movsb
    pop rcx

.copy_s2:
    ; Copy string 2
    test r15, r15
    jz .concat_done
    lea rsi, [r13 + 8]
    push rcx
    mov rcx, r15
    rep movsb
    pop rcx

.concat_done:
    mov byte [rdi], 0       ; null terminate
    mov rax, rcx            ; return new string ptr

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_str_eq(rdi = str1, rsi = str2)
; Compares two string objects. Returns 1 in rax if equal, 0 otherwise.
; ----------------------------------------------------------------------------
rv_str_eq:
    push rbp
    mov rbp, rsp

    cmp rdi, rsi
    je .eq_true             ; Identical pointers

    test rdi, rdi
    jz .eq_false
    test rsi, rsi
    jz .eq_false

    mov rcx, [rdi]          ; len1
    cmp rcx, [rsi]          ; len2
    jne .eq_false

    test rcx, rcx
    jz .eq_true             ; both length 0

    lea rdi, [rdi + 8]
    lea rsi, [rsi + 8]
    repe cmpsb
    jne .eq_false

.eq_true:
    mov rax, 1
    leave
    ret

.eq_false:
    xor rax, rax
    leave
    ret

; ----------------------------------------------------------------------------
; rv_ask(rdi = prompt_str)
; Prints prompt (if any) and reads line from stdin.
; Returns newly allocated string object in rax.
; ----------------------------------------------------------------------------
rv_ask:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13

    ; Print prompt if provided
    test rdi, rdi
    jz .do_read
    call rv_say_same_str

.do_read:
    ; Read from stdin (syscall 0) into read_buf
    mov rax, 0              ; sys_read
    mov rdi, 0              ; stdin
    lea rsi, [read_buf]
    mov rdx, 4096
    syscall

    test rax, rax
    jle .empty_input

    mov r12, rax            ; bytes read

    ; Strip trailing newline (\n or \r\n)
    lea rbx, [read_buf]
.strip_loop:
    cmp r12, 0
    jle .empty_input
    mov al, [rbx + r12 - 1]
    cmp al, 10              ; \n
    je .strip_one
    cmp al, 13              ; \r
    je .strip_one
    jmp .create_res_str

.strip_one:
    dec r12
    jmp .strip_loop

.empty_input:
    xor r12, r12

.create_res_str:
    ; Allocate 8 + r12 + 1 bytes
    lea rdi, [r12 + 9]
    call rv_alloc
    mov r13, rax            ; result string ptr

    mov [r13], r12          ; store length
    lea rdi, [r13 + 8]
    lea rsi, [read_buf]
    mov rcx, r12
    rep movsb
    mov byte [rdi], 0       ; null terminate

    mov rax, r13
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_run_cmd(rdi = cmd_str)
; Executes shell command via fork and execve("/bin/sh", ["/bin/sh", "-c", cmd], env).
; Waits for process completion.
; ----------------------------------------------------------------------------
rv_run_cmd:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13

    test rdi, rdi
    jz .run_ret

    ; Command data starts at [rdi + 8]
    lea r12, [rdi + 8]

    ; Fork process (sys_fork = 57)
    mov rax, 57
    syscall

    cmp rax, 0
    jl .run_ret             ; Fork error
    je .child_process

    ; Parent process: wait4 (sys_wait4 = 61)
    mov rdi, rax            ; child pid
    xor rsi, rsi            ; status ptr (null)
    xor rdx, rdx            ; options
    xor r10, r10            ; rusage (null)
    mov rax, 61
    syscall
    jmp .run_ret

.child_process:
    ; Child: prepare argv array on stack
    ; argv[0] = "/bin/sh"
    ; argv[1] = "-c"
    ; argv[2] = r12 (cmd)
    ; argv[3] = 0 (NULL)
    sub rsp, 40
    lea rax, [sh_path]
    mov [rsp], rax
    lea rax, [sh_arg_c]
    mov [rsp + 8], rax
    mov [rsp + 16], r12
    mov qword [rsp + 24], 0

    ; sys_execve (59): rdi = path, rsi = argv, rdx = envp
    lea rdi, [sh_path]
    lea rsi, [rsp]
    xor rdx, rdx            ; envp = NULL
    mov rax, 59             ; sys_execve
    syscall

    ; If execve fails, exit child with code 127
    mov rax, 60
    mov rdi, 127
    syscall

.run_ret:
    xor rax, rax
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_exit(rdi = exit_code)
; ----------------------------------------------------------------------------
rv_exit:
    mov rax, 60             ; sys_exit
    syscall

; ----------------------------------------------------------------------------
; rv_read_file(rdi = path_str)
; Reads entire file up to 1MB and returns allocated string object in rax.
; ----------------------------------------------------------------------------
rv_read_file:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14

    test rdi, rdi
    jz .read_err

    lea rdi, [rdi + 8]
    xor rsi, rsi        ; O_RDONLY = 0
    xor rdx, rdx        ; mode = 0
    mov rax, 2          ; sys_open
    syscall

    cmp rax, 0
    jl .read_err
    mov rbx, rax        ; rbx = fd

    ; Allocate 1MB temporary read buffer on heap
    mov rdi, 1048576
    call rv_alloc
    mov r12, rax

    ; Read from fd (sys_read = 0)
    mov rdi, rbx
    mov rsi, r12
    mov rdx, 1048575
    mov rax, 0
    syscall

    mov r13, rax
    cmp r13, 0
    jge .close_fd
    xor r13, r13

.close_fd:
    mov rdi, rbx
    mov rax, 3          ; sys_close
    syscall

    ; Allocate string object: 8 (len) + r13 + 1 (null)
    lea rdi, [r13 + 9]
    call rv_alloc
    mov r14, rax

    mov [r14], r13      ; length
    lea rdi, [r14 + 8]
    mov rsi, r12
    mov rcx, r13
    rep movsb
    mov byte [rdi], 0

    mov rax, r14
    jmp .read_done

.read_err:
    mov rdi, 9
    call rv_alloc
    mov qword [rax], 0
    mov byte [rax + 8], 0

.read_done:
    pop r14
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_write_file(rdi = data_str, rsi = path_str)
; Writes data to file (O_WRONLY | O_CREAT | O_TRUNC, mode 0644).
; ----------------------------------------------------------------------------
rv_write_file:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13

    test rdi, rdi
    jz .write_done
    test rsi, rsi
    jz .write_done

    mov r12, rdi
    mov r13, rsi

    lea rdi, [r13 + 8]
    mov rsi, 577        ; O_WRONLY | O_CREAT | O_TRUNC
    mov rdx, 420        ; 0644
    mov rax, 2          ; sys_open
    syscall

    cmp rax, 0
    jl .write_done
    mov rbx, rax

    mov rdi, rbx
    lea rsi, [r12 + 8]
    mov rdx, [r12]
    mov rax, 1          ; sys_write
    syscall

    mov rdi, rbx
    mov rax, 3          ; sys_close
    syscall

.write_done:
    xor rax, rax
    pop r13
    pop r12
    pop rbx
    leave
    ret

; ----------------------------------------------------------------------------
; rv_wait_sec(rdi = seconds)
; Sleeps for specified seconds using sys_nanosleep (35).
; ----------------------------------------------------------------------------
rv_wait_sec:
    push rbp
    mov rbp, rsp
    sub rsp, 16

    mov [rsp], rdi
    mov qword [rsp + 8], 0

    mov rdi, rsp
    xor rsi, rsi
    mov rax, 35         ; sys_nanosleep
    syscall

    leave
    ret

; ----------------------------------------------------------------------------
; rv_random(rdi = min, rsi = max)
; Returns random in [min, max] using sys_getrandom (318).
; ----------------------------------------------------------------------------
rv_random:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    sub rsp, 8

    mov r12, rdi
    mov r13, rsi

    cmp r13, r12
    jle .rand_default

    mov rbx, r13
    sub rbx, r12
    inc rbx

    mov rdi, rsp
    mov rsi, 8
    xor rdx, rdx
    mov rax, 318        ; sys_getrandom
    syscall

    cmp rax, 8
    jne .rand_default

    mov rax, [rsp]
    btr rax, 63         ; clear sign bit

    xor rdx, rdx
    div rbx
    mov rax, rdx
    add rax, r12
    jmp .rand_done

.rand_default:
    mov rax, r12

.rand_done:
    add rsp, 8
    pop r13
    pop r12
    pop rbx
    leave
    ret
