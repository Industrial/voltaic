# voltaic 2 signature sidecar — generated, do not edit
serve : &Connection -> Result<(), String> ! fail String @ connection shared
absorb : &Notification -> &mut Documents -> Option<Uri> ! none @ notification shared, documents shared
answer : &Request -> &Documents -> Response @ request shared, documents shared
cast : &Request -> &str -> Result<P, ExtractError<Request>> ! fail ExtractError<Request> @ request shared, method shared
declared : &Item -> Option<(&'static str, String)> ! none @ item shared
definition : &str -> &Uri -> Position -> Option<GotoDefinitionResponse> ! none @ source owned, uri owned, at shared
word_at : &str -> usize -> Option<String> ! none + mut index @ source shared, offset shared
diagnostics : &Uri -> &str -> Vec<Diagnostic> @ uri shared, source shared
convert : &voltaic_diag::Diagnostic -> &str -> Diagnostic @ diagnostic shared, source owned
path_of : &Uri -> std::path::PathBuf ! none @ uri shared
new : Documents
set : Uri -> String -> Documents -> () ! none @ uri shared, text shared, self shared
close : &Uri -> Documents -> () @ uri shared, self shared
text : &Uri -> Documents -> Option<&str> ! none @ uri shared, self shared
folding : &str -> Vec<FoldingRange> @ source shared
formatting : &str -> Option<Vec<TextEdit>> ! none @ source shared
hover : &str -> Position -> Option<Hover> ! none @ source owned, at shared
innermost : &[Item] -> usize -> Option<(&'static str, String, (usize, usize))> ! none @ items shared, offset shared
offset_of : &str -> Position -> usize @ source owned, at shared
position_of : &str -> usize -> Position @ source shared, offset shared
range_of : &str -> (usize, usize) -> Range @ source owned, span shared
publish : &Connection -> &Uri -> &Documents -> Result<(), String> ! fail String @ connection shared, uri shared, documents shared
reply : RequestId -> Result<P, ExtractError<Request>> -> impl FnOnce(P) -> T -> Response @ id shared, params shared, handle shared
semantic_tokens : &str -> SemanticTokensResult @ source shared
token_index : &TokenKind -> Option<u32> ! none @ kind shared
symbols : &str -> DocumentSymbolResponse @ source shared
symbol_of : &Item -> &str -> DocumentSymbol @ item shared, source shared
Documents.open field
Documents.new method
Documents.set method
Documents.close method
Documents.text method
