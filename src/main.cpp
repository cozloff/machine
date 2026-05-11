#include "db/database.h"
#include "pubchem.h"

#include <curl/curl.h>

#include <chrono>
#include <cstdio>
#include <vector>

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

    const std::vector<Candidate> candidates = {
        {"valproic acid", "valproic acid", "HDAC inhibitor", "Common small-molecule reprogramming enhancer"},
        {"CHIR99021", "CHIR99021", "GSK3 inhibitor", "Component of chemical reprogramming cocktails"},
        {"RepSox", "RepSox", "TGF-beta receptor inhibitor", "Often used as a reprogramming enhancer"},
        {"tranylcypromine", "tranylcypromine", "LSD1/KDM1A inhibitor", "Epigenetic modifier used in reprogramming contexts"},
        {"forskolin", "forskolin", "adenylyl cyclase activator", "Component of VC6TFZ-style chemical reprogramming cocktails"},
        {"DZNep", "3-Deazaneplanocin A", "EZH2/PRC2 pathway inhibitor", "Epigenetic small molecule used in reprogramming studies"},
        {"5-azacytidine", "5-azacytidine", "DNA methyltransferase inhibitor", "Epigenetic modifier relevant to cell-state resetting"},
        {"RG108", "RG108", "DNA methyltransferase inhibitor", "Non-nucleoside DNMT inhibitor"},
        {"sodium butyrate", "sodium butyrate", "HDAC inhibitor", "Common chromatin-opening reprogramming enhancer"},
        {"BIX-01294", "BIX-01294", "G9a/EHMT2 inhibitor", "Histone methyltransferase inhibitor used in reprogramming studies"},
        {"TTNPB", "TTNPB", "retinoic acid receptor agonist", "Used in some chemical reprogramming combinations"},
        {"SB431542", "SB431542", "TGF-beta receptor inhibitor", "Small-molecule pathway modulator used in stem-cell workflows"}
    };

    curl_global_init(CURL_GLOBAL_DEFAULT);
    CURL* curl = curl_easy_init();
    if (curl == nullptr) {
        std::fprintf(stderr, "Failed to initialize libcurl\n");
        sqlite3_close(db);
        curl_global_cleanup();
        return 1;
    }

    int stored = 0;
    for (const Candidate& candidate : candidates) {
        CompoundProperties properties;
        if (!fetch_pubchem_properties(curl, candidate.pubchem_query, &properties)) {
            std::fprintf(stderr, "Skipping %s\n", candidate.name);
            continue;
        }

        if (upsert_candidate(db, candidate, properties)) {
            ++stored;
            std::printf("Stored %-18s PubChem CID %s\n", candidate.name, properties.cid.c_str());
        }
    }

    curl_easy_cleanup(curl);
    curl_global_cleanup();

    const Clock::time_point end = Clock::now();
    const long long total_ns = std::chrono::duration_cast<Nanoseconds>(end - start).count();

    std::printf("Stored candidates: %d\n", stored);
    std::printf("Total ns: %lld\n", total_ns);

    sqlite3_close(db);
    return 0;
}
