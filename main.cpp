#include <unistd.h>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <sqlite3.h>

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

    sqlite3* db = nullptr;
    char* err_msg = nullptr;

    if (sqlite3_open("data/machine.db", &db) != SQLITE_OK) {
        std::snprintf(
            buffer,
            sizeof(buffer),
            "SQLite open failed: %s\n",
            sqlite3_errmsg(db)
        );
        write(STDERR_FILENO, buffer, std::strlen(buffer));
        sqlite3_close(db);
        return 1;
    }

    const char* setup_sql =
        "CREATE TABLE IF NOT EXISTS smoke_test ("
        "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        "created_at TEXT DEFAULT CURRENT_TIMESTAMP"
        ");"
        "INSERT INTO smoke_test DEFAULT VALUES;";

    if (sqlite3_exec(db, setup_sql, nullptr, nullptr, &err_msg) != SQLITE_OK) {
        std::snprintf(
            buffer,
            sizeof(buffer),
            "SQLite exec failed: %s\n",
            err_msg
        );
        write(STDERR_FILENO, buffer, std::strlen(buffer));
        sqlite3_free(err_msg);
        sqlite3_close(db);
        return 1;
    }

    sqlite3_stmt* stmt = nullptr;
    if (sqlite3_prepare_v2(
            db,
            "SELECT COUNT(*) FROM smoke_test;",
            -1,
            &stmt,
            nullptr
        ) != SQLITE_OK) {
        std::snprintf(
            buffer,
            sizeof(buffer),
            "SQLite prepare failed: %s\n",
            sqlite3_errmsg(db)
        );
        write(STDERR_FILENO, buffer, std::strlen(buffer));
        sqlite3_close(db);
        return 1;
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        const int row_count = sqlite3_column_int(stmt, 0);
        const int sqlite_len = std::snprintf(
            buffer,
            sizeof(buffer),
            "SQLite smoke_test rows: %d\n",
            row_count
        );
        write(STDOUT_FILENO, buffer, static_cast<std::size_t>(sqlite_len));
    }

    sqlite3_finalize(stmt);
    sqlite3_close(db);

    return 0;
}
