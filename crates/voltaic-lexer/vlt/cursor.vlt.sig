# voltaic 2 signature sidecar — generated, do not edit
new : &str -> Cursor @ src shared
pos : Cursor -> usize @ self shared
is_eof : Cursor -> bool @ self shared
peek : Cursor -> Option<char> ! none @ self shared
peek_at : usize -> Cursor -> Option<char> ! none @ n owned, self shared
starts_with : &str -> Cursor -> bool @ s owned, self shared
rest : Cursor -> &str @ self shared
bump : Cursor -> Option<char> ! none + mut self @ self unique
eat : &str -> Cursor -> bool @ s shared, self unique
eat_while : impl Fn(char) -> bool -> Cursor -> Span @ pred shared, self unique
eat_spaces : Cursor -> usize @ self unique
slice : Span -> Cursor -> &str @ span shared, self shared
reset_to : usize -> Cursor -> () ! mut self @ at shared, self unique
mark : Cursor -> usize @ self shared
Cursor.src field
Cursor.pos field
Cursor.new method
Cursor.pos method
Cursor.is_eof method
Cursor.peek method
Cursor.peek_at method
Cursor.starts_with method
Cursor.rest method
Cursor.bump method
Cursor.eat method
Cursor.eat_while method
Cursor.eat_spaces method
Cursor.slice method
Cursor.reset_to method
Cursor.mark method
