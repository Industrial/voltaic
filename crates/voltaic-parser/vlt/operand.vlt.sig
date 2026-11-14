# voltaic 2 signature sidecar — generated, do not edit
assign_tail : &mut Parser -> Expr -> usize -> Result<Expr, ParseError> ! fail ParseError @ p shared, target owned, start owned
bracketed : &mut Parser -> Result<Expr, ParseError> ! fail ParseError + panic @ p shared
byte_literal : u8 -> String @ value shared
compound_assign : &mut Parser -> Option<BinaryOp> ! none @ p shared
constructor_path : &mut Parser -> Option<String> ! none @ p shared
field_init : &mut Parser -> Result<FieldInit, ParseError> ! fail ParseError @ p shared
interpolated_string : &[StrPart] -> Span -> Result<Expr, ParseError> ! fail ParseError @ parts shared, span owned
lambda : &mut Parser -> Result<Expr, ParseError> ! fail ParseError + panic @ p shared
macro_call : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p shared
method_call_syntax : &str -> Span -> ParseError @ name shared, span owned
for_class : &[Ident] -> MethodContext @ params shared
for_instance : Option<&str> -> &str -> MethodContext @ var shared, subject shared
reserved : MethodContext -> Vec<String> @ self shared
opens_a_section : &TokenKind -> bool @ kind shared
opens_an_over : &Parser -> bool @ p shared
optional_chaining : Span -> ParseError @ span owned
over_update : &mut Parser -> usize -> Result<Expr, ParseError> ! fail ParseError + panic @ p shared, start owned
path_or_ident : &mut Parser -> Result<Expr, ParseError> ! fail ParseError @ p shared
pipeline_section : &str -> Span -> ParseError @ name shared, span owned
range_or_literal : &mut Parser -> String -> Span -> Pattern @ p shared, lo shared, span shared
is_receiver : &Param -> bool @ param shared
receiver_name : &Param -> String @ param shared
receiver : &mut Signature -> () ! mut sig @ sig unique
receiver_text : &str -> Option<String> ! none @ ty shared
record_literal : &mut Parser -> Expr -> Result<Expr, ParseError> ! fail ParseError + panic @ p shared, head owned
section : &mut Parser -> usize -> Result<Option<Expr>, ParseError> ! fail ParseError + panic @ p shared, start owned
try_await : &mut Parser -> Result<Option<Expr>, ParseError> ! fail ParseError + panic @ p shared
ungrouped : &str -> String @ text owned
unit_tail : &Ident -> &Block -> Result<(), ParseError> ! fail ParseError @ name shared, block shared
MethodContext.vars field
MethodContext.recording field
MethodContext.implementing field
MethodContext.for_class method
MethodContext.for_instance method
MethodContext.reserved method
