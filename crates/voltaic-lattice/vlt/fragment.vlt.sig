# voltaic 2 signature sidecar — generated, do not edit
read : &Expr -> Result<Predicate, Refusal> ! fail Refusal @ e owned
term_of : &Expr -> Result<Term, Refusal> ! fail Refusal @ e owned
path_of : &Expr -> Result<String, Refusal> ! fail Refusal @ e owned
comparison_of : BinaryOp -> Option<Comparison> ! none @ op shared
shown : &Expr -> String @ e shared
divisor_label : BinaryOp -> &str @ op shared
RULE_PRODUCT : &str
RULE_DIVISION : &str
RULE_CALL : &str
RULE_SHAPE : &str
Refusal.subterm field
Refusal.rule field
Refusal.span field
