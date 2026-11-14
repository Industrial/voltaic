# voltaic 2 signature sidecar — generated, do not edit
crate_of : &str -> Option<String> ! none @ path shared
workspace_above : &Path -> Option<PathBuf> ! none @ from shared
sidecars_of : &Path -> &str -> Vec<PathBuf> @ root shared, named shared
gather : &Path -> &mut Vec<PathBuf> -> () @ dir shared, into shared
upstream : &Program -> &Path -> Declarations @ program shared, from shared
