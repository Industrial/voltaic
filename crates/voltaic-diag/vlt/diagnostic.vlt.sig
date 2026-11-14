# voltaic 2 signature sidecar — generated, do not edit
is_fatal : Severity -> bool @ self shared
to_miette : Severity -> miette::Severity @ self shared
fmt : Diagnostic -> &mut Formatter<'_> -> Result @ self shared, f shared
Severity.is_fatal method
Severity.to_miette method
Diagnostic.code field
Diagnostic.severity field
Diagnostic.message field
Diagnostic.span field
Diagnostic.related field
Diagnostic.help field
Diagnostic.path field
Diagnostic.source field
Diagnostic.fmt method
