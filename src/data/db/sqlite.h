#ifndef MACHINE_DB_DATABASE_H_
#define MACHINE_DB_DATABASE_H_

#include <ingest/pubchem.h>

#include <sqlite3.h>

struct Candidate {
    const char* name;
    const char* pubchem_query;
    const char* mechanism;
    const char* evidence_note;
};

bool open_database(const char* path, sqlite3** db);
bool initialize_schema(sqlite3* db);
bool upsert_candidate(
    sqlite3* db,
    const Candidate& candidate,
    const CompoundProperties& properties
);

#endif
