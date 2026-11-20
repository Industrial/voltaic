# voltaic 2 signature sidecar — generated, do not edit
emit_class : &mut Sink<'_> -> &ClassDecl -> usize -> () @ out shared, c shared, indent shared
supported : &str -> Option<&[&'static str]> ! none @ text shared
field_types : &DataDecl -> Vec<String> @ d shared
writes_its_own_derives : &DataDecl -> bool @ d shared
computed_derives : &DataDecl -> Vec<&str> @ d shared
is_error_type : &DataDecl -> bool @ d shared
payload_types : &ConstructorFields -> Vec<TypeExpr> @ fields shared
as_variant : &Constructor -> voltaic_ast::ErrorVariant @ c shared
as_errors : &DataDecl -> ErrorsDecl @ d shared
emit_data : &mut Sink<'_> -> &DataDecl -> usize -> () @ out shared, d shared, indent shared
error_message : &str -> (String, Vec<String>) @ raw shared
emit_errors : &mut Sink<'_> -> &ErrorsDecl -> usize -> () @ out shared, d shared, indent shared
unparenthesised : &str -> &str @ text owned
emit_fields : &mut Sink<'_> -> &ConstructorFields -> &[Visibility] -> usize -> bool -> () @ out shared, fields shared, visibilities shared, indent shared, top shared
emit_instance : &mut Sink<'_> -> &InstanceDecl -> usize -> () @ out shared, i shared, indent shared
emit_member : &mut Sink<'_> -> &ClassMember -> usize -> () @ out shared, m shared, indent shared
names_a_constructor : &Expr -> bool @ head shared
one_field : &FieldInit -> usize -> String @ f shared, indent shared
emit_record : &Expr -> &[FieldInit] -> usize -> String @ head shared, fields shared, indent shared
