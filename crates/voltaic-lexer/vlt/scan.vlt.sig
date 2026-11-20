# voltaic 2 signature sidecar — generated, do not edit
is_ident_continue : char -> bool @ c shared
is_ident_start : char -> bool @ c shared
scan_byte : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
scan_comment : &str -> Option<(TriviaKind, usize)> ! none @ rest shared
scan_cstring : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
scan_escape : &mut Cursor -> Result<char, LexError> ! fail LexError @ cur owned
decode : &str -> Result<char, String> ! fail LexError + fail String @ after_backslash shared
scan_number : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError + mut is_float @ cur owned
scan_number_probe : &str -> (TokenKind, usize) @ src shared
scan_operator : &mut Cursor -> Option<TokenKind> ! none @ cur shared
scan_operator_probe : &str -> (Option<TokenKind>, usize) @ src shared
scan_quote : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
scan_raw_ident : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
scan_raw_string : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
scan_rust_block : &mut Cursor<'_> -> usize -> Option<String> ! none @ cur owned, base shared
scan_token_tree : &mut Cursor<'_> -> Result<(), LexError> ! fail LexError @ cur shared
skip_rust_string : &mut Cursor<'_> -> usize -> Result<(), LexError> ! fail LexError @ cur shared, start owned
skip_rust_quote : &mut Cursor<'_> -> () @ cur shared
skip_rust_block_comment : &mut Cursor<'_> -> usize -> Result<(), LexError> ! fail LexError @ cur shared, start owned
skip_rust_word : &mut Cursor<'_> -> usize -> Result<(), LexError> ! fail LexError @ cur owned, start owned
scan_string : &mut Cursor -> Result<TokenKind, LexError> ! fail LexError @ cur owned
split_parts : &str -> usize -> Result<Vec<StrPart>, LexError> ! fail LexError @ body shared, base shared
skip_interpolation : &mut Cursor -> usize -> Result<(usize, usize), LexError> ! fail LexError @ cur shared, base shared
split_format_spec : &str -> (&str, Option<&str>) @ body owned
trim_comment : &str -> String @ rest shared
verbatim_triple : &str -> Option<String> ! none @ raw shared
dedent_triple_body : &str -> String @ body shared
ASSIGNABLE_OPERATORS : &[&str]
OPERATOR_TABLE : &[(&str, TokenKind)]
