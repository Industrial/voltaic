# voltaic 2 signature sidecar — generated, do not edit
class_body : &mut Parser -> &MethodContext -> Result<Vec<ClassMember>, ParseError> ! fail ParseError + panic @ p shared, ctx shared
class_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<ClassDecl, ParseError> ! fail ParseError + mut ctx + panic @ p shared, leading shared, start owned, visibility shared
class_member : &mut Parser -> &MethodContext -> Result<ClassMember, ParseError> ! fail ParseError + mut decl + panic @ p owned, ctx shared
class_supers : &mut Parser -> Result<Vec<Ident>, ParseError> ! fail ParseError @ p shared
constraints : &mut Parser -> Result<Vec<Constraint>, ParseError> ! fail ParseError @ p shared
given_clause : &mut Parser -> Result<Option<String>, ParseError> ! fail ParseError @ p owned
angle_group : &mut Parser -> Option<String> ! none @ p shared
instance_decl : &mut Parser -> Vec<Trivia> -> usize -> Result<InstanceDecl, ParseError> ! fail ParseError + mut ctx + panic @ p shared, leading shared, start owned
walk_member_for_tails : &ClassMember -> &mut Vec<ParseError> -> () @ member shared, problems shared
