# voltaic 2 signature sidecar — generated, do not edit
stmt_text : &Stmt -> usize -> &str -> String @ stmt shared, indent owned, source owned
binding_text : &Binding -> usize -> &str -> String @ binding shared, indent shared, source shared
block_text : &Block -> usize -> &str -> String @ block shared, indent shared, source shared
case_arm_text : &CaseArm -> usize -> &str -> String @ arm shared, indent shared, source shared
code_text : &Expr -> usize -> &str -> String @ expr shared, indent owned, source owned
condition_text : &Condition -> usize -> &str -> String @ condition shared, indent shared, source shared
else_text : &Option<Block> -> usize -> &str -> String @ r#otherwise shared, indent shared, source shared
indent_text : usize -> String @ indent owned
label_text : &Option<String> -> String @ label shared
trivia_text : &Trivia -> usize -> String @ trivium shared, indent shared
with_marker : &str -> &str -> &str -> String @ pad shared, marker shared, text shared
