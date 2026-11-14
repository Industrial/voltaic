# voltaic 2 signature sidecar — generated, do not edit
alias_target : &mut Parser -> &Ident -> Result<String, ParseError> ! fail ParseError @ p shared, spelling shared
const_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<ConstDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned, visibility shared
declares_an_operator : &Parser -> bool @ p shared
extern_block : &mut Parser -> Vec<Trivia> -> usize -> Result<ExternBlock, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
import_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<ImportDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned, visibility shared
import_path : &mut Parser -> Result<String, ParseError> ! fail ParseError @ p owned
local_test : &mut Parser -> Result<(), ParseError> ! fail ParseError @ p shared
module_decl : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<ModuleDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned, visibility shared
operator_decl : &mut Parser -> Vec<Trivia> -> usize -> Result<OperatorDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
pattern_synonym : &mut Parser -> Vec<Trivia> -> usize -> Result<PatternSynonym, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
retired_spelling : &Ident -> Option<ParseError> ! none @ name shared
rewritten : &str -> &str -> &str -> String @ text shared, var shared, subject shared
starts_a_declaration : &Parser -> bool @ p shared
test_decl : &mut Parser -> Vec<Trivia> -> usize -> Result<TestDecl, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned
type_alias : &mut Parser -> Vec<Trivia> -> usize -> Visibility -> Result<TypeAlias, ParseError> ! fail ParseError + panic @ p shared, leading shared, start owned, visibility shared
visibility : &mut Parser -> Result<Visibility, ParseError> ! fail ParseError + panic @ p shared
walk_item_for_tails : &Item -> &mut Vec<ParseError> -> () @ item shared, problems shared
