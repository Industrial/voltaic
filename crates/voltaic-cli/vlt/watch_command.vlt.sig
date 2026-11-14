# voltaic 2 signature sidecar — generated, do not edit
watch_command : &[String] -> Result<(), String> ! fail String + io @ args shared
is_change : &EventKind -> bool @ kind shared
is_relevant : &Path -> bool @ path shared
report : &Target -> bool -> () @ target shared, plain shared
run_build : &str -> bool -> () @ input shared, plain shared
sources : &Path -> Vec<String> @ dir shared
resolve : &Path -> Result<Target, String> ! fail String @ arg owned
new : &str -> Dir ! panic @ tag shared
drop : Dir -> () @ self shared
Target.dir field
Target.file field
Target.resolve method
Dir.0 field
Dir.new method
Dir.drop method
