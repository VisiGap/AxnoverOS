# FractureOS Boot System

## 概述

FractureOS 使用两阶段 BIOS 引导系统，从 16 位实模式过

## 引导流程

```

```

### 阶段详解

1. **BIOS 启动**
   - BIOS 加载第一个扇区（512 字节）到 0x7C00
   - 检查引导签名 0xAA55

2. **Stage 1 Boot
器
   - 保存启动驱动器号
   tage 2
   - 跳转到 Stage 2

3. .asm`)

   - 启用 A20 地址线

   - 设置页表（恒等映射前 2MB）
位）
   - 启用 PAE 和长模式
   - 跳转到内核

4. **Kernel** (64-bit)
   - 内核在 64 位长模式下运行
   - 地址：0x100000 (1MB)

---



阶段内存映射

```
0x00000000 - 0x000003FF  
0x00000400 - 0x000004FF  BIOS 数据区 (BDA
0x00000500 - 0x00007BFF
0x0)

0x00009000 - 0x0
0x0009
0x000A0000 - 0x000FFFFF  视频内存 + BIOS RO
0x00100000 - ...         内核加载地址 (1MB+)
```

e 2 设置）

```
0x00001000  PML4 (Page Map Level 4
0x00002000  PDPT (Page Directable)
0x00003000  PD   (Pary)
0x0le)
``

---

## 磁盘布局

```
(512B)
扇区 1-4:    Stage 2 Bootl2KB)
扇区 Kernel
```

---

## Stage 1 功能

**文件**: `boot/boot.asm`

**大小**: 512 字节（1 扇区）

**功
寄存器和栈
- ✅ 保存 BIOS 提供的启动驱动器号

- ✅）
- ✅ 错误处理
- ✅ 跳转到 Stage 2

**关键代码**:
```asm
; 加载 Stage 2
mov ah, 0x02        ; BIOS 读扇
mov al, 4           ; 读取 4 个扇区
mov ch, 0           ; 柱面 0
mov cl, 2           ; 扇区 2（从 1 开始计数）
mov dh, 0           ; 磁头 0
mov
; 目标地址
int 0x13            ; B
`

---

## Stage 2 功能

**文件**: `boot/stage2.asm`

**大小**: 2048 字节）

**功能**:
- ✅PUID）
持检查
- ✅ A20 地址线启用（多种方法
- ✅ 内核加载（64 扇区 = 32KB）
- ✅ 页表设置（4 级页表）
- ✅ GDT 设置（32 
- ✅ 模式切换（16→32位）

 启用

：
1. BIOS 方法（INT 0x15, AX=0x240
2. 
3. Fast A20 方法（端口 0x92）

### 长模式检查

`asm
; 检查 CPUID 支持
mov0
cpuid
cmp eax, 0x80000001
jb .no_long_mode

位
mov eax, 0x80000001
id
test edx, 1 << 2 LM 位
jz .no_long_mode
```

### 页表设置

恒等映射前 2MB 内存：
```asm
; PML4[0] → PDPT
mov dword [0x1000],

; PDPT[0] → PD
3

; P→ PT
mov dword [0x3000], 0x4003

; PT[0-511] → 物理B）
mov ebx, 0x00000003  ;  Writable
mov ecx, 512
.loop:
x
    add ebxx1000

    loop .loop
```

### 换

**1
```asm
lgdt [gdt_dr]
mov eax, cr0
or eax, 1

jmp CODE_SEG:protecode
```

**32
```asm
; 启PAE
, cr4
or eax, 1 << 5
mov cr4, eax

; 加载 PML4
mov eaxx1000
mov cr3, eax

; 启用长模式
mov ecx, 0xC2_EFER MSR
rdmsr
位
wrmsr

; 启用分页
mov eax, cr0
or eax, 1 << 31
moveax

; 跳转到 64 位码

jmp CODE64_SEG:g_mode
```

---

## 构

### 32 位 GDT

```
Offs   Flags
0x00    Null            0       0           -
0x08    Code            0       0xFFFFF     Executable, Readable
0x1table


### 64 

```
Offset  Segment         Flags
0x00    Null            -
0x0le
able
```

---

## 构建和测试

er

```bash
# 构建两个阶段
t

# 输出文件
# build/boot.bin    - St
# build/stage2.bin )
```

### 创建磁盘镜像

```bash
引导镜像
./tools/create-image.sh

# 输g
```

### 在 QEMU 中测试

```bash
# 运行镜
qemu-systmg

# 带串口输出
qemu-system-x86_64 -dr \
         dio

# 调试模式
qemu-system-x86_6
                  
```

---

## 调试技巧

### 使用 QEMU Monitor

```bash
# 启动时按 Ctrl+Alt+2 进入 monitor
(qemu) info registers
(qe/10i $pc
x7c00
```

### 使用 GDB

```bash
动 QEMU
qemmg

# 终端 2: 连接 GDB
就绪 ✅
生产: *状态****: 1.0  
*-16  
**版本5-11最后更新**: 202
**
ll)

---errupt_caIOS_int.org/wiki/Biaen.wikipedhttps://lls](Interrupt Cal)
- [BIOS sdm.htmles/intel-evelop/artic/www/us/en/dntentcotel.com/in/software.l](https:/oper's Manuae Develoftwaritectures S2 ArchA-3and I[Intel 64 loader)
- g/Bootev.orki.osd/wier](https:/ - Bootload[OSDev Wiki考资料

- 

## 参全引导

--- 安）
- [ ]] 网络引导（PXE菜单
- [ 形化引导 长期
- [ ] 图持

###- [ ] 压缩内核支boot2）
Multi持（- [ ] 多引导支] UEFI 支持
期
- [ 

### 中测（E820）
- [ ] 内存映射检A 扩展）支持更大的内核（LB]  [ 误检查
-[ ] 添加更多错- 期


### 短 未来改进

##6KB

---: 1- 页表2048 字节
  Stage 2: 
  -  字节1: 512age *: 
  - St用* **内存占（不含内核）
-ms: < 10*总启动时间**s
- *加载时间**: < 5m- **Stage 2  1ms
 1 加载时间**: < **Stage

--

## 性能特性试

--B 单步调 GD确
- 使用 验证页表设置是否正 是否成功启用
-*:
- 检查 A20*A*e 2
*卡在 Stag
### Q: 86-64
支持 xCPU  在真实硬件上，确保 
-tem-x86_64`emu-sys的 QEMU: `q使用支持 64 位- *A**: 
"
* supportedode notng mt loQ: "64-bi取参数

###  验证 BIOS 磁盘读区位置
- 在正确的扇 2- 确保 Stage镜像是否正确创建
*: 
- 检查磁盘
**A*消息" "DISK ERROR
### Q: 5）
否正确（0xAA5引导签名是: 检查
**A**黑屏动时# Q: 启问题

##--

## 常见
```

- aa显示: 55mg
# 应该.ifractureos build/0 -l 2
xxd -s 51

# 检查引导签名ureos.imgractild/fl 512 bu
xxd -前 512 字节bash
# 查看导扇区

```查看引
### `
nue
``b) conti7c00
(gd0xak *db) bre
(gi8086re rchitectugdb) set aote :1234
(et rem
(gdb) targgdb