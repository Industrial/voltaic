# voltaic 2 signature sidecar — generated, do not edit
empty : Declarations
len : Declarations -> usize @ self shared
is_empty : Declarations -> bool @ self shared
alias : &str -> Declarations -> Option<&Ty> ! none @ name shared, self shared
expand : &Ty -> Declarations -> Ty @ t owned, self owned
knows : &str -> Declarations -> bool @ name shared, self shared
member : &str -> &str -> Declarations -> Option<&Ty> ! none @ owner shared, name shared, self shared
kind_of : &str -> &str -> Declarations -> Option<MemberKind> ! none @ owner shared, name shared, self shared
is_field : &str -> &str -> Declarations -> bool ! none @ owner shared, name shared, self shared
is_method : &str -> &str -> Declarations -> bool ! none @ owner shared, name shared, self shared
members_of : &str -> Declarations -> Vec<&Member> @ owner shared, self shared
requires : &str -> Declarations -> Vec<&Requirement> @ class_name shared, self shared
satisfies : &str -> &str -> Declarations -> bool @ owner shared, class_name shared, self shared
expand_within : &Ty -> &Declarations -> Vec<String> -> Ty @ t owned, known owned, seen shared
collect : &Program -> Declarations @ program shared
Member.owner field
Member.name field
Member.ty field
Member.kind field
Requirement.class_name field
Requirement.name field
Alias.name field
Alias.ty field
Declarations.members field
Declarations.requirements field
Declarations.types field
Declarations.aliases field
Declarations.empty method
Declarations.len method
Declarations.is_empty method
Declarations.alias method
Declarations.expand method
Declarations.knows method
Declarations.member method
Declarations.kind_of method
Declarations.is_field method
Declarations.is_method method
Declarations.members_of method
Declarations.requires method
Declarations.satisfies method
