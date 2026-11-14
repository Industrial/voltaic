# voltaic 2 signature sidecar — generated, do not edit
read : &Expr -> Result<Predicate, Refusal> ! fail Refusal @ e owned
term_of : &Expr -> Result<Term, Refusal> ! fail Refusal @ e owned
path_of : &Expr -> Result<String, Refusal> ! fail Refusal @ e owned
comparison_of : BinaryOp -> Option<Comparison> ! none @ op shared
shown : &Expr -> String @ e shared
divisor_label : BinaryOp -> &str @ op shared
Refusal.subterm field
Refusal.rule field
Refusal.span field
