# voltaic 2 signature sidecar — generated, do not edit
marks : &str -> Vec<Mark> @ text shared
agree : &[Mark] -> usize -> &[Mark] -> usize -> bool @ old shared, i shared, new shared, j shared
resync : &[Mark] -> usize -> &[Mark] -> usize -> Option<(usize, usize)> ! none @ old shared, i shared, new shared, j shared
shifted : &SourceMap -> Option<(usize, usize, usize)> -> Option<Origin> ! none @ map shared, entry shared
realign : &str -> &str -> &str -> Option<String> ! none @ before shared, after shared, map shared
sample_before : &str
sample_after : &str
sample_map : &str
Mark.ch field
Mark.line field
Mark.col field
