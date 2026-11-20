# voltaic 2 signature sidecar — generated, do not edit
annotated_binding : &Binding -> (Option<String>, &Expr) @ b shared
selects_a_container : &Expr -> bool @ inner shared
bind_block : &Ident -> &[Stmt] -> usize -> String @ method shared, stmts shared, indent owned
enter : BindMode -> BindScope @ mode owned
mode : BindMode
drop : BindScope -> () @ self shared
enter : BindingTable -> BindingScope @ table shared
kind_of : &str -> Option<BindingKind> ! none @ name shared
drop : BindingScope -> () @ self shared
from_items : &[Item] -> BindingTable @ items shared
gather : &[Item] -> &[String] -> &mut Vec<String> -> BindingTable -> () @ items shared, prefix shared, ambiguous shared, self shared
insert : String -> BindingKind -> &mut Vec<String> -> BindingTable -> () @ key shared, kind shared, ambiguous shared, self shared
kind_of : &str -> BindingTable -> Option<BindingKind> ! none @ name shared, self shared
classify : &FnDecl -> Option<BindingKind> ! none @ decl shared
is_upper_case : &str -> bool @ name shared
binding_text : &str -> String @ name owned
member_word : &str -> String @ name owned
path_word : &str -> String @ name owned
emit_bind : &Option<Pattern> -> &Expr -> usize -> String @ pattern shared, value shared, indent shared
emit_where_locals : &mut Sink<'_> -> &[Stmt] -> usize -> () @ out shared, wheres shared, indent shared
one_tail_expression : &Clause -> bool -> Option<&Stmt> ! none @ clause shared, tail shared
where_order : &[Stmt] -> Vec<usize> @ entries shared
visit : usize -> &[Vec<usize>] -> &mut [bool] -> &mut [bool] -> &mut Vec<usize> -> () ! mut done + mut visiting @ at shared, graph shared, done unique, visiting unique, sorted shared
BindScope.previous field
BindScope.enter method
BindScope.mode method
BindScope.drop method
BindingScope.previous field
BindingScope.enter method
BindingScope.kind_of method
BindingScope.drop method
BindingTable.kinds field
BindingTable.from_items method
BindingTable.gather method
BindingTable.insert method
BindingTable.kind_of method
