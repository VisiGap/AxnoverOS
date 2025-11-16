#ifndef FRACTURE_SIGNAL_H
#define FRACTURE_SIGNAL_H

#include "types.h"
#include "syscall.h"

namespace fracture {
namespace signal {

enum class Signal : uint32_t {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
};

using SignalHandler = void (*)(int);

constexpr SignalHandler SIG_DFL = nullptr;
inline SignalHandler SIG_IGN = reinterpret_cast<SignalHandler>(1);

class SignalManager {
public:
    static int kill(int32_t pid, Signal sig) {
        // TODO: Add proper kill syscall
        return 0;
    }
    
    static SignalHandler signal(Signal sig, SignalHandler handler) {
        // TODO: Implement signal handler registration
        return SIG_DFL;
    }
    
    static int raise(Signal sig) {
        int32_t pid = syscall::syscall0(syscall::SyscallNumber::GETPID);
        return kill(pid, sig);
    }
};

} // namespace signal
} // namespace fracture

#endif // FRACTURE_SIGNAL_H
