# voltaic 2 signature sidecar — generated, do not edit
least : Mode
greatest : Mode
join : Mode -> Mode -> Mode @ other owned, self owned
permits : Mode -> Mode -> bool @ needed shared, self shared
label : Mode -> &str @ self shared
emits : Mode -> &str @ self shared
of_receiver : &str -> Option<Mode> ! none @ name shared
of_written : &Ty -> Mode @ written shared
fmt : Mode -> &mut Formatter<'_> -> Result @ self shared, f shared
Mode.least method
Mode.greatest method
Mode.join method
Mode.permits method
Mode.label method
Mode.emits method
Mode.of_receiver method
Mode.of_written method
Mode.fmt method
