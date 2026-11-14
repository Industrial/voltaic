# voltaic 2 signature sidecar — generated, do not edit
transpile : &Path -> &str -> Result<String, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ path owned, text owned
gate : &Path -> &str -> &str -> Gate @ path shared, original shared, migrated shared
first_difference : &str -> &str -> Option<(usize, String, String)> ! none @ before shared, after shared
passed : Gate -> bool @ self shared
explain : Gate -> String @ self shared
apply : &Path -> &str -> Result<Gate, String> ! fail String @ path shared, migrated shared
Gate.line field
Gate.was field
Gate.now field
Gate.side field
Gate.problems field
Gate.passed method
Gate.explain method
