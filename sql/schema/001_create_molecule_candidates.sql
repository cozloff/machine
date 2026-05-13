CREATE TABLE IF NOT EXISTS molecule_candidates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    pubchem_cid TEXT,
    title TEXT,
    canonical_smiles TEXT,
    molecular_formula TEXT,
    molecular_weight TEXT,
    inchikey TEXT,
    mechanism TEXT,
    evidence_note TEXT,
    fetched_at TEXT NOT NULL
);
