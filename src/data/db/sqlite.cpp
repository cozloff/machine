#include <data/db/sqlite.h>

#include <cstdio>
#include <fstream>
#include <sstream>
#include <string>

namespace {

constexpr const char* kSchemaPath = "sql/schema/001_create_molecule_candidates.sql";
constexpr const char* kUpsertCandidatePath = "sql/queries/upsert_molecule_candidate.sql";

bool read_file(const char* path, std::string* contents) {
    std::ifstream file(path);
    if (!file) {
        std::fprintf(stderr, "Failed to open SQL file: %s\n", path);
        return false;
    }

    std::ostringstream buffer;
    buffer << file.rdbuf();
    *contents = buffer.str();
    return true;
}

bool exec_sql(sqlite3* db, const std::string& sql) {
    char* err_msg = nullptr;
    if (sqlite3_exec(db, sql.c_str(), nullptr, nullptr, &err_msg) != SQLITE_OK) {
        std::fprintf(stderr, "SQLite exec failed: %s\n", err_msg);
        sqlite3_free(err_msg);
        return false;
    }

    return true;
}

bool bind_text(sqlite3_stmt* stmt, int index, const std::string& value) {
    return sqlite3_bind_text(stmt, index, value.c_str(), -1, SQLITE_TRANSIENT) == SQLITE_OK;
}

bool bind_text(sqlite3_stmt* stmt, int index, const char* value) {
    return sqlite3_bind_text(stmt, index, value, -1, SQLITE_TRANSIENT) == SQLITE_OK;
}

}  // namespace

bool open_database(const char* path, sqlite3** db) {
    if (sqlite3_open(path, db) != SQLITE_OK) {
        std::fprintf(stderr, "SQLite open failed: %s\n", sqlite3_errmsg(*db));
        sqlite3_close(*db);
        return false;
    }

    return true;
}

bool initialize_schema(sqlite3* db) {
    std::string schema_sql;
    return read_file(kSchemaPath, &schema_sql) && exec_sql(db, schema_sql);
}

bool upsert_candidate(
    sqlite3* db,
    const Candidate& candidate,
    const CompoundProperties& properties
) {
    std::string sql;
    if (!read_file(kUpsertCandidatePath, &sql)) {
        return false;
    }

    sqlite3_stmt* stmt = nullptr;
    if (sqlite3_prepare_v2(db, sql.c_str(), -1, &stmt, nullptr) != SQLITE_OK) {
        std::fprintf(stderr, "SQLite prepare failed: %s\n", sqlite3_errmsg(db));
        return false;
    }

    const bool ok =
        bind_text(stmt, 1, candidate.name) &&
        bind_text(stmt, 2, properties.cid) &&
        bind_text(stmt, 3, properties.title) &&
        bind_text(stmt, 4, properties.canonical_smiles) &&
        bind_text(stmt, 5, properties.molecular_formula) &&
        bind_text(stmt, 6, properties.molecular_weight) &&
        bind_text(stmt, 7, properties.inchikey) &&
        bind_text(stmt, 8, candidate.mechanism) &&
        bind_text(stmt, 9, candidate.evidence_note);

    if (!ok || sqlite3_step(stmt) != SQLITE_DONE) {
        std::fprintf(stderr, "SQLite upsert failed: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return false;
    }

    sqlite3_finalize(stmt);
    return true;
}
