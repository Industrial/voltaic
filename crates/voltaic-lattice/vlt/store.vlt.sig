# voltaic 2 signature sidecar — generated, do not edit
new : Store
fresh : Store -> Variable ! mut self @ self unique
fresh_hole : Store -> Ty ! mut self @ self owned
push : Obligation -> Provenance -> Store -> usize ! none @ obligation shared, provenance shared, self shared
require : Obligation -> Span -> Reason -> Store -> usize ! none @ obligation owned, span owned, reason owned, self owned
require_from : usize -> Obligation -> Span -> Reason -> Store -> usize ! none @ parent owned, obligation owned, span owned, reason owned, self owned
len : Store -> usize @ self shared
is_empty : Store -> bool @ self shared
variables : Store -> u32 @ self shared
at : usize -> Store -> Option<&Constraint> ! none @ index owned, self shared
all : Store -> &[Constraint] @ self shared
chain : usize -> Store -> Vec<Provenance> @ index shared, self shared
chain_indices : usize -> Store -> Vec<usize> @ index shared, self shared
Store.constraints field
Store.next field
Store.new method
Store.fresh method
Store.fresh_hole method
Store.push method
Store.require method
Store.require_from method
Store.len method
Store.is_empty method
Store.variables method
Store.at method
Store.all method
Store.chain method
Store.chain_indices method
