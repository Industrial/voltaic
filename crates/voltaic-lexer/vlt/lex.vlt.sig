# voltaic 2 signature sidecar — generated, do not edit
lex : &str -> Result<Vec<Token>, LexError> ! fail LexError + mut last + mut self @ src owned
offset_by : usize -> LexError -> LexError ! mut self @ base shared, self owned
new : &str -> Lexer @ src owned
run : Lexer -> Result<Vec<Token>, LexError> ! fail LexError + mut last + mut self @ self owned
step : Lexer -> Result<(), LexError> ! fail LexError + mut last + mut self @ self owned
scan_token : Lexer -> Result<TokenKind, LexError> ! fail LexError @ self shared
scan_hash : Lexer -> Result<bool, LexError> ! fail LexError + mut self @ self shared
scan_dashes : Lexer -> Result<bool, LexError> ! fail LexError + mut self @ self unique
track_delims : &TokenKind -> usize -> Lexer -> () @ kind shared, start owned, self unique
next_line_continues : Lexer -> bool @ self shared
end_of_line : Lexer -> () ! mut last + mut self @ self unique
start_of_line : Lexer -> Result<(), LexError> ! fail LexError @ self owned
measure_indent : Lexer -> Result<usize, LexError> ! fail LexError @ self shared
sync_indent : usize -> usize -> Lexer -> Result<(), LexError> ! fail LexError + mut self @ width unique, line_start shared, self unique
finish : Lexer -> Result<(), LexError> ! fail LexError + mut last + mut self @ self unique
push : TokenKind -> Span -> bool -> Lexer -> () ! mut self @ kind shared, span shared, ws_before shared, self unique
push_blank : usize -> usize -> Lexer -> () @ start owned, end owned, self shared
LexError.code field
LexError.message field
LexError.span field
LexError.help field
LexError.offset_by method
Lexer.cur field
Lexer.toks field
Lexer.indents field
Lexer.pending field
Lexer.open_delims field
Lexer.trailing field
Lexer.at_line_start field
Lexer.new method
Lexer.run method
Lexer.step method
Lexer.scan_token method
Lexer.scan_hash method
Lexer.scan_dashes method
Lexer.track_delims method
Lexer.next_line_continues method
Lexer.end_of_line method
Lexer.start_of_line method
Lexer.measure_indent method
Lexer.sync_indent method
Lexer.finish method
Lexer.push method
Lexer.push_blank method
