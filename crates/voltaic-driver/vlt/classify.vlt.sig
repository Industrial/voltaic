# voltaic 2 signature sidecar — generated, do not edit
fmt : Class -> &mut Formatter<'_> -> Result @ self shared, f shared
markers : Vec<Marker>
add : &mut Vec<Marker> -> &str -> &str -> Class -> () @ into owned, needle owned, construct owned, kind owned
at_end : &mut Vec<Marker> -> &str -> &str -> Class -> () @ into owned, needle owned, construct owned, kind owned
at_start : &mut Vec<Marker> -> &str -> &str -> Class -> () @ into owned, needle owned, construct owned, kind owned
at : &mut Vec<Marker> -> &str -> &str -> Class -> Position -> () @ into shared, needle owned, construct owned, kind shared, position shared
of_class : Class -> Verdict -> Vec<&Finding> @ wanted shared, self shared
classify : &str -> Verdict @ source shared
hits : &str -> &Marker -> bool @ text owned, marker shared
opens_a_statement : &str -> &str -> bool @ text shared, raw shared
mentions : &str -> &str -> bool @ text owned, raw owned
is_word : &str -> bool @ needle owned
is_word_byte : &str -> bool @ held owned
is_comment : &str -> bool @ text owned
new : Tally
record : Class -> Tally -> () ! mut ? + mut self @ kind shared, self unique
count : Class -> Tally -> usize @ kind shared, self shared
blocking : Tally -> usize @ self owned
Class.fmt method
Marker.needle field
Marker.construct field
Marker.kind field
Marker.position field
Finding.line field
Finding.construct field
Finding.kind field
Verdict.kind field
Verdict.findings field
Verdict.of_class method
Tally.files field
Tally.by_class field
Tally.new method
Tally.record method
Tally.count method
Tally.blocking method
