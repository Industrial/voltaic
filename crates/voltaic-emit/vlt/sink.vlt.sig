# voltaic 2 signature sidecar — generated, do not edit
double_braces : &str -> String @ literal shared
format_template : &[StrInterpPart] -> usize -> (String, Vec<String>) @ parts shared, indent shared
holes : &[Expr] -> Vec<usize> @ args shared
emit_inline_comment : &mut Sink<'_> -> Option<&Trivia> -> () @ out shared, trivia shared
nest : &[(&str, &Expr)] -> &[String] -> &str -> usize -> bool -> (String, Vec<String>) @ draws shared, guards shared, item shared, base owned, nested shared
pad : &mut Sink<'_> -> usize -> () @ out shared, n shared
pad_to : &mut String -> usize -> () @ out shared, n shared
rust_escapes : &str -> String @ raw shared
rust_hatch_text : &str -> usize -> String @ code shared, indent shared
new : &str -> Sink @ source shared
mark : Span -> Sink -> () ! mut self @ span shared, self unique
unmark : Sink -> () ! mut self @ self unique
push : char -> Sink -> () @ ch shared, self shared
push_str : &str -> Sink -> () @ s shared, self shared
pop : Sink -> Option<char> ! none @ self shared
ends_with : &str -> Sink -> bool @ s owned, self shared
is_empty : Sink -> bool @ self shared
finish : &str -> Sink -> (String, SourceMap) @ source_path owned, self shared
close_line : Sink -> () @ self shared
origin_of : Span -> Sink -> Option<Origin> ! none @ span shared, self owned
gen_col_of_current_line : Sink -> usize @ self shared
write_text : &str -> Sink -> () @ s shared, self shared
write_str : &str -> Sink -> std::fmt::Result @ s shared, self shared
emit_trivia : &mut Sink<'_> -> &[Trivia] -> usize -> () @ out unique, trivia shared, indent shared
line : &mut Sink<'_> -> usize -> &str -> &str -> () @ out shared, indent shared, marker shared, text shared
Sink.out field
Sink.source field
Sink.current field
Sink.origins field
Sink.new method
Sink.mark method
Sink.unmark method
Sink.push method
Sink.push_str method
Sink.pop method
Sink.ends_with method
Sink.is_empty method
Sink.finish method
Sink.close_line method
Sink.origin_of method
Sink.gen_col_of_current_line method
Sink.write_text method
Sink.write_str method
