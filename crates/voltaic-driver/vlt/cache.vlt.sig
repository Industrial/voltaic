# voltaic 2 signature sidecar — generated, do not edit
cache_key : &str -> &Fingerprint -> blake3::Hash @ source shared, fingerprint shared
fp : Fingerprint
current : Fingerprint
now : u128
r#of : &[u8] -> Fingerprint @ bytes owned
as_bytes : Fingerprint -> &[u8; 32] @ self shared
report : ?52 -> &str -> bool -> () @ self shared, __vlt_a1 shared, __vlt_a2 shared
report : Collect -> &str -> bool -> () @ self shared, label shared, hit shared
report : Silent -> &str -> bool -> () @ self shared, _label shared, _hit shared
new : &Path -> Store ! panic @ out owned
paths : &blake3::Hash -> Store -> (PathBuf, PathBuf) @ key shared, self shared
get : &blake3::Hash -> Store -> Option<Cached> ! none @ key shared, self shared
put : &blake3::Hash -> &Cached -> Store -> () @ key shared, cached shared, self shared
new : &str -> Dir ! panic @ tag shared
drop : Dir -> () @ self shared
entry : Cached
Fingerprint.0 field
Fingerprint.current method
Fingerprint.now method
Fingerprint.r#of method
Fingerprint.as_bytes method
Collect.0 field
Collect.report method
Silent.report method
Store.dir field
Store.new method
Store.paths method
Store.get method
Store.put method
Dir.0 field
Dir.new method
Dir.drop method
Cached.r#rust field
Cached.map field
