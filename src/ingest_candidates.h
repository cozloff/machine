#ifndef INGEST_CANDIDATES_H
#define INGEST_CANDIDATES_H

#include <vector>
#include <sqlite3.h>
#include <curl/curl.h>

#include <data/db/sqlite.h>

std::vector<Candidate> getCandidates();
bool ingestCandidates(sqlite3* db, CURL* curl);

#endif
