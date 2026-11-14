# voltaic 2 signature sidecar — generated, do not edit
application : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p shared
atom : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p shared
binary_op : &mut Parser -> Option<BinaryOp> ! none @ p shared
digit_precedence : &str -> Option<u8> ! none @ text shared
operator_chains : &Parser -> &Ident -> bool @ p shared, spelling shared
parse_expr : &mut Parser -> u8 -> Result<Expr, ParseError> ! fail ParseError @ p owned, min shared
parse_spine : &mut Parser -> Result<Vec<TypeExpr>, ParseError> ! fail ParseError @ p shared
postfix : &mut Parser -> Expr -> Result<Result<Expr, Expr>, ParseError> ! fail ParseError + panic @ p owned, lhs owned
prefix : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p owned
starts_argument : &TokenKind -> bool @ kind shared
unary_op : &mut Parser -> Option<UnaryOp> ! none @ p shared
user_rung : &Parser -> Option<(Ident, Fixity, u8, u8, String, u8)> ! none @ p shared
