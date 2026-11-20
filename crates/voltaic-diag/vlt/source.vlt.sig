# voltaic 2 signature sidecar — generated, do not edit
map_col : Origin -> usize -> usize @ self shared, gen_col shared
new : impl Into<String> -> Vec<Option<Origin>> -> SourceMap @ source_path owned, origins shared
len : SourceMap -> usize @ self shared
is_empty : SourceMap -> bool @ self shared
origin : SourceMap -> usize -> Option<&Origin> ! none @ self shared, gen_line shared
resolve : SourceMap -> usize -> usize -> Option<(usize, usize)> ! none + async @ self shared, gen_line shared, gen_col shared
synthetic_lines : SourceMap -> Vec<usize> @ self shared
render_map : &SourceMap -> String @ map shared
parse_map : &str -> Result<SourceMap, String> ! fail String @ text shared
sample_origin : Origin
sample_map : SourceMap
sample : SourceMap
VERSION : u32
Origin.span field
Origin.src_line field
Origin.src_col field
Origin.gen_col field
Origin.map_col method
SourceMap.source_path field
SourceMap.origins field
SourceMap.new method
SourceMap.len method
SourceMap.is_empty method
SourceMap.origin method
SourceMap.resolve method
SourceMap.synthetic_lines method
