# voltaic 2 signature sidecar — generated, do not edit
parse : &[Token] -> usize -> Result<Program, Vec<ParseError>> ! fail Vec<ParseError> @ toks owned, src_len shared
new : &[Token] -> Parser @ toks shared
run : usize -> Parser -> Result<Program, Vec<ParseError>> ! fail Vec<ParseError> @ src_len owned, self shared
program : Parser -> Vec<Item> @ self owned
items_until_dedent : Parser -> Result<Vec<Item>, ParseError> ! fail ParseError + panic @ self owned
item : Parser -> Result<Item, ParseError> ! fail ParseError + mut decl + mut self @ self owned
function : &[String] -> Parser -> Result<FnDecl, ParseError> ! fail ParseError @ reserved owned, self owned
where_function : Parser -> Result<FnDecl, ParseError> ! fail ParseError + mut settled @ self owned
signed : Parser -> bool @ self owned
peeked_name : &str -> Parser -> Result<Ident, ParseError> ! fail ParseError @ message owned, self owned
signature_head : &Ident -> Parser -> Result<SignatureHead, ParseError> ! fail ParseError + mut self + panic @ name shared, self owned
record_head : &str -> &str -> &SignatureHead -> Parser -> () @ class_name shared, member shared, head shared, self shared
borrowed_head : &str -> &str -> Parser -> Option<SignatureHead> ! none @ class_name shared, member shared, self shared
equations : Ident -> Vec<Trivia> -> usize -> &SignatureHead -> &[String] -> bool -> Parser -> Result<FnDecl, ParseError> ! fail ParseError + mut settled @ name shared, leading shared, start owned, head shared, reserved shared, ends_on_a_new_name shared, self owned
unsigned_equations : Ident -> Vec<Trivia> -> usize -> &[String] -> bool -> Parser -> Result<FnDecl, ParseError> ! fail ParseError @ name shared, leading shared, start owned, reserved shared, ends_on_a_new_name shared, self owned
statements : Parser -> Result<Block, ParseError> ! fail ParseError + panic @ self owned
indented_block : Code -> &str -> Parser -> Result<Block, ParseError> ! fail ParseError + mut self + panic @ code shared, message shared, self owned
statement_block : &str -> Parser -> Result<Block, ParseError> ! fail ParseError + mut self + panic @ message owned, self owned
at_statement_end : Parser -> bool @ self shared
if_statement : Parser -> Result<Expr, ParseError> ! fail ParseError + mut self + panic @ self owned
inline_body : Parser -> Result<Block, ParseError> ! fail ParseError @ self shared
opens_a_guard_list : Parser -> bool ! none @ self shared
guards : Parser -> Result<Vec<Guard>, ParseError> ! fail ParseError @ self owned
is_wildcard : Parser -> bool @ self shared
names_the_retired_catch_all : Parser -> bool @ self shared
try_draw : Parser -> Result<Option<Expr>, ParseError> ! fail ParseError + mut self + panic @ self owned
try_binding : Parser -> Result<Option<Binding>, ParseError> ! fail ParseError + mut self + panic @ self owned
else_tail : Parser -> Result<Option<Block>, ParseError> ! fail ParseError + mut self + panic @ self owned
one_statement : Expr -> usize -> Parser -> Block @ expr shared, start owned, self shared
value_or_ladder : Parser -> Result<Expr, ParseError> ! fail ParseError + mut self + panic @ self owned
looks_like_a_binding : Parser -> bool @ self owned
reaches_an_else_tail : Parser -> bool @ self shared
reaches_a_binding_eq : Parser -> bool @ self shared
type_expr : &str -> Parser -> Result<TypeExpr, ParseError> ! fail ParseError @ message owned, self owned
peek : Parser -> &Token @ self shared
peek_at : usize -> Parser -> Option<&Token> ! none @ n shared, self shared
peek_span : Parser -> Span @ self shared
peek_leading : Parser -> Vec<Trivia> @ self shared
prev_span : Parser -> Span @ self shared
at_eof : Parser -> bool @ self shared
check : &TokenKind -> Parser -> bool @ kind shared, self shared
advance : Parser -> () ! mut self @ self unique
expect : &TokenKind -> &str -> Parser -> Result<(), ParseError> ! fail ParseError + mut self @ kind owned, message owned, self owned
expect_with : Code -> &TokenKind -> &str -> Parser -> Result<(), ParseError> ! fail ParseError + mut self @ code owned, kind shared, message owned, self owned
eat_keyword : Keyword -> Parser -> bool ! mut self @ kw shared, self unique
mentions : &str -> (usize, usize) -> Parser -> bool @ name shared, range shared, self shared
position : Parser -> usize @ self shared
rewind : usize -> Parser -> () ! mut self @ mark shared, self unique
try_ident : Parser -> Option<Ident> ! none + mut self @ self unique
ident : &str -> Parser -> Result<Ident, ParseError> ! fail ParseError + mut self @ message owned, self owned
newline_with_trailing : &str -> Parser -> Result<Option<Trivia>, ParseError> ! fail ParseError + mut self @ message owned, self owned
just_dedented : Parser -> bool @ self shared
skip_separators : Parser -> () @ self unique
skip_layout : Parser -> () @ self unique
eof_trivia : Parser -> Vec<Trivia> @ self shared
recover_to_item : Parser -> () @ self unique
recover_to_stmt : Parser -> () @ self unique
Parser.toks field
Parser.pos field
Parser.r#errors field
Parser.class_heads field
Parser.fixities field
Parser.new method
Parser.run method
Parser.program method
Parser.items_until_dedent method
Parser.item method
Parser.function method
Parser.where_function method
Parser.signed method
Parser.peeked_name method
Parser.signature_head method
Parser.record_head method
Parser.borrowed_head method
Parser.equations method
Parser.unsigned_equations method
Parser.statements method
Parser.indented_block method
Parser.statement_block method
Parser.at_statement_end method
Parser.if_statement method
Parser.inline_body method
Parser.opens_a_guard_list method
Parser.guards method
Parser.is_wildcard method
Parser.names_the_retired_catch_all method
Parser.try_draw method
Parser.try_binding method
Parser.else_tail method
Parser.one_statement method
Parser.value_or_ladder method
Parser.looks_like_a_binding method
Parser.reaches_an_else_tail method
Parser.reaches_a_binding_eq method
Parser.type_expr method
Parser.peek method
Parser.peek_at method
Parser.peek_span method
Parser.peek_leading method
Parser.prev_span method
Parser.at_eof method
Parser.check method
Parser.advance method
Parser.expect method
Parser.expect_with method
Parser.eat_keyword method
Parser.mentions method
Parser.position method
Parser.rewind method
Parser.try_ident method
Parser.ident method
Parser.newline_with_trailing method
Parser.just_dedented method
Parser.skip_separators method
Parser.skip_layout method
Parser.eof_trivia method
Parser.recover_to_item method
Parser.recover_to_stmt method
