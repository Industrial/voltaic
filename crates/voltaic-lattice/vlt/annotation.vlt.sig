# voltaic 2 signature sidecar — generated, do not edit
write_type : &Ty -> Option<String> ! none @ t shared
with_owner : &Ty -> &str -> Ty @ t owned, owner shared
read : &TypeExpr -> Ty @ written shared
from_text : &str -> Ty @ text shared
without_lifetime : &str -> &str @ text owned
split_arrow : &str -> Option<usize> ! none @ text shared
split_arguments : &str -> Vec<Ty> @ body shared
