# voltaic 2 signature sidecar — generated, do not edit
judged : &crate::driver::Outcome -> Inferred -> Ty @ outcome shared, self owned
declared : Inferred -> Option<Ty> ! none @ self owned
self_free : &Ty -> &Inferred -> Ty @ t owned, definition shared
find : &str -> Report -> Option<&Inferred> ! none @ name shared, self shared
walk : &Program -> Report @ program shared
published_constants : &Program -> Vec<(String, Ty)> @ program shared
infer_all : &Program -> &[(String, Ty)] -> (Store, Vec<Inferred>) @ program shared, globals shared
grown_globals : &[Inferred] -> &crate::driver::Outcome -> &[(String, Ty)] -> Vec<(String, Ty)> @ definitions shared, outcome shared, globals shared
raise_receivers : &mut [Inferred] -> &Program -> () @ definitions shared, program shared
written_globals : &Program -> Vec<(String, Ty)> @ program shared
raise_rows : &mut [Inferred] -> &Program -> () @ definitions shared, program shared
row_table : &[Inferred] -> Vec<(String, crate::row::Row)> @ definitions shared
join_into : &mut [Inferred] -> &str -> &crate::row::Row -> bool @ definitions shared, name shared, reached shared
call_edges : &Program -> Vec<(String, String, String)> @ program shared
methods_of : &Item -> Vec<&voltaic_ast::FnDecl> @ item shared
receiver_mode : &[Inferred] -> &str -> Option<crate::mode::Mode> ! none @ definitions shared, name shared
raise_in : &mut [Inferred] -> &str -> &str -> crate::mode::Mode -> bool @ definitions shared, caller shared, binder shared, needed shared
one : &voltaic_ast::FnDecl -> Option<&str> -> &mut Store -> &[(String, Ty)] -> Inferred @ f shared, owner owned, store shared, globals shared
is_published : &Visibility -> bool @ visibility shared
modes_of : &voltaic_ast::Signature -> &[voltaic_ast::Clause] -> Vec<(String, crate::mode::Mode)> @ sig shared, clauses owned
written_modes_of : &voltaic_ast::Signature -> Vec<(String, crate::mode::Mode)> @ sig shared
row_of : &voltaic_ast::Signature -> &[voltaic_ast::Clause] -> crate::row::Row ! mut found @ sig shared, clauses shared
signature_type : &voltaic_ast::Signature -> Option<&str> -> Option<Ty> ! none @ sig shared, owner shared
clause_type : &[voltaic_ast::Clause] -> &mut Store -> &[(String, Ty)] -> Ty @ clauses shared, store shared, env shared
block_type : &Block -> &mut Store -> &[(String, Ty)] -> Ty @ block shared, store shared, env shared
expr_type : &Expr -> &mut Store -> &[(String, Ty)] -> Ty @ e shared, store owned, env owned
str_ref : Ty
binary_type : BinaryOp -> Ty -> Ty -> voltaic_ast::Span -> &mut Store -> Ty @ op shared, left owned, right owned, span shared, store owned
is_comparison : BinaryOp -> bool @ op shared
is_arithmetic : BinaryOp -> bool @ op shared
lookup : &str -> &mut Store -> &[(String, Ty)] -> Ty @ name shared, store owned, env shared
GLOBAL_ROUNDS : u32
ROW_ROUNDS : u32
Inferred.name field
Inferred.ty field
Inferred.row field
Inferred.modes field
Inferred.written field
Inferred.written_modes field
Inferred.published field
Inferred.owner field
Inferred.span field
Inferred.judged method
Inferred.declared method
Report.store field
Report.definitions field
Report.known field
Report.constants field
Report.find method
