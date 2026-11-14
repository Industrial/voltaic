# voltaic 2 signature sidecar — generated, do not edit
empty : Row
single : Effect -> Row @ effect shared
from_effects : Vec<Effect> -> Row @ every shared
is_empty : Row -> bool @ self shared
len : Row -> usize @ self shared
holds : &Effect -> Row -> bool @ effect owned, self shared
effects : Row -> Vec<&Effect> @ self shared
join : &Row -> Row -> Row @ other shared, self shared
is_subrow_of : &Row -> Row -> bool @ other shared, self shared
without : &Effect -> Row -> Row @ effect shared, self shared
discharge_fallible : Row -> Row @ self shared
fallible : Row -> Vec<&Effect> @ self shared
fmt : Row -> &mut Formatter<'_> -> Result @ self shared, f shared
Row.effects field
Row.empty method
Row.single method
Row.from_effects method
Row.is_empty method
Row.len method
Row.holds method
Row.effects method
Row.join method
Row.is_subrow_of method
Row.without method
Row.discharge_fallible method
Row.fallible method
Row.fmt method
