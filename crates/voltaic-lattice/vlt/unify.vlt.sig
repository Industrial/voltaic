# voltaic 2 signature sidecar — generated, do not edit
between : &Ty -> &Ty -> Mismatch @ left owned, right owned
cycle : Variable -> &Ty -> Mismatch @ v owned, right owned
is_cycle : Mismatch -> bool @ self shared
is_ground_conflict : Mismatch -> bool @ self shared
unify : &Ty -> &Ty -> &mut Substitution -> Result<(), Mismatch> ! fail Mismatch @ left shared, right shared, subst owned
bind : Variable -> &Ty -> &mut Substitution -> Result<(), Mismatch> ! fail Mismatch @ v owned, t owned, subst shared
unify_constructors : &Ty -> &Ty -> &mut Substitution -> Result<(), Mismatch> ! fail Mismatch @ l owned, r owned, subst shared
unify_all : &[Ty] -> &[Ty] -> &mut Substitution -> Result<(), Mismatch> ! fail Mismatch @ left shared, right shared, subst shared
Mismatch.left field
Mismatch.right field
Mismatch.occurs field
Mismatch.between method
Mismatch.cycle method
Mismatch.is_cycle method
Mismatch.is_ground_conflict method
