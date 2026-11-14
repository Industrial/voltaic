# voltaic 2 signature sidecar — generated, do not edit
judge : &Path -> &str -> &Program -> Vec<Diagnostic> @ path shared, source owned, program shared
enclosing : &[Inferred] -> Span -> Option<&Inferred> ! none @ definitions shared, at shared
boundary : Option<&Inferred> -> Span -> Span @ owner shared, at owned
stated : Option<&Inferred> -> &Obligation -> &voltaic_lattice::Outcome -> String @ owner shared, obligation shared, outcome shared
sides : &voltaic_lattice::Store -> usize -> &Obligation -> &voltaic_lattice::Outcome -> Span -> Vec<(Span, String)> @ store shared, at_index shared, obligation shared, outcome shared, at owned
opposing : &voltaic_lattice::Store -> usize -> Option<(Span, String)> ! none @ store shared, at_index shared
shares : &[voltaic_lattice::Variable] -> &voltaic_lattice::Constraint -> bool @ wanted shared, candidate shared
advice : Option<&Inferred> -> &voltaic_lattice::Provenance -> String @ owner shared, provenance shared
judgements : &Program -> Vec<(String, String)> @ program shared
