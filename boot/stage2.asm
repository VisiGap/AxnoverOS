; FractureOS Stage 2 Bootloader
; Loads kernel and enters long mode (64-bit)

[BITS 16]
[ORG 0x7E00]

KERNEL_OFFSET equ 0x100000

stage2_start:
    mov si, msg_stage2
    call print_string
    
    ; Check for long mode support
    call check_long_mode
    
    ; Enable A20 line
    mov si, msg_a20
    call print_string
    call enable_a20
    mov si, msg_ok
    call print_string
    
    ; Load kernel
    mov si, msg_loading_kernel
    call print_string
    call load_kernel
    
    ; Setup paging
    mov si, msg_paging
    call print_string
    call setup_paging
    mov si, msg_ok
    call print_string
    
    ; Enter protected mode
    mov si, msg_protected
    call print_string
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp CODE_SEG:protected_mode

;===========================================
; Check for long mode (CPUID)
;===========================================
check_long_mode:
    pushad
    
    ; Check if CPUID is supported
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    xor eax, ecx
    jz .no_long_mode
    
    ; Check for extended CPUID
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode
    
    ; Check for long mode
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    
    popad
    ret

.no_long_mode:
    mov si, msg_no_long_mode
    call print_string
    jmp $

;===========================================
; Enable A20 line
;===========================================
enable_a20:
    ; Try BIOS method first
    mov ax, 0x2401
    int 0x15
    jnc .done
    
    ; Try keyboard controller method
    call .wait_input
    mov al, 0xAD
    out 0x64, al
    
    call .wait_input
    mov al, 0xD0
    out 0x64, al
    
    call .wait_output
    in al, 0x60
    push ax
    
    call .wait_input
    mov al, 0xD1
    out 0x64, al
    
    call .wait_input
    pop ax
    or al, 2
    out 0x60, al
    
    call .wait_input
    mov al, 0xAE
    out 0x64, al
    
    call .wait_input
    
.done:
    ret

.wait_input:
    in al, 0x64
    test al, 2
    jnz .wait_input
    ret

.wait_output:
    in al, 0x64
    test al, 1
    jz .wait_output
    ret

;===========================================
; Load kernel from disk
;===========================================
load_kernel:
    mov ah, 0x02        ; Read sectors
    mov al, 64          ; Number of sectors (32KB kernel)
    mov ch, 0           ; Cylinder 0
    mov cl, 6           ; Sector 6 (after boot + stage2)
    mov dh, 0           ; Head 0
    mov dl, [0x7C00 + boot_drive - start]
    mov bx, KERNEL_OFFSET
    int 0x13
    
    jc .error
    mov si, msg_ok
    call print_string
    ret

.error:
    mov si, msg_disk_error
    call print_string
    jmp $

;===========================================
; Setup identity paging for first 2MB
;===========================================
setup_paging:
    ; Clear page tables
    mov edi, 0x1000
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, cr3
    
    ; Setup PML4
    mov dword [edi], 0x2003
    add edi, 0x1000
    
    ; Setup PDPT
    mov dword [edi], 0x3003
    add edi, 0x1000
    
    ; Setup PD
    mov dword [edi], 0x4003
    add edi, 0x1000
    
    ; Setup PT (identity map first 2MB)
    mov ebx, 0x00000003
    mov ecx, 512
.set_entry:
    mov dword [edi], ebx
    add ebx, 0x1000
    add edi, 8
    loop .set_entry
    
    ret

;===========================================
; Print string (SI = string pointer)
;===========================================
print_string:
    pusha
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    mov bh, 0
    int 0x10
    jmp .loop
.done:
    popa
    ret

;===========================================
; Protected Mode (32-bit)
;===========================================
[BITS 32]
protected_mode:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    
    ; Enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    
    ; Load CR3 with PML4
    mov eax, 0x1000
    mov cr3, eax
    
    ; Enable long mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    
    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    
    ; Load 64-bit GDT
    lgdt [gdt64_descriptor]
    jmp CODE64_SEG:long_mode

;===========================================
; Long Mode (64-bit)
;===========================================
[BITS 64]
long_mode:
    ; Setup segments
    mov ax, DATA64_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Jump to kernel
    mov rax, KERNEL_OFFSET
    jmp rax

;===========================================
; GDT for 32-bit protected mode
;===========================================
gdt_start:
    dq 0x0000000000000000    ; Null descriptor

gdt_code:
    dq 0x00CF9A000000FFFF    ; Code segment

gdt_data:
    dq 0x00CF92000000FFFF    ; Data segment

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

;===========================================
; GDT for 64-bit long mode
;===========================================
gdt64_start:
    dq 0x0000000000000000    ; Null descriptor

gdt64_code:
    dq 0x00209A0000000000    ; 64-bit code segment

gdt64_data:
    dq 0x0000920000000000    ; 64-bit data segment

gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64_start - 1
    dd gdt64_start

CODE64_SEG equ gdt64_code - gdt64_start
DATA64_SEG equ gdt64_data - gdt64_start

;===========================================
; Data
;===========================================
msg_stage2:         db 'Stage 2 Bootloader', 13, 10, 0
msg_a20:            db 'Enabling A20...', 0
msg_loading_kernel: db 'Loading kernel...', 0
msg_paging:         db 'Setting up paging...', 0
msg_protected:      db 'Entering protected mode...', 13, 10, 0
msg_ok:             db ' [OK]', 13, 10, 0
msg_disk_error:     db ' [DISK ERROR]', 13, 10, 0
msg_no_long_mode:   db 'ERROR: 64-bit long mode not supported!', 13, 10, 0

; Pad to 4 sectors (2048 bytes)
times 2048-($-$$) db 0
