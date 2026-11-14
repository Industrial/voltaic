# voltaic 2 signature sidecar — generated, do not edit
at : Span -> Reason -> Provenance @ span shared, reason shared
from : usize -> Span -> Reason -> Provenance @ parent owned, span shared, reason shared
is_root : Provenance -> bool @ self shared
Provenance.span field
Provenance.reason field
Provenance.derived_from field
Provenance.at method
Provenance.from method
Provenance.is_root method
