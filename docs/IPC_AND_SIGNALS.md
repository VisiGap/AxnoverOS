# FractureOS IPC 和信号系统

## ✅ 已实现功能

### 1. 消息传递 IPC (Message-Passing IPC)

**文件**: `kernel/src/ipc.rs`

**功能**:
- ✅ 进程间消息队列
- ✅ 同步消息发送/接收
- ✅ 消息类型支持
- ✅ 队列管理

**数据结构**:
```rust
pub struct Message {
    pub sender: Pid,
    pub receiver: Pid,
    pub data: Vec<u8>,
    pub msg_type: MessageType,
}

pub enum MessageType {
    Data,
    Signal,
    Request,
    Response,
}
```

**API**:
```rust
// 发送消息
pub fn send_message(sender: Pid, receiver: Pid, data: &[u8]) -> Result<(), IpcError>

// 接收消息
pub fn receive_message(pid: Pid) -> Option<Message>

// 检查是否有消息
pub fn has_messages(pid: Pid) -> bool
```

**特性**:
- 最大消息大小: 4KB
- 每个进程队列容量: 64 条消息
- 自动队列管理
- 进程注册/注销

---

### 2. 共享内存 (Shared Memory)

**文件**: `kernel/src/shm.rs`

**功能**:
- ✅ 共享内存段创建
- ✅ 进程附加/分离
- ✅ 权限控制
- ✅ 生命周期管理

**数据结构**:
```rust
pub struct SharedMemory {
    id: ShmId,
    owner: Pid,
    size: usize,
    address: VirtAddr,
    attached_processes: Vec<(Pid, ShmPermissions)>,
}

pub struct ShmPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}
```

**API**:
```rust
// 创建共享内存
pub fn create(owner: Pid, size: usize) -> Result<ShmId, ShmError>

// 附加到共享内存
pub fn attach(id: ShmId, pid: Pid, perms: ShmPermissions) -> Result<VirtAddr, ShmError>

// 分离共享内存
pub fn detach(id: ShmId, pid: Pid) -> Result<(), ShmError>

// 删除共享内存
pub fn delete(id: ShmId, pid: Pid) -> Result<(), ShmError>
```

**特性**:
- 最大段大小: 16MB
- 权限控制: 读/写/执行
- 所有者管理
- 引用计数

---

### 3. 信号系统 (Signals)

**文件**: `kernel/src/signal.rs`

**功能**:
- ✅ POSIX 风格信号
- ✅ 信号处理器注册
- ✅ 信号阻塞/解除阻塞
- ✅ 待处理信号队列

**支持的信号**:
```rust
pub enum Signal {
    SIGHUP = 1,     // 挂起
    SIGINT = 2,     // 中断 (Ctrl+C)
    SIGQUIT = 3,    // 退出
    SIGILL = 4,     // 非法指令
    SIGTRAP = 5,    // 跟踪陷阱
    SIGABRT = 6,    // 中止
    SIGBUS = 7,     // 总线错误
    SIGFPE = 8,     // 浮点异常
    SIGKILL = 9,    // 强制终止 (不可捕获)
    SIGUSR1 = 10,   // 用户定义信号 1
    SIGSEGV = 11,   // 段错误
    SIGUSR2 = 12,   // 用户定义信号 2
    SIGPIPE = 13,   // 管道破裂
    SIGALRM = 14,   // 定时器
    SIGTERM = 15,   // 终止
    SIGCHLD = 17,   // 子进程状态改变
    SIGCONT = 18,   // 继续
    SIGSTOP = 19,   // 停止 (不可捕获)
    SIGTSTP = 20,   // 终端停止
}
```

**信号动作**:
```rust
pub enum SignalAction {
    Default,           // 默认动作
    Ignore,            // 忽略
    Handler(u64),      // 自定义处理器
}
```

**API**:
```rust
// 发送信号
pub fn send_signal(target: Pid, signal: Signal, sender: Option<Pid>)

// 设置信号处理器
pub fn set_handler(pid: Pid, signal: Signal, action: SignalAction)

// 处理待处理信号
pub fn process_signals(pid: Pid) -> Option<(Signal, SignalAction)>
```

---

### 4. 系统调用扩展

**新增系统调用**:

```rust
// IPC 消息传递
SYS_SEND    // 发送消息
SYS_RECEIVE // 接收消息

// 信号
SYS_KILL    // 发送信号
SYS_SIGNAL  // 设置信号处理器

// 共享内存
SYS_SHMGET  // 创建共享内存
SYS_SHMAT   // 附加共享内存
SYS_SHMDT   // 分离共享内存
SYS_SHMCTL  // 控制共享内存
```

**SYSCALL/SYSRET 支持**:
- ✅ 快速系统调用接口
- ✅ MSR 寄存器配置
- ✅ 用户/内核栈切换
- ✅ 寄存器保存/恢复

---

## 使用示例

### IPC 消息传递

**发送消息**:
```cpp
#include "ipc.h"

// 发送消息到进程 42
const char* msg = "Hello, Process 42!";
fracture::ipc::IPC::send(42, msg, strlen(msg));
```

**接收消息**:
```cpp
#include "ipc.h"

// 接收消息
char buffer[4096];
ssize_t len = fracture::ipc::IPC::receive(buffer, sizeof(buffer));
if (len > 0) {
    // 处理消息
}
```

### 共享内存

**创建和使用**:
```rust
use fracture_kernel::shm;

// 进程 A: 创建共享内存
let shm_id = shm::create(pid_a, 4096)?;

// 进程 B: 附加到共享内存
let addr = shm::attach(shm_id, pid_b, ShmPermissions::READ_WRITE)?;

// 使用共享内存...

// 分离
shm::detach(shm_id, pid_b)?;
```

### 信号处理

**发送信号**:
```cpp
#include "signal.h"

// 发送 SIGTERM 到进程 42
fracture::signal::SignalManager::kill(42, fracture::signal::Signal::SIGTERM);
```

**设置信号处理器**:
```cpp
void my_handler(int sig) {
    // 处理信号
}

// 注册处理器
fracture::signal::SignalManager::signal(
    fracture::signal::Signal::SIGINT,
    my_handler
);
```

---

## 性能特性

### IPC
- **消息发送**: O(1)
- **消息接收**: O(1)
- **队列查找**: O(n) - n 为进程数

### 共享内存
- **创建**: O(1)
- **附加**: O(1)
- **访问**: O(1) - 直接内存访问

### 信号
- **发送**: O(1)
- **处理**: O(1)
- **查找**: O(n) - n 为进程数

---

## 安全特性

### IPC
- ✅ 消息大小限制
- ✅ 队列容量限制
- ✅ 进程隔离

### 共享内存
- ✅ 权限检查
- ✅ 所有者验证
- ✅ 引用计数

### 信号
- ✅ 不可捕获信号 (SIGKILL, SIGSTOP)
- ✅ 信号阻塞
- ✅ 权限检查

---

## 限制和约束

### IPC
- 最大消息大小: 4KB
- 队列容量: 64 条消息/进程
- 同步传递（阻塞）

### 共享内存
- 最大段大小: 16MB
- 虚拟地址范围: 0x8000_0000_0000+
- 需要手动同步

### 信号
- 最多 20 种信号
- 待处理队列无限制
- 不支持实时信号

---

## 未来改进

### 短期
- [ ] 异步 IPC
- [ ] 共享内存同步原语
- [ ] 更多信号类型

### 中期
- [ ] 消息优先级
- [ ] 共享内存池
- [ ] 信号队列

### 长期
- [ ] 分布式 IPC
- [ ] 远程共享内存
- [ ] 实时信号

---

## 测试

```bash
cd kernel
cargo test --lib ipc
cargo test --lib signal
cargo test --lib shm
```

---

**最后更新**: 2025-11-16  
**版本**: 0.2.0  
**状态**: Phase 2 完成 ✅
