# voltaic 2 signature sidecar — generated, do not edit
is_written : Obligation -> bool @ self shared
new : Obligation -> Provenance -> Constraint @ obligation shared, provenance shared
holes : Constraint -> Vec<Variable> @ self shared
types : Constraint -> Vec<&Ty> @ self shared
Obligation.left field
Obligation.right field
Obligation.recv field
Obligation.name field
Obligation.result field
Obligation.callee field
Obligation.args field
Obligation.is_written method
Constraint.obligation field
Constraint.provenance field
Constraint.new method
Constraint.holes method
Constraint.types method
