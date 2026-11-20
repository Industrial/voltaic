# voltaic 2 signature sidecar — generated, do not edit
parse_gen_args : &[String] -> Result<GenArgs, String> ! fail String @ args shared
gen_command : &[String] -> Result<(), String> ! fail String + io @ args shared
summary : Tally -> bool -> Result<(), String> ! fail String + io @ tally shared, check shared
jobs : &GenArgs -> Result<Vec<Job>, String> ! fail String @ parsed shared
workspace_jobs : &Path -> Result<Vec<Job>, String> ! fail String @ root owned
shown_under : &Path -> Option<&Path> -> String @ source shared, base shared
write_pair : &Path -> &Emitted -> bool -> bool -> &mut Tally -> Result<(), String> ! fail String + mut tally @ target shared, emitted shared, rustfmt shared, check owned, tally owned
settle : &Path -> &str -> bool -> &mut Tally -> Result<(), String> ! fail String + mut tally @ target shared, content shared, check shared, tally unique
sweep : &Path -> &Path -> bool -> &mut Tally -> Result<(), String> ! fail String + mut tally @ root shared, stale shared, check shared, tally unique
formatted : &Path -> &str -> Result<String, String> ! fail String @ target shared, text shared
GenArgs.dirs field
GenArgs.workspace field
GenArgs.check field
GenArgs.rustfmt field
GenArgs.plain field
Job.src field
Job.out field
Job.base field
Tally.written field
Tally.removed field
Tally.unchanged field
Tally.drift field
