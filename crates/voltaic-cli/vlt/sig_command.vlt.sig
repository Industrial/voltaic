# voltaic 2 signature sidecar — generated, do not edit
new : Agreement
absorb : &Agreement -> Agreement -> () ! mut self @ other shared, self unique
parse_sig_args : &[String] -> Result<SigArgs, String> ! fail String @ args shared
sig_command : &[String] -> Result<(), String> ! fail String + io @ args shared
sidecar_path : &Path -> PathBuf @ input shared
built : &PathBuf -> Result<voltaic_lattice::Sidecar, String> ! fail String @ input shared
write_sidecar : &PathBuf -> Result<(), String> ! fail String @ input shared
emit_one : &PathBuf -> Result<(), String> ! fail String + io @ input shared
require_one : &PathBuf -> Result<usize, String> ! fail String @ input shared
semver : &PathBuf -> &PathBuf -> Result<(), String> ! fail String + io @ input shared, older shared
self_free : &voltaic_lattice::Ty -> &voltaic_lattice::Inferred -> voltaic_lattice::Ty @ written owned, definition shared
verify_one : &PathBuf -> Result<Agreement, String> ! fail String @ input shared
report : &PathBuf -> Option<&str> -> Result<(), String> ! fail String + io @ input shared, why shared
touching : &voltaic_lattice::Inferred -> &voltaic_lattice::Store -> Vec<usize> @ definition shared, store shared
SigArgs.inputs field
SigArgs.why field
SigArgs.verify field
SigArgs.emit field
SigArgs.require field
SigArgs.against field
Agreement.checked field
Agreement.agreed field
Agreement.differed field
Agreement.new method
Agreement.absorb method
