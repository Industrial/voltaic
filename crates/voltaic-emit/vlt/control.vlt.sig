# voltaic 2 signature sidecar — generated, do not edit
emit_arm : &mut Sink<'_> -> &Clause -> &str -> usize -> bool -> () @ out shared, clause shared, arm shared, indent shared, tail shared
ascribed_arm : &Pattern -> &str -> Option<String> ! none @ pattern shared, ty shared
sole_binder : &Pattern -> Option<String> ! none @ pattern shared
ascribed_list : &Expr -> &TypeExpr -> usize -> Option<String> ! none @ inner shared, ty shared, indent shared
assert_call : &Expr -> usize -> String @ e shared, indent shared
emit_ladder : &[Guard] -> usize -> String @ rows shared, indent shared
emit_case : &Expr -> &[CaseArm] -> usize -> String @ scrutinee shared, arms shared, indent shared
catch_fallback : &Expr -> &Expr -> usize -> String @ subject shared, fallback shared, indent shared
emit_comprehension : &Expr -> &[Qualifier] -> usize -> String @ element shared, qualifiers shared, indent owned
condition_head : &str -> &Condition -> usize -> String @ keyword shared, condition shared, indent shared
breaks : &Condition -> bool @ condition shared
control_text : &Expr -> usize -> String @ expr shared, indent shared
emit_block : &Option<String> -> &Block -> usize -> String @ label shared, body shared, indent shared
emit_condition : &Condition -> usize -> String @ condition shared, indent shared
emit_control : &mut Sink<'_> -> &Expr -> usize -> bool -> () @ out shared, expr shared, indent shared, tail shared
emit_control_inline : &mut Sink<'_> -> &Expr -> usize -> bool -> () @ out shared, expr shared, indent shared, tail shared
emit_stmts : &mut Sink<'_> -> &[Stmt] -> usize -> bool -> () @ out shared, stmts shared, indent shared, tail shared
emit_guards : &mut Sink<'_> -> &[Guard] -> usize -> bool -> bool -> () @ out shared, guards shared, indent shared, tail shared, joinable shared
label_prefix : &Option<String> -> String @ label shared
one_line_if : &Condition -> &Block -> Option<&Block> -> usize -> Option<String> ! none @ condition shared, then_block shared, else_block shared, indent shared
emit_stmt : &mut Sink<'_> -> &Stmt -> usize -> bool -> () @ out shared, stmt shared, indent shared, tail shared
try_block : &Block -> usize -> String @ body shared, indent shared
two_armed : &[Guard] -> usize -> Option<String> ! none @ guards shared, indent shared
unlowerable : &str -> String @ what shared
emit_unsafe_block : &Block -> usize -> String @ body shared, indent shared
