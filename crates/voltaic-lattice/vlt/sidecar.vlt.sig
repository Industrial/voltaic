# voltaic 2 signature sidecar — generated, do not edit
new : Sidecar
len : Sidecar -> usize @ self shared
is_empty : Sidecar -> bool @ self shared
find : &str -> Sidecar -> Option<&Published> ! none @ name shared, self shared
is_breaking : Change -> bool @ self shared
name : Change -> &str @ self shared
fmt : Change -> &mut Formatter<'_> -> Result @ self shared, f shared
member : &str -> &str -> Sidecar -> Option<&Member> ! none @ owner shared, name shared, self shared
declares : &str -> Sidecar -> bool @ owner shared, self shared
judgement_of : &crate::generate::Inferred -> &crate::driver::Outcome -> String @ definition shared, outcome shared
of_report : &Report -> &crate::driver::Outcome -> Sidecar @ found shared, outcome owned
render_sidecar : &Sidecar -> String @ sidecar shared
read_sidecar : &str -> Sidecar @ text shared
compare : &Sidecar -> &Sidecar -> Vec<Change> @ before shared, after shared
unwritten : &Report -> &crate::driver::Outcome -> Vec<Unwritten> @ found shared, outcome owned
breaking : &[Change] -> usize @ every shared
Published.name field
Published.judgement field
Member.owner field
Member.name field
Member.is_field field
Sidecar.published field
Sidecar.members field
Sidecar.new method
Sidecar.len method
Sidecar.is_empty method
Sidecar.find method
Change.0 field
Change.name field
Change.before field
Change.after field
Change.is_breaking method
Change.name method
Change.fmt method
Sidecar.member method
Sidecar.declares method
Unwritten.name field
Unwritten.inferred field
Unwritten.span field
