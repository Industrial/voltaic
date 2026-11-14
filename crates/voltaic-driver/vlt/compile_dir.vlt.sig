# voltaic 2 signature sidecar — generated, do not edit
compile_dir : &Path -> &Path -> Result<Compiled, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ src owned, out owned
compile_file : &Path -> &Path -> &Store -> &Fingerprint -> Result<Outcome, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ source owned, out shared, store shared, fingerprint shared
write_outputs : &Path -> &Path -> &str -> &Cached -> Result<PathBuf, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ source shared, out shared, path shared, content shared
rerun_directives : Compiled -> Vec<String> @ self shared
is_empty : Compiled -> bool @ self shared
vlt_in : &Path -> Vec<PathBuf> @ dir shared
r#with : &str -> &[&str] -> Dir ! panic @ tag shared, names shared
drop : Dir -> () @ self shared
compile_dir_with : &Path -> &Path -> &dyn CacheReporter -> Result<Compiled, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ src shared, out shared, reporter shared
Outcome.written field
Outcome.was_cached field
Compiled.inputs field
Compiled.outputs field
Compiled.rerun_directives method
Compiled.is_empty method
Dir.0 field
Dir.r#with method
Dir.drop method
