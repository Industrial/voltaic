# voltaic 2 signature sidecar — generated, do not edit
enter : MethodTable -> MethodScope @ table shared
is_method : Span -> bool @ at shared
drop : MethodScope -> () @ self shared
from_spans : &[Span] -> MethodTable @ every owned
holds : Span -> MethodTable -> bool @ at shared, self shared
len : MethodTable -> usize @ self shared
is_empty : MethodTable -> bool @ self shared
MethodScope.previous field
MethodScope.enter method
MethodScope.is_method method
MethodScope.drop method
MethodTable.spans field
MethodTable.from_spans method
MethodTable.holds method
MethodTable.len method
MethodTable.is_empty method
