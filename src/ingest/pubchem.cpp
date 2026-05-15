#include <data/db/sqlite.h>
#include <ingest/ingest_candidates.h>
#include <ingest/pubchem.h>

#include <chrono>
#include <cstdio>
#include <string>
#include <vector>

namespace {

size_t write_callback(char* ptr, size_t size, size_t nmemb, void* userdata) {
    auto* response = static_cast<std::string*>(userdata);
    response->append(ptr, size * nmemb);
    return size * nmemb;
}

std::string url_escape(CURL* curl, const std::string& value) {
    char* escaped = curl_easy_escape(curl, value.c_str(), static_cast<int>(value.size()));
    if (escaped == nullptr) {
        return "";
    }

    std::string result(escaped);
    curl_free(escaped);
    return result;
}

bool http_get(CURL* curl, const std::string& url, std::string* response) {
    response->clear();
    curl_easy_reset(curl);
    curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, response);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_USERAGENT, "machine/0.1");
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 20L);

    const CURLcode result = curl_easy_perform(curl);
    if (result != CURLE_OK) {
        std::fprintf(stderr, "HTTP failed: %s\n", curl_easy_strerror(result));
        return false;
    }

    long status = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &status);
    if (status < 200 || status >= 300) {
        std::fprintf(stderr, "HTTP status %ld for %s\n", status, url.c_str());
        return false;
    }

    return true;
}

std::vector<std::string> parse_csv_line(const std::string& line) {
    std::vector<std::string> fields;
    std::string field;
    bool in_quotes = false;

    for (std::size_t i = 0; i < line.size(); ++i) {
        const char ch = line[i];
        if (ch == '"') {
            if (in_quotes && i + 1 < line.size() && line[i + 1] == '"') {
                field.push_back('"');
                ++i;
            } else {
                in_quotes = !in_quotes;
            }
        } else if (ch == ',' && !in_quotes) {
            fields.push_back(field);
            field.clear();
        } else if (ch != '\r') {
            field.push_back(ch);
        }
    }

    fields.push_back(field);
    return fields;
}

bool parse_pubchem_csv(const std::string& csv, CompoundProperties* properties) {
    const std::size_t first_newline = csv.find('\n');
    if (first_newline == std::string::npos) {
        return false;
    }

    const std::size_t second_newline = csv.find('\n', first_newline + 1);
    const std::string row = csv.substr(
        first_newline + 1,
        second_newline == std::string::npos
            ? std::string::npos
            : second_newline - first_newline - 1
    );

    const std::vector<std::string> fields = parse_csv_line(row);
    if (fields.size() < 6) {
        return false;
    }

    properties->cid = fields[0];
    properties->title = fields[1];
    properties->canonical_smiles = fields[2];
    properties->molecular_formula = fields[3];
    properties->molecular_weight = fields[4];
    properties->inchikey = fields[5];
    return true;
}

}

bool ingestPubChem() {
    using Clock = std::chrono::steady_clock;
    using Nanoseconds = std::chrono::nanoseconds;

    const Clock::time_point start = Clock::now();

    sqlite3* db = nullptr;
    if (!open_database("data/machine.db", &db)) {
        return false;
    }

    if (!initialize_schema(db)) {
        sqlite3_close(db);
        return false;
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

bool fetch_pubchem_properties(
    CURL* curl,
    const std::string& name,
    CompoundProperties* properties
) {
    const std::string escaped_name = url_escape(curl, name);
    const std::string url =
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/" +
        escaped_name +
        "/property/Title,CanonicalSMILES,MolecularFormula,MolecularWeight,InChIKey/CSV";

    std::string response;
    return http_get(curl, url, &response) && parse_pubchem_csv(response, properties);
}
