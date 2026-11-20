# voltaic 2 signature sidecar — generated, do not edit
can_end_a_type : &TokenKind -> bool @ kind shared
capture_type : &mut Parser -> &str -> TypeStop -> Result<TypeExpr, ParseError> ! fail ParseError @ p owned, message owned, stop shared
clause : Vec<Pattern> -> &Ident -> bool -> &mut Parser -> Result<Clause, ParseError> ! fail ParseError + panic @ row owned, name shared, returns_unit shared, p shared
rust_generic : &str -> String @ name owned
signature_from : &[TypeExpr] -> &[Pattern] -> Span -> Result<Signature, ParseError> ! fail ParseError @ spine shared, bound shared, span owned
lend_names : &mut Signature -> &[Clause] -> &[(usize, usize)] -> &Parser -> () @ sig unique, clauses shared, ranges shared, p shared
sole_equation_attributes : &[Clause] -> Result<(), ParseError> ! fail ParseError @ clauses shared
lendable : &str -> usize -> &[Clause] -> &[(usize, usize)] -> &Parser -> bool @ name shared, i shared, clauses shared, ranges shared, p shared
signature_unsigned : &[Pattern] -> Span -> Signature @ bound shared, span shared
ty : &str -> TypeExpr @ text owned
id : &str -> Pattern @ text owned
signature_only : &[TypeExpr] -> Span -> Signature @ spine shared, span shared
starts_a_field_type : &TokenKind -> bool @ kind shared
type_app : &mut Parser -> Result<TypeApp, ParseError> ! fail ParseError + panic @ p shared
type_vars : &[&str] -> Vec<String> @ types shared
SignatureHead.types field
SignatureHead.bounds field
SignatureHead.r#given field
SignatureHead.predicates field
SignatureHead.is_const field
SignatureHead.is_async field
SignatureHead.is_unsafe field
