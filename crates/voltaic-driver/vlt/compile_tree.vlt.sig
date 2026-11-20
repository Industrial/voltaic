# voltaic 2 signature sidecar — generated, do not edit
emit_file : &Path -> &str -> Result<Emitted, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ source owned, label shared
compile_tree : &Path -> &Path -> Result<Compiled, Vec<Diagnostic>> ! fail Vec<Diagnostic> @ src shared, out shared
settle : &Path -> &Emitted -> Result<(), Vec<Diagnostic>> ! fail Vec<Diagnostic> @ target shared, emitted shared
map_beside : &Path -> PathBuf @ target shared
differs : &Path -> &str -> bool @ target shared, content shared
write_changed : &Path -> &str -> std::io::Result<bool> @ target shared, content shared
vlt_tree : &Path -> Vec<PathBuf> @ root shared
orphans : &Path -> &Path -> Vec<PathBuf> @ src shared, out shared
orphan_sidecars : &Path -> Vec<PathBuf> @ src shared
prune : &Path -> &Path -> () @ root shared, removed shared
gather : &Path -> &str -> &mut Vec<PathBuf> -> () @ dir shared, suffix shared, into shared
Emitted.r#rust field
Emitted.map field
Emitted.warnings field
