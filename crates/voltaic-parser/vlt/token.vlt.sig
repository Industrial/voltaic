# voltaic 2 signature sidecar — generated, do not edit
expr_from_tokens : &[Token] -> Span -> Result<Expr, ParseError> ! fail ParseError @ tokens shared, span shared
flush_word : &mut String -> &mut String -> &str -> &str -> () @ out shared, word shared, var shared, subject shared
inline_rust : &mut Parser -> Vec<Trivia> -> usize -> Result<InlineRust, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
is_word : char -> bool @ ch shared
reserved_word : &mut Parser -> ParseError @ p shared
rust_hatch : &mut Parser -> Result<(String, Span), ParseError> ! fail ParseError @ p shared
rust_string_body : &str -> String @ text shared
scan_run : &[Stmt] -> bool -> &mut Vec<ParseError> -> () @ stmts shared, in_tail shared, problems shared
substitute_var : &mut Signature -> &str -> &str -> () ! mut ret @ sig shared, var shared, subject shared
token_text : &TokenKind -> String @ kind shared
