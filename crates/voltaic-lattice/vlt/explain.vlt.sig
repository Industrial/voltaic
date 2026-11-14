# voltaic 2 signature sidecar — generated, do not edit
steps : usize -> &Store -> Vec<Step> @ index shared, store shared
explain : usize -> &Store -> String @ index shared, store shared
rendered : usize -> &Store -> &str -> String @ index shared, store shared, source shared
summarise : &Obligation -> String @ obligation shared
Step.depth field
Step.phrasing field
Step.summary field
Step.span field
