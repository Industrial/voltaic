# voltaic 2 signature sidecar — generated, do not edit
pattern : &mut Parser -> Result<Pattern, ParseError> ! fail ParseError @ p owned
pattern_alt : &mut Parser -> Result<Pattern, ParseError> ! fail ParseError @ p shared
pattern_app : &mut Parser -> Result<Pattern, ParseError> ! fail ParseError + panic @ p owned
pattern_series : &mut Parser -> &TokenKind -> Result<Vec<Pattern>, ParseError> ! fail ParseError @ p shared, close shared
patterns : &mut Parser -> Result<Vec<Pattern>, ParseError> ! fail ParseError @ p shared
record_pattern : &mut Parser -> String -> usize -> Result<Pattern, ParseError> ! fail ParseError + panic @ p shared, head shared, start owned
starts_a_pattern_argument : &Parser -> bool @ p shared
