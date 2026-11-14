# voltaic 2 signature sidecar — generated, do not edit
sole_expression : Block -> Option<&Expr> ! none @ self shared
where_entry_name : Stmt -> Option<String> ! none @ self shared
plain : Expr -> Condition @ expr owned
sources : Condition -> Vec<&Expr> @ self shared
has_pattern : Condition -> bool @ self shared
Block.stmts field
Block.span field
Block.sole_expression method
Stmt.expr field
Stmt.span field
Stmt.leading field
Stmt.trailing field
Stmt.where_entry_name method
Binding.pattern field
Binding.ty field
Binding.value field
Binding.is_mut field
Binding.r#otherwise field
Binding.span field
Condition.operands field
Condition.plain method
Condition.sources method
Condition.has_pattern method
ConditionOperand.0 field
ConditionOperand.pattern field
ConditionOperand.source field
