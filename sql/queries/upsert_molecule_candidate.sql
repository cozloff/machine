INSERT INTO molecule_candidates (
    name,
    pubchem_cid,
    title,
    canonical_smiles,
    molecular_formula,
    molecular_weight,
    inchikey,
    mechanism,
    evidence_note,
    fetched_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
ON CONFLICT(name) DO UPDATE SET
    pubchem_cid = excluded.pubchem_cid,
    title = excluded.title,
    canonical_smiles = excluded.canonical_smiles,
    molecular_formula = excluded.molecular_formula,
    molecular_weight = excluded.molecular_weight,
    inchikey = excluded.inchikey,
    mechanism = excluded.mechanism,
    evidence_note = excluded.evidence_note,
    fetched_at = CURRENT_TIMESTAMP;
