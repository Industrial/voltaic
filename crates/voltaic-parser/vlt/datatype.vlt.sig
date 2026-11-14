# voltaic 2 signature sidecar — generated, do not edit
constructor : &mut Parser -> Vec<voltaic_lexer::Trivia> -> Result<Constructor, ParseError> ! fail ParseError @ p shared, leading shared
trailing_message : &mut Parser -> Result<Option<String>, ParseError> ! fail ParseError @ p shared
constructor_fields : &mut Parser -> Result<ConstructorFields, ParseError> ! fail ParseError + panic @ p shared
data_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<DataDecl, ParseError> ! fail ParseError + mut d @ p shared, leading shared, start owned, visibility shared
derive_template : &mut Parser -> Vec<Trivia> -> usize -> Result<DeriveTemplate, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
r#deriving : &mut Parser -> Result<Vec<Ident>, ParseError> ! fail ParseError + panic @ p shared
error_variant : &mut Parser -> Result<ErrorVariant, ParseError> ! fail ParseError @ p shared
errors_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<ErrorsDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned, visibility shared
field_decl : &mut Parser -> Result<FieldDecl, ParseError> ! fail ParseError @ p shared
generalise : &mut Signature -> &[Constraint] -> &[String] -> () ! mut sig @ sig unique, constraints shared, reserved shared
generalise_data : &mut DataDecl -> () ! mut d @ d unique
newtype_shape : &DataDecl -> Result<(), ParseError> ! fail ParseError @ decl shared
