# voltaic 2 signature sidecar — generated, do not edit
new : Substitution
len : Substitution -> usize @ self shared
is_empty : Substitution -> bool @ self shared
get : &Variable -> Substitution -> Option<&Ty> ! none @ v owned, self shared
bind : Variable -> Ty -> Substitution -> () @ v shared, t shared, self shared
bound_variables : Substitution -> Vec<Variable> @ self shared
resolve : &Ty -> Substitution -> Ty @ t owned, self owned
Substitution.bound field
Substitution.new method
Substitution.len method
Substitution.is_empty method
Substitution.get method
Substitution.bind method
Substitution.bound_variables method
Substitution.resolve method
