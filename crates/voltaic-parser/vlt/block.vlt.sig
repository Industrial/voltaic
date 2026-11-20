# voltaic 2 signature sidecar — generated, do not edit
names_the_retired_until : &Parser -> bool @ p shared
arm_body : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p owned
bind_block : &mut Parser -> usize -> Result<Expr, ParseError> ! fail ParseError @ p owned, start owned
block_expr : &mut Parser -> Option<String> -> usize -> Result<Expr, ParseError> ! fail ParseError @ p shared, label shared, start owned
case_arm : &mut Parser -> Result<CaseArm, ParseError> ! fail ParseError + panic @ p shared
case_of : &mut Parser -> Result<Expr, ParseError> ! fail ParseError + panic @ p shared
check_callback_tails : &Program -> Vec<ParseError> @ program shared
condition : &mut Parser -> Result<Condition, ParseError> ! fail ParseError @ p owned
ladder_condition : &mut Parser -> Result<Condition, ParseError> ! fail ParseError @ p owned
condition_operand : &mut Parser -> Result<ConditionOperand, ParseError> ! fail ParseError + panic @ p owned
control_flow : &mut Parser -> Result<Option<Expr>, ParseError> ! fail ParseError + panic @ p owned
holds_a_pattern_test : &Parser -> bool -> &TokenKind -> bool @ p shared, one_operand shared, marker shared
modified_block : &mut Parser -> Keyword -> usize -> Result<Expr, ParseError> ! fail ParseError @ p owned, kw shared, start shared
qualifiers : &mut Parser -> Result<Vec<Qualifier>, ParseError> ! fail ParseError @ p shared
reaches_a_draw_arrow : &Parser -> bool @ p shared
reaches_a_fat_arrow : &Parser -> Option<Span> ! none @ p shared
return_in_tail : &Block -> Vec<ParseError> @ block shared
stmt : &mut Parser -> Result<Stmt, ParseError> ! fail ParseError @ p shared
