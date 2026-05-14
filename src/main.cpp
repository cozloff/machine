#include <db/database.h>
#include <gpu/gpu_test.h>
#include <gpu/rho_guesser.h>
#include <ingest_candidates.h>

#include <CLI/CLI.hpp>

#include <curl/curl.h>

#include <chrono>
#include <cstdio>
#include <string>

namespace {

bool ingestPubChem() {
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
    if (curl == nullptr) {
        std::fprintf(stderr, "Failed to initialize curl.\n");
        curl_global_cleanup();
        sqlite3_close(db);
        return false;
    }

    const bool ingested = ingestCandidates(db, curl);

    curl_easy_cleanup(curl);
    curl_global_cleanup();

    const Clock::time_point end = Clock::now();
    const long long total_ns = std::chrono::duration_cast<Nanoseconds>(end - start).count();

    std::printf("Total ns: %lld\n", total_ns);

    sqlite3_close(db);
    return ingested;
}

}  // namespace

int main(int argc, char** argv) {
    CLI::App app{"machine"};
    app.require_subcommand(1, 1);

    CLI::App* ingest_cmd = app.add_subcommand("ingest", "Ingest candidate compounds from PubChem into sqlite.");
    CLI::App* gpu_cmd = app.add_subcommand("gpu", "Run CUDA helper commands.");
    gpu_cmd->require_subcommand(1, 1);

    CLI::App* gpu_report_cmd = gpu_cmd->add_subcommand("report", "Print CUDA device capabilities.");
    CLI::App* rho_guess_cmd = gpu_cmd->add_subcommand("rho-guess", "Run the CUDA reciprocal-space rho guess demo.");

    if (argc == 1) {
        std::printf("%s", app.help().c_str());
        return 0;
    }

    CLI11_PARSE(app, argc, argv);

    if (ingest_cmd->parsed()) {
        return ingestPubChem() ? 0 : 1;
    }

    if (gpu_report_cmd->parsed()) {
        return runGpuTest() ? 0 : 1;
    }

    if (rho_guess_cmd->parsed()) {
        return runRhoGuessDemo() ? 0 : 1;
    }

    std::fprintf(stderr, "No command selected.\n");
    return 1;
}
