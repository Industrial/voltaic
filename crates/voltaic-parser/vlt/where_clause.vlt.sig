# voltaic 2 signature sidecar — generated, do not edit
where_entry_is_a_function : &Parser -> bool @ p owned
where_group : &[Stmt] -> &[Pattern] -> Result<(), ParseError> ! fail ParseError @ entries shared, params shared
where_locals : &[Pattern] -> &mut Parser -> Result<Vec<Stmt>, ParseError> ! fail ParseError + panic @ params shared, p shared
where_predicates : &mut Parser -> Result<Option<String>, ParseError> ! fail ParseError @ p shared
