# voltaic 2 signature sidecar — generated, do not edit
emit_program : &[u8] -> () @ bytes shared
lex_source : &[u8] -> () @ bytes shared
parse_program : &[u8] -> () @ bytes shared
roundtrip : &[u8] -> Option<String> ! none @ bytes shared
solve_program : &[u8] -> Option<u32> ! none @ bytes shared
