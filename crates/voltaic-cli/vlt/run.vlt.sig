# voltaic 2 signature sidecar — generated, do not edit
run : &[String] -> Result<(), String> ! fail String @ args shared
matches : &str -> Command -> bool @ arg shared, self shared
not_yet : Command -> Option<String> ! none @ self shared
lookup : &str -> Option<&Command> ! none @ arg shared
usage : String
Command.name field
Command.summary field
Command.state field
Command.matches method
Command.not_yet method
State.plan field
State.id field
