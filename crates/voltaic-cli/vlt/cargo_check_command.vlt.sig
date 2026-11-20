# voltaic 2 signature sidecar — generated, do not edit
parse_cargo_check_args : &[String] -> CargoCheckArgs @ args shared
cargo_check_command : &[String] -> Result<(), String> ! fail String + io @ args shared
compiler_message : &str -> &Path -> Option<Reported> ! none + mut located @ line shared, here owned
text_at : &Value -> &str -> Option<String> ! none @ value owned, key owned
number_at : &Value -> &str -> usize @ value owned, key owned
is_primary : &Value -> bool @ span shared
help_of : &Value -> Option<String> ! none @ message shared
resolve_rust : &str -> Option<&Path> -> &Path -> PathBuf @ file shared, manifest shared, here owned
mapped : &RustcMessage -> &Path -> &Path -> Option<Diagnostic> ! none @ message owned, generated shared, here shared
source_of : &str -> &Path -> &Path -> Option<PathBuf> ! none @ recorded shared, generated shared, here shared
shown : &Path -> &Path -> String @ path shared, here shared
cargo_path : Option<PathBuf> ! none
is_real_cargo : &Path -> bool @ candidate shared
CargoCheckArgs.forwarded field
CargoCheckArgs.plain field
Reported.0 field
