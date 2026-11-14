# voltaic 2 signature sidecar — generated, do not edit
render : &[Diagnostic] -> bool -> String @ diagnostics shared, plain shared
anchor_span : &str -> Span -> Span @ source shared, span owned
clamped_width : &str -> Span -> usize -> usize @ source shared, span shared, column shared
caret_width : &str -> Span -> usize @ source shared, span shared
line_col : &str -> usize -> (usize, usize) @ source shared, offset shared
snippet_line : &str -> usize -> usize -> Option<(usize, &str, usize)> ! none @ source shared, line owned, col owned
plain_one : &Diagnostic -> String @ d shared
graphical_one : &Diagnostic -> String @ d shared
new : &Diagnostic -> Reportable @ d shared
fmt : Reportable -> &mut Formatter<'_> -> Result @ self shared, f shared
fmt : Reportable -> &mut Formatter<'_> -> Result @ self shared, f shared
code : Reportable -> Option<Box<dyn Display + '_>> ! none @ self shared
severity : Reportable -> Option<miette::Severity> ! none @ self shared
help : Reportable -> Option<Box<dyn Display + '_>> ! none @ self shared
source_code : Reportable -> Option<&dyn miette::SourceCode> ! none @ self shared
labels : Reportable -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> ! none @ self shared
Reportable.message field
Reportable.code field
Reportable.help field
Reportable.span field
Reportable.source field
Reportable.severity field
Reportable.new method
Reportable.fmt method
Reportable.code method
Reportable.severity method
Reportable.help method
Reportable.source_code method
Reportable.labels method
