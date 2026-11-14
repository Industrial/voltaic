# voltaic 2 signature sidecar — generated, do not edit
item_text : &Item -> usize -> &str -> String @ item shared, indent shared, source shared
class_text : &ClassDecl -> usize -> &str -> String @ decl shared, indent shared, source shared
clause_text : &Clause -> &str -> usize -> &str -> String @ clause shared, name shared, indent shared, source shared
inline_body : &Block -> usize -> &str -> String @ block shared, indent shared, source shared
const_text : &ConstDecl -> &str -> String @ decl shared, source shared
constructor_text : &Constructor -> usize -> String @ constructor shared, indent shared
constructor_head : &Constructor -> usize -> String @ constructor shared, indent shared
headless_text : &Constructor -> usize -> String @ constructor shared, indent shared
data_text : &DataDecl -> usize -> &str -> String @ decl shared, indent shared, source shared
errors_text : &ErrorsDecl -> usize -> String @ decl shared, indent shared
fn_text : &FnDecl -> usize -> &str -> String @ decl shared, indent shared, source shared
instance_text : &InstanceDecl -> usize -> &str -> String @ decl shared, indent shared, source shared
target_text : &TypeApp -> String @ target shared
leading_text : &Item -> usize -> String @ item shared, indent shared
member_text : &ClassMember -> usize -> &str -> String @ member shared, indent shared, source shared
same_line : &str -> usize -> usize -> bool @ source shared, left shared, right shared
signature_line : &FnDecl -> &str -> Option<String> ! none @ decl shared, source shared
visibility_text : &Visibility -> String @ visibility shared
