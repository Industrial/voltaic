# voltaic 2 signature sidecar — generated, do not edit
check : &Path -> &str -> Result<CheckOutcome, String> ! fail String @ path owned, source owned
new : &str -> Result<ScratchDir, String> ! fail String @ tag shared
path : ScratchDir -> &Path @ self shared
drop : ScratchDir -> () @ self shared
diagnose : &Path -> &str -> &ParseFailure -> Vec<Diagnostic> @ path shared, source owned, failure shared
io_diagnostic : &str -> String -> Diagnostic @ path owned, message shared
rustc_path : Option<PathBuf> ! none
is_real_rustc : &Path -> bool @ candidate shared
NEXT : std::sync::atomic::AtomicUsize
CheckOutcome.diagnostics field
CheckOutcome.failed field
ScratchDir.0 field
ScratchDir.new method
ScratchDir.path method
ScratchDir.drop method
