# voltaic 2 signature sidecar — generated, do not edit
mode_of : &str -> &[voltaic_ast::Clause] -> Mode @ name shared, clauses shared
binder_of : &str -> &str @ name shared
calls_on : &[voltaic_ast::Clause] -> Vec<(String, String)> @ clauses shared
collect_calls_block : &Block -> &mut Vec<(String, String)> -> () @ block shared, found shared
collect_calls : &Expr -> &mut Vec<(String, String)> -> () @ e shared, found shared
demands_of : &[voltaic_ast::Clause] -> Vec<Demand> @ clauses shared
walk_block : &Block -> &mut Vec<Demand> -> bool -> () @ block shared, found shared, escaping shared
walk_expr : &Expr -> &mut Vec<Demand> -> bool -> () @ e shared, found shared, escaping shared
root_name : &Expr -> Option<String> ! none @ target shared
Demand.name field
Demand.mode field
Demand.span field
