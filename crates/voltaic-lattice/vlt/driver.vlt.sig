# voltaic 2 signature sidecar — generated, do not edit
is_solved : Outcome -> bool @ self shared
resolved : &Ty -> Outcome -> Ty @ t owned, self shared
contradictions : &Store -> Outcome -> Vec<&Failure> @ store shared, self shared
solve : &Store -> Outcome @ store owned
solve_with : &Store -> &crate::declarations::Declarations -> Outcome @ store shared, known shared
apply : usize -> &crate::constraint::Constraint -> &crate::declarations::Declarations -> &mut Substitution -> &mut Vec<Failure> -> &mut Vec<usize> -> () @ at shared, c shared, known shared, subst shared, failures shared, deferred shared
receiver_owner : &Store -> &Outcome -> Span -> Option<String> ! none @ store shared, outcome shared, at shared
owner_of : &Ty -> Option<String> ! none @ resolved shared
is_contradiction : &Store -> &Failure -> bool @ store shared, failure shared
arrow_of : &[Ty] -> &Ty -> Ty @ args shared, result shared
Failure.at field
Failure.mismatch field
Outcome.substitution field
Outcome.failures field
Outcome.rounds field
Outcome.deferred field
Outcome.is_solved method
Outcome.resolved method
Outcome.contradictions method
