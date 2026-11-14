# voltaic 2 signature sidecar — generated, do not edit
emit_body : &mut Sink<'_> -> &ClauseBody -> usize -> bool -> bool -> () @ out shared, body shared, indent shared, tail shared, joinable shared
emit_clauses : &mut Sink<'_> -> &FnDecl -> usize -> () @ out shared, f shared, base shared
emit_const : &mut Sink<'_> -> &ConstDecl -> usize -> () @ out shared, c shared, indent shared
emit_fn : &mut Sink<'_> -> &FnDecl -> usize -> () @ out shared, f shared, indent shared
emit_extern_block : &mut Sink<'_> -> &ExternBlock -> usize -> () @ out shared, e shared, indent shared
generics : &TypeApp -> &[Constraint] -> String @ target shared, constraints shared
emit_import : &mut Sink<'_> -> &ImportDecl -> usize -> () @ out shared, i shared, indent shared
emit_inline_rust : &mut Sink<'_> -> &InlineRust -> usize -> () @ out shared, r shared, indent shared
is_shareable : &Expr -> bool @ body shared
emit : &mut Sink<'_> -> usize -> ?182 -> () @ __vlt_a1 shared, __vlt_a2 shared, self shared
emit_item : &mut Sink<'_> -> &Item -> usize -> () @ out shared, item shared, indent shared
emit : &mut Sink<'_> -> usize -> Item -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> FnDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> DataDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> TypeAlias -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ClassDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ClassMember -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> InstanceDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ErrorsDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ImportDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ModuleDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ConstDecl -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> InlineRust -> () @ out owned, indent owned, self owned
emit : &mut Sink<'_> -> usize -> ExternBlock -> () @ out owned, indent owned, self owned
emit_items : &mut Sink<'_> -> &[Item] -> usize -> () @ out shared, items shared, indent shared
emit_module : &mut Sink<'_> -> &ModuleDecl -> usize -> () @ out shared, m shared, indent shared
shared_const : &FnDecl -> Option<ConstDecl> ! none @ decl shared
signature : &Signature -> usize -> String @ sig shared, indent shared
test_slug : &str -> String @ description shared
emit_tests_module : &mut Sink<'_> -> &[&TestDecl] -> usize -> () @ out shared, tests shared, indent shared
emit_type_alias : &mut Sink<'_> -> &TypeAlias -> usize -> () @ out shared, a shared, indent shared
Item.emit method
FnDecl.emit method
DataDecl.emit method
TypeAlias.emit method
ClassDecl.emit method
ClassMember.emit method
InstanceDecl.emit method
ErrorsDecl.emit method
ImportDecl.emit method
ModuleDecl.emit method
ConstDecl.emit method
InlineRust.emit method
ExternBlock.emit method
