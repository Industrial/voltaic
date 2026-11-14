# voltaic 2 signature sidecar — generated, do not edit
rewrite_program : &mut Program -> Pass -> () @ program shared, step shared
rewrite_item : &mut Item -> Pass -> () @ item shared, step owned
rewrite_block : &mut Block -> Pass -> () @ block shared, step shared
rewrite_stmt : &mut Stmt -> Pass -> () @ stmt shared, step shared
rewrite_guard : &mut Guard -> Pass -> () @ row shared, step shared
rewrite_condition : &mut Condition -> Pass -> () @ cond shared, step shared
rewrite_arm : &mut CaseArm -> Pass -> () @ arm shared, step shared
rewrite_field : &mut FieldInit -> Pass -> () @ field shared, step shared
rewrite_part : &mut StrInterpPart -> Pass -> () @ part shared, step owned
rewrite_qualifier : &mut Qualifier -> Pass -> () @ q shared, step owned
rewrite_section : &mut Section -> Pass -> () @ s shared, step owned
rewrite_expr : &mut Expr -> Pass -> () @ e shared, step shared
descend : &mut Expr -> Pass -> () @ e shared, step owned
apply_pass : &mut Expr -> Pass -> () @ e owned, pass shared
blank_number : &mut Expr -> () ! mut ? @ e shared
collapse_pipeline : &mut Expr -> () ! mut ? + mut found + mut named @ e shared
