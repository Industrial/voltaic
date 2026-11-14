# voltaic 2 signature sidecar — generated, do not edit
render : usize -> &str -> ?2 -> String @ __vlt_a1 shared, __vlt_a2 shared, self shared
format : &Program -> &str -> String @ program shared, source shared
trimmed_tail : String -> String @ text owned
render : usize -> &str -> Item -> String @ indent owned, src owned, self owned
render : usize -> &str -> Stmt -> String @ indent owned, src owned, self owned
render : usize -> &str -> Expr -> String @ indent owned, src owned, self owned
render : usize -> &str -> Pattern -> String @ _indent shared, _src shared, self owned
Item.render method
Stmt.render method
Expr.render method
Pattern.render method
