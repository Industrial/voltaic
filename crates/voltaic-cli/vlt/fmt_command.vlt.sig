# voltaic 2 signature sidecar — generated, do not edit
check_command : &[String] -> Result<(), String> ! fail String + io @ args shared
report : &CheckOutcome -> bool -> Result<(), String> ! fail String + io @ outcome shared, plain shared
fmt_command : &[String] -> Result<(), String> ! fail String @ args shared
parse_fmt_args : &[String] -> Result<FmtArgs, String> ! fail String @ args shared
FmtArgs.inputs field
FmtArgs.check field
FmtArgs.plain field
