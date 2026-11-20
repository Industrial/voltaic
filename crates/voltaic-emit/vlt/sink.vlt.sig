# voltaic 2 signature sidecar — generated, do not edit
double_braces : &str -> String @ literal shared
format_template : &[StrInterpPart] -> usize -> (String, Vec<String>) @ parts shared, indent shared
inlinable : &str -> bool @ text shared
holes : &[Expr] -> Vec<usize> @ args shared
emit_inline_comment : &mut Sink<'_> -> Option<&Trivia> -> () @ out unique, trivia shared
nest : &[(&str, &Expr)] -> &[String] -> &str -> usize -> bool -> (String, Vec<String>) @ draws shared, guards shared, item shared, base owned, nested shared
pad : &mut Sink<'_> -> usize -> () @ out unique, n shared
pad_to : &mut String -> usize -> () @ out unique, n shared
rust_escapes : &str -> String @ raw shared
rust_hatch_text : &str -> usize -> String @ code shared, indent shared
macro_verbatim : &str -> char -> &str -> bool -> usize -> usize -> String @ name shared, open shared, body shared, indented shared, at shared, indent shared
line_origin : usize -> usize -> String @ at shared, breaks shared
resolve_origin : &str -> &str -> Option<usize> ! none @ source shared, marker shared
new : &str -> Sink @ source shared
nested : &str -> Sink @ source shared
mark : Span -> Sink -> () ! mut self @ span shared, self unique
unmark : Sink -> () ! mut self @ self unique
push : char -> Sink -> () @ ch shared, self unique
push_str : &str -> Sink -> () @ s shared, self unique
pop : Sink -> Option<char> ! none @ self shared
ends_with : &str -> Sink -> bool @ s owned, self shared
is_empty : Sink -> bool @ self shared
finish : &str -> Sink -> (String, SourceMap) @ source_path owned, self unique
close_line : Sink -> () @ self unique
origin_of : Span -> Sink -> Option<Origin> ! none @ span shared, self owned
gen_col_of_current_line : Sink -> usize @ self shared
take_origin : &str -> Sink -> () @ part shared, self unique
write_text : &str -> Sink -> () @ s shared, self unique
write_str : &str -> Sink -> std::fmt::Result @ s shared, self unique
emit_trivia : &mut Sink<'_> -> &[Trivia] -> usize -> () @ out unique, trivia shared, indent shared
line : &mut Sink<'_> -> usize -> &str -> &str -> () @ out unique, indent shared, marker shared, text shared
Sink.out field
Sink.source field
Sink.current field
Sink.origins field
Sink.nested field
Sink.pinned field
Sink.new method
Sink.nested method
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
Sink.take_origin method
Sink.write_text method
Sink.write_str method
