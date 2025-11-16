#ifndef FRACTURE_IPC_H
#define FRACTURE_IPC_H

#include "types.h"
#include "syscall.h"

namespace fracture {
namespace ipc {

constexpr size_t MAX_MESSAGE_SIZE = 4096;

enum class MessageType : uint32_t {
    DATA = 0,
    SIGNAL = 1,
    REQUEST = 2,
    RESPONSE = 3,
};

class Message {
public:
    Message() : sender_(0), receiver_(0), type_(MessageType::DATA), size_(0) {}
    
    int32_t sender() const { return sender_; }
    int32_t receiver() const { return receiver_; }
    MessageType type() const { return type_; }
    size_t size() const { return size_; }
    const uint8_t* data() const { return data_; }
    
private:
    int32_t sender_;
    int32_t receiver_;
    MessageType type_;
    size_t size_;
    uint8_t data_[MAX_MESSAGE_SIZE];
};

class IPC {
public:
    static ssize_t send(int32_t receiver, const void* data, size_t size) {
        return syscall::syscall3(
            syscall::SyscallNumber::MMAP,
            receiver,
            reinterpret_cast<uint64_t>(data),
            size
        );
    }
    
    static ssize_t receive(void* buffer, size_t size) {
        return syscall::syscall3(
            syscall::SyscallNumber::MUNMAP,
            reinterpret_cast<uint64_t>(buffer),
            size,
            0
        );
    }
    
    static bool has_messages() {
        // TODO: Implement check syscall
        return false;
    }
};

} // namespace ipc
} // namespace fracture

#endif // FRACTURE_IPC_H
