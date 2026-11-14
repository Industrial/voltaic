# voltaic 2 signature sidecar — generated, do not edit
monomorphic : Ty -> Scheme @ body shared
is_polymorphic : Scheme -> bool @ self shared
arity : Scheme -> usize @ self shared
generalise : &Ty -> &[Variable] -> Scheme @ t owned, bound_outside shared
instantiate : &Scheme -> &mut Store -> Ty @ scheme shared, store shared
Scheme.quantified field
Scheme.body field
Scheme.monomorphic method
Scheme.is_polymorphic method
Scheme.arity method
