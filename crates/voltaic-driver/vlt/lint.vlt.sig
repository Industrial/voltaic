# voltaic 2 signature sidecar — generated, do not edit
lint : &Path -> &str -> Option<&Program> -> Vec<Diagnostic> @ path shared, source shared, program shared
allow_pragma : &Trivia -> Lint -> bool ! none @ trivium shared, lint shared
attribute_pragma : &Trivia -> Option<Lint> ! none @ trivium_attribute_pragma shared
trivium_attribute_pragma : TriviaKind -> &str -> Trivia @ kind shared, text owned
binding_name : &Binding -> Option<&Ident> ! none @ binding shared
comment_pragma : &Trivia -> Option<Lint> ! none @ trivium_comment_pragma shared
trivium_comment_pragma : TriviaKind -> &str -> Trivia @ kind shared, text owned
comment_pragma_message : Lint -> String @ lint shared
comment_pragma_sites : &str -> Vec<(Span, Lint)> @ source shared
line_of : &str -> usize -> usize @ source shared, offset shared
item : PragmaWalk -> &Item -> () @ self shared, item shared
stmts : PragmaWalk -> &[Stmt] -> () @ self shared, stmts shared
item : ?95 -> &Item -> () @ self shared, __vlt_a1 shared
stmts : ?98 -> &[Stmt] -> () @ self shared, __vlt_a1 shared
shadow_message : &str -> String @ name shared
shadow_sites : &Program -> Vec<(Span, String)> @ program shared
item : ShadowWalk -> &Item -> () @ self shared, _item shared
stmts : ShadowWalk -> &[Stmt] -> () @ self shared, stmts shared
suppressed_spans : &Program -> Lint -> Vec<Span> @ program shared, lint shared
unwrap_call_sites : &str -> Option<&Program> -> Vec<Span> @ source shared, program shared
walk_item : &Item -> &mut dyn ProgramWalk -> () @ item shared, walk shared
walk_member : &ClassMember -> &mut dyn ProgramWalk -> () @ member shared, walk shared
walk_program : &Program -> &mut dyn ProgramWalk -> () @ program shared, walk shared
walk_stmts : &[Stmt] -> &mut dyn ProgramWalk -> () @ stmts shared, walk shared
PragmaWalk.lint field
PragmaWalk.found field
PragmaWalk.item method
PragmaWalk.stmts method
ShadowWalk.found field
ShadowWalk.item method
ShadowWalk.stmts method
