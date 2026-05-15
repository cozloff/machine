#include <gpu/gpu_test.h>
#include <gpu/rho_guesser.h>
#include <ingest/pubchem.h>

#include <CLI/CLI.hpp>

#include <cstdio>

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
