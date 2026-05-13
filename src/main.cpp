#include <db/database.h>
#include <ingest_candidates.h>

#include <curl/curl.h>

#include <chrono>
#include <cstdio>

int main() {
    using Clock = std::chrono::steady_clock;
    using Nanoseconds = std::chrono::nanoseconds;

    const Clock::time_point start = Clock::now();

    sqlite3* db = nullptr;
    if (!open_database("data/machine.db", &db)) {
        return 1;
    }

    if (!initialize_schema(db)) {
        sqlite3_close(db);
        return 1;
    }

    curl_global_init(CURL_GLOBAL_DEFAULT);
    CURL* curl = curl_easy_init();

    // Ingest candidates into sqlite database
    const bool ingested = ingestCandidates(db, curl);

    curl_easy_cleanup(curl);
    curl_global_cleanup();

    const Clock::time_point end = Clock::now();
    const long long total_ns = std::chrono::duration_cast<Nanoseconds>(end - start).count();

    std::printf("Total ns: %lld\n", total_ns);

    sqlite3_close(db);
    return ingested ? 0 : 1;
}
