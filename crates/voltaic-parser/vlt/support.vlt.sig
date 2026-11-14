# voltaic 2 signature sidecar — generated, do not edit
expected : impl Into<String> -> Span -> ParseError @ message owned, span shared
r#with : Code -> impl Into<String> -> Span -> ParseError @ code shared, message owned, span shared
with_help : Code -> impl Into<String> -> impl Into<String> -> Span -> ParseError @ code shared, message owned, help owned, span shared
nesting_depth : &[Token] -> usize @ toks shared
above_bitor : u8
binary_power : &TokenKind -> Option<(BinaryOp, u8, u8)> ! none @ kind shared
power : TokenKind -> u8 @ kind shared
parse_source : &str -> Result<Program, ParseFailure> ! fail ParseFailure @ src owned
ParseError.code field
ParseError.message field
ParseError.span field
ParseError.help field
ParseError.expected method
ParseError.r#with method
ParseError.with_help method
ParseFailure.0 field
