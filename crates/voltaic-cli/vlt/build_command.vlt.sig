# voltaic 2 signature sidecar — generated, do not edit
build_command : &[String] -> Result<(), String> ! fail String + io @ args shared
default_output : &Path -> PathBuf @ input owned
map_path : &Path -> PathBuf @ output owned
parse_build_args : &[String] -> Result<BuildArgs, String> ! fail String @ args shared
BuildArgs.input field
BuildArgs.output field
BuildArgs.plain field
