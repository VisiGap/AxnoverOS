; FractureOS Stage 1 Bootloader
; BIOS Boot Sector - loads Stage 2
; Author: FractureOS Team

[BITS 16]
[ORG 0x7C00]

KERNEL_OFFSET equ 0x1000
STAGE2_OFFSET equ 0x7E00

start:
    ; Initialize segments
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    ; Save boot drive
    mov [boot_drive], dl
    
    ; Print boot message
    mov si, msg_boot
    call print_string
    
    ; Load stage 2
    mov si, msg_loading_stage2
    call print_string
    call load_stage2
    
    ; Jump to stage 2
    jmp STAGE2_OFFSET

;===========================================
; Load Stage 2 from disk
;===========================================
load_stage2:
    mov ah, 0x02        ; Read sectors
    mov al, 4           ; Number of sectors to read
    mov ch, 0           ; Cylinder 0
    mov cl, 2           ; Sector 2 (after boot sector)
    mov dh, 0           ; Head 0
    mov dl, [boot_drive]
    mov bx, STAGE2_OFFSET
    int 0x13
    
    jc disk_error
    
    mov si, msg_ok
    call print_string
    ret

disk_error:
    mov si, msg_disk_error
    call print_string
    jmp $

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
; Data
;===========================================
boot_drive:     db 0
msg_boot:       db 'FractureOS Bootloader v1.0', 13, 10, 0
msg_loading_stage2: db 'Loading Stage 2...', 0
msg_ok:         db ' [OK]', 13, 10, 0
msg_disk_error: db ' [DISK ERROR]', 13, 10, 0

; Boot signature
times 510-($-$$) db 0
dw 0xAA55
