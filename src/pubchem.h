#ifndef MACHINE_PUBCHEM_H_
#define MACHINE_PUBCHEM_H_

#include <curl/curl.h>

#include <string>

struct CompoundProperties {
    std::string cid;
    std::string title;
    std::string canonical_smiles;
    std::string molecular_formula;
    std::string molecular_weight;
    std::string inchikey;
};

bool fetch_pubchem_properties(
    CURL* curl,
    const std::string& name,
    CompoundProperties* properties
);

#endif
