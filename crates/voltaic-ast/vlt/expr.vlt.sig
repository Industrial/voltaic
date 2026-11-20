# voltaic 2 signature sidecar — generated, do not edit
span : Expr -> Span @ self shared
is_statement_form : Expr -> bool @ self shared
is_control_flow : Expr -> bool @ self shared
statement_blocks : Expr -> Vec<&crate::Block> @ self shared
rust_text : BinaryOp -> Option<&str> ! none @ self shared
voltaic_text : BinaryOp -> &str @ self shared
power : BinaryOp -> u8 @ self shared
is_range : BinaryOp -> bool @ self shared
compare_method : BinaryOp -> Option<&str> ! none @ self shared
flipped : BinaryOp -> Option<BinaryOp> ! none @ self shared
rust_text : UnaryOp -> &str @ self shared
Expr.callee field
Expr.args field
Expr.is_dollar field
Expr.span field
Expr.recv field
Expr.name field
Expr.index field
Expr.op field
Expr.lhs field
Expr.rhs field
Expr.spelling field
Expr.fixity field
Expr.precedence field
Expr.target field
Expr.selector field
Expr.function field
Expr.place field
Expr.operand field
Expr.inner field
Expr.text field
Expr.params field
Expr.body field
Expr.is_move field
Expr.method field
Expr.ty field
Expr.value field
Expr.parts field
Expr.items field
Expr.count field
Expr.head field
Expr.fields field
Expr.base field
Expr.element field
Expr.qualifiers field
Expr.condition field
Expr.then_block field
Expr.r#otherwise field
Expr.label field
Expr.section field
Expr.scrutinee field
Expr.arms field
Expr.rows field
Expr.0 field
Expr.item field
Expr.pattern field
Expr.source field
Expr.binder field
Expr.code field
Expr.open field
Expr.indented field
Expr.body_span field
Expr.span method
Expr.is_statement_form method
Expr.is_control_flow method
Expr.statement_blocks method
BinaryOp.rust_text method
BinaryOp.voltaic_text method
BinaryOp.power method
BinaryOp.is_range method
BinaryOp.compare_method method
BinaryOp.flipped method
CaseArm.pattern field
CaseArm.ty field
CaseArm.r#guard field
CaseArm.body field
CaseArm.span field
CaseArm.leading field
CaseArm.trailing field
FieldInit.name field
FieldInit.value field
FieldInit.span field
Ident.text field
Ident.span field
StrInterpPart.0 field
StrInterpPart.expr field
StrInterpPart.spec field
TypeExpr.text field
TypeExpr.span field
UnaryOp.rust_text method
