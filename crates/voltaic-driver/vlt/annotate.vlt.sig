# voltaic 2 signature sidecar — generated, do not edit
annotate : &mut Program -> Vec<Unresolved> @ program shared
analyse : &mut Program -> Analysis @ program owned
analyse_at : &mut Program -> &Path -> Analysis @ program shared, from shared
method_spans : &Store -> &Outcome -> &Declarations -> Vec<Span> @ store shared, outcome shared, known shared
methods_of : &mut Item -> Vec<&mut FnDecl> @ item shared
written_for : &str -> voltaic_ast::Span -> &[(String, Ty, voltaic_ast::Span)] -> Option<Ty> ! none @ name shared, span shared, filled shared
fill : &mut FnDecl -> &Ty -> bool ! mut f @ f unique, whole shared
spine_of : &Ty -> usize -> Option<(Vec<String>, String)> ! none @ whole shared, arity shared
arrow_parts : &Ty -> Option<(Ty, Ty)> ! none @ held shared
refusals : &Path -> &str -> &[Unresolved] -> Vec<Diagnostic> @ path shared, source owned, open shared
Unresolved.name field
Unresolved.shown field
Unresolved.span field
Analysis.open field
Analysis.methods field
