#include <unistd.h>
#include <chrono>
#include <cstdint>
#include <cstdio>

int main() {
    using Clock = std::chrono::steady_clock;
    using Nanoseconds = std::chrono::nanoseconds;

    const Clock::time_point start = Clock::now();

    int a = 1 + 1;

    const Clock::time_point end = Clock::now();

    const double total_ns = std::chrono::duration_cast<Nanoseconds>(
        end - start
    ).count();

    char buffer[128];

    const int len = std::snprintf(
        buffer,
        sizeof(buffer),
        "Total ns: %lld\n",
        static_cast<long long>(total_ns)
    );

    write(STDOUT_FILENO, buffer, static_cast<std::size_t>(len));
    

    return 0;
}