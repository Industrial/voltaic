# voltaic 2 signature sidecar — generated, do not edit
dedent_block : &str -> String @ body shared
lex_fragment : &str -> usize -> Result<Vec<Token>, LexError> ! fail LexError @ src shared, base shared
