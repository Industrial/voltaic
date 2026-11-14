# voltaic 2 signature sidecar — generated, do not edit
fill_holes : &[Expr] -> &[String] -> Vec<Expr> @ args shared, params shared
emit_dot : &Expr -> &Ident -> &[Expr] -> usize -> String @ recv shared, name shared, args shared, indent shared
emit_application : &Expr -> &[Expr] -> usize -> String @ callee shared, args owned, indent owned
enter : ArityTable -> ArityScope @ table shared
arity_of : &str -> Option<usize> ! none @ name shared
drop : ArityScope -> () @ self shared
from_items : &[Item] -> ArityTable @ items shared
gather : &[Item] -> &mut Vec<String> -> ArityTable -> () @ items shared, ambiguous shared, self shared
arity_of : &str -> ArityTable -> Option<usize> ! none @ name shared, self shared
chain : &str -> &[String] -> usize -> String @ receiver shared, segments shared, base shared
closure : &[String] -> &str -> String @ params shared, body shared
closure_param : usize -> String @ index shared
coalesce : &Expr -> &Expr -> usize -> String @ lhs shared, rhs shared, indent shared
compose : BinaryOp -> &Expr -> &Expr -> usize -> usize -> String @ op shared, lhs shared, rhs shared, param shared, indent shared
compose_chain : BinaryOp -> &Expr -> &Expr -> &mut Vec<&Expr> -> () @ op shared, lhs shared, rhs shared, out shared
emit_expr : &Expr -> usize -> String @ e owned, indent owned
expr_list : &[Expr] -> usize -> String @ args shared, indent shared
field_value : &FieldInit -> usize -> String @ f shared, indent owned
macro_call : &str -> &[Expr] -> usize -> String @ name shared, args shared, indent shared
macro_argument : &Expr -> usize -> Vec<String> @ arg shared, indent shared
method_ref : &Ident -> &[Expr] -> usize -> String @ name shared, args shared, indent shared
missing_arity : &Expr -> &[Expr] -> Option<usize> ! none @ callee shared, args shared
emit_over : &[Ident] -> &Expr -> &Expr -> usize -> String @ selector shared, function shared, place owned, indent shared
pipeline : &Expr -> &Expr -> usize -> String @ lhs shared, rhs shared, indent shared
callable : &Expr -> usize -> String @ callee owned, indent owned
receiver : &Expr -> usize -> String @ e shared, indent shared
emit_section : &Section -> usize -> String @ section shared, indent shared
sole_expression : &Guard -> usize -> Option<String> ! none @ row shared, indent shared
unparen : &Expr -> &Expr @ expr shared
ident : &str -> Expr @ text owned
paren : Expr -> Expr @ inner owned
emit_with_map : &Program -> &str -> &str -> (String, SourceMap) @ program shared, source shared, source_path owned
wraps_as_receiver : &Expr -> bool ! none @ e shared
ArityScope.previous field
ArityScope.enter method
ArityScope.arity_of method
ArityScope.drop method
ArityTable.arities field
ArityTable.from_items method
ArityTable.gather method
ArityTable.arity_of method
