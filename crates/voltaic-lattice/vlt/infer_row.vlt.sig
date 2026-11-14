# voltaic 2 signature sidecar — generated, do not edit
clause_row : &[voltaic_ast::Clause] -> Row @ clauses owned
clause_row_with : &[voltaic_ast::Clause] -> &[(String, Row)] -> Row @ clauses shared, known shared
block_row : &Block -> Row @ block owned
block_row_with : &Block -> &[(String, Row)] -> Row @ block shared, known shared
expr_row : &Expr -> Row @ e owned
expr_row_with : &Expr -> &[(String, Row)] -> Row @ e shared, known owned
called : &str -> &[(String, Row)] -> Row @ name shared, known shared
root_of : &Expr -> String @ target shared
fail_of : &Ty -> Option<Effect> ! none @ declared shared
