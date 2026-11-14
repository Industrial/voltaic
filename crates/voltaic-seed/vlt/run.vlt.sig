# voltaic 2 signature sidecar — generated, do not edit
run : i32 ! io
arguments : Option<(PathBuf, PathBuf)> ! none
report : &Compiled -> () @ compiled shared
tidy : &Path -> () @ dir shared
walk : &Path -> &Path -> Vec<(PathBuf, PathBuf)> @ src shared, out shared
