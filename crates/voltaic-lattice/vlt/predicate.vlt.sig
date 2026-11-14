# voltaic 2 signature sidecar — generated, do not edit
literal : i64 -> Term @ constant shared
variable : &str -> Term @ name shared
length_of : &str -> Term @ name shared
is_constant : Term -> bool @ self shared
plus : &Term -> Term -> Term @ other shared, self shared
scaled : i64 -> Term -> Term @ factor shared, self shared
minus : &Term -> Term -> Term @ other shared, self owned
mentions : Term -> Vec<&String> @ self shared
fmt : Term -> &mut Formatter<'_> -> Result @ self shared, f shared
label : Comparison -> &str @ self shared
negated : Comparison -> Comparison @ self shared
always : Predicate
never : Predicate
compare : Term -> Comparison -> Term -> Predicate @ left shared, op shared, right shared
mentions : Predicate -> Vec<String> @ self shared
collect_mentions : &mut Vec<String> -> Predicate -> () @ out shared, self shared
fmt : Predicate -> &mut Formatter<'_> -> Result @ self shared, f shared
Term.weights field
Term.constant field
Term.literal method
Term.variable method
Term.length_of method
Term.is_constant method
Term.plus method
Term.scaled method
Term.minus method
Term.mentions method
Term.fmt method
Comparison.label method
Comparison.negated method
Predicate.left field
Predicate.op field
Predicate.right field
Predicate.0 field
Predicate.always method
Predicate.never method
Predicate.compare method
Predicate.mentions method
Predicate.collect_mentions method
Predicate.fmt method
