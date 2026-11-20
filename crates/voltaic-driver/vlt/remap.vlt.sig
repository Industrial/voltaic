# voltaic 2 signature sidecar — generated, do not edit
remap : &RustcMessage -> &SourceMap -> &str -> &str -> &str -> &str -> Diagnostic @ message shared, map shared, vlt_path owned, vlt_source owned, vrs_path owned, vrs_source owned
code_of : &RustcMessage -> DiagnosticCode @ message shared
severity_of : &RustcMessage -> voltaic_diag::Severity @ message shared
remap_file : &RustcMessage -> &SourceMap -> &str -> &str -> Option<Diagnostic> ! none @ message owned, map shared, vlt_path owned, vlt_source owned
parsed_map : SourceMap ! panic
map_of : SourceMap
message_at : usize -> usize -> RustcMessage @ line shared, col shared
run : &RustcMessage -> Diagnostic @ m owned
char_offset : &str -> usize -> usize -> Option<usize> ! none @ source shared, line shared, col shared
generated_offset : &str -> usize -> usize -> usize ! none @ source owned, line owned, col owned
narrow : Span -> &str -> usize -> usize -> Span @ span owned, source shared, line shared, col shared
parse : &str -> Option<RustcMessage> ! none @ line shared
is_error : RustcMessage -> bool @ self shared
RustcMessage.path field
RustcMessage.line field
RustcMessage.col field
RustcMessage.level field
RustcMessage.code field
RustcMessage.message field
RustcMessage.parse method
RustcMessage.is_error method
