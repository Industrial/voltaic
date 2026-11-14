# voltaic 2 signature sidecar — generated, do not edit
hole : Variable -> Ty @ v owned
named : &str -> Ty @ name owned
applied : &str -> Vec<Ty> -> Ty @ name owned, args shared
arrow : Ty -> Ty -> Ty @ from shared, to shared
unit : Ty
is_arrow : Ty -> bool @ self shared
is_resolved : Ty -> bool @ self shared
holes : Ty -> Vec<Variable> @ self shared
collect_holes : &mut Vec<Variable> -> Ty -> () @ out shared, self shared
congruent : &Ty -> &Ty -> bool @ left shared, right shared
same_name : &str -> &str -> bool @ left shared, right owned
fmt : Ty -> &mut Formatter<'_> -> Result @ self shared, f shared
Ty.0 field
Ty.name field
Ty.args field
Ty.hole method
Ty.named method
Ty.applied method
Ty.arrow method
Ty.unit method
Ty.is_arrow method
Ty.is_resolved method
Ty.holes method
Ty.collect_holes method
Ty.fmt method
