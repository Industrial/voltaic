# voltaic 2 signature sidecar — generated, do not edit
emit_templates : &mut Sink<'_> -> &[Ident] -> &DataDecl -> usize -> () @ out shared, named shared, d shared, indent shared
expanded_template : &DeriveTemplate -> &DataDecl -> usize -> String ! mut out @ template shared, d shared, indent shared
field_names : &DataDecl -> Vec<String> @ d shared
pattern_text : &Pattern -> String @ pattern owned
expanded : &Pattern -> usize -> Pattern @ pattern owned, depth owned
rebuilt : &Pattern -> usize -> Pattern @ pattern owned, depth shared
substituted : &Pattern -> &str -> &Pattern -> Pattern @ body owned, param owned, actual owned
enter : SynonymTable -> SynonymScope @ table shared
lookup : &str -> Option<voltaic_ast::PatternSynonym> ! none @ name shared
drop : SynonymScope -> () @ self shared
from_items : &[Item] -> SynonymTable @ items shared
get : &str -> SynonymTable -> Option<PatternSynonym> ! none @ name shared, self shared
SynonymScope.previous field
SynonymScope.enter method
SynonymScope.lookup method
SynonymScope.drop method
SynonymTable.by_name field
SynonymTable.from_items method
SynonymTable.get method
