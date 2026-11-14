# voltaic 2 signature sidecar — generated, do not edit
label : Effect -> &str @ self shared
is_fallible : Effect -> bool @ self shared
error_type : Effect -> Option<&Ty> ! none @ self shared
region : Effect -> Option<&str> ! none @ self shared
fmt : Effect -> &mut Formatter<'_> -> Result @ self shared, f shared
Effect.0 field
Effect.label method
Effect.is_fallible method
Effect.error_type method
Effect.region method
Effect.fmt method
