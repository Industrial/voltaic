# voltaic 2 signature sidecar — generated, do not edit
root : PathBuf
main : Result<(), String> ! fail String + io
sources : &Path -> Result<Vec<(String, String)>, String> ! fail String @ dir shared
machine : String
model_name : &str -> Option<String> ! none @ body shared
render : &[(String, usize, Duration)] -> usize -> Duration -> &str -> String @ rows shared, total_bytes shared, total_time shared, machine_text shared
