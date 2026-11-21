# voltaic 2 signature sidecar — generated, do not edit
expr_text : &Expr -> u8 -> usize -> &str -> String @ expr owned, required owned, indent owned, source owned
argument : &Expr -> usize -> &str -> String @ expr owned, indent owned, source owned
is_unit : &Expr -> bool @ expr shared
stands_alone : &Expr -> bool @ expr shared
can_drop_paren : &Expr -> u8 -> bool ! none @ inner owned, required shared
comma_list : &[Expr] -> usize -> &str -> String @ items shared, indent shared, source shared
comprehension_text : &Expr -> &[Qualifier] -> usize -> &str -> String @ element shared, qualifiers shared, indent shared, source shared
dollar_text : &Expr -> &Expr -> usize -> &str -> String @ callee shared, arg shared, indent shared, source shared
lambda_text : &[Param] -> &Expr -> bool -> usize -> &str -> String @ params shared, body shared, is_move shared, indent shared, source shared
param_text : &Param -> String @ param shared
literal_text : &Expr -> &str -> String @ expr owned, source shared
stored : &Expr -> String @ expr shared
macro_call_text : &str -> &[Expr] -> usize -> &str -> String @ name shared, args shared, indent shared, source shared
operand : &Expr -> u8 -> usize -> &str -> String @ expr owned, required owned, indent owned, source owned
pipeline_text : &Expr -> &Expr -> usize -> &str -> String @ lhs shared, rhs shared, indent shared, source shared
power_of : &Expr -> Option<u8> ! none @ expr shared
reaches_right : &Expr -> bool @ expr shared
open_on_the_right : &Expr -> bool @ expr owned
deref_text : &Expr -> &str @ e shared
receiver : &Expr -> usize -> &str -> String @ expr owned, indent owned, source owned
no_real_arguments : &[Expr] -> bool @ args shared
takes_a_postfix : &Expr -> bool @ expr shared
record_text : &Expr -> &[FieldInit] -> Option<&Expr> -> usize -> &str -> String @ head shared, fields shared, base shared, indent shared, source shared
rust_hatch_text : &str -> usize -> String @ code shared, indent shared
macro_verbatim_text : &str -> char -> &str -> bool -> usize -> String @ name shared, open shared, body shared, indented shared, indent shared
section_text : &Section -> usize -> &str -> String @ section shared, indent shared, source shared
