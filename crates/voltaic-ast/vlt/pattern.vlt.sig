# voltaic 2 signature sidecar — generated, do not edit
span : Pattern -> Span @ self shared
is_irrefutable : Pattern -> bool @ self shared
rust_text : Pattern -> String @ self shared
binders : Pattern -> Vec<String> @ self shared
collect_binders : Pattern -> &mut Vec<String> -> () @ self shared, into shared
rust_text : RecordPatternField -> String @ self shared
Pattern.0 field
Pattern.text field
Pattern.span field
Pattern.lo field
Pattern.hi field
Pattern.inclusive field
Pattern.path field
Pattern.args field
Pattern.items field
Pattern.fields field
Pattern.rest field
Pattern.name field
Pattern.inner field
Pattern.span method
Pattern.is_irrefutable method
Pattern.rust_text method
Pattern.binders method
Pattern.collect_binders method
RecordPatternField.name field
RecordPatternField.pattern field
RecordPatternField.rust_text method
