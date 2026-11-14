# voltaic 2 signature sidecar — generated, do not edit
number : Code -> u16 @ self shared
text : Code -> String @ self shared
all : &[Code]
is_lexical : Code -> bool @ self shared
text : DiagnosticCode -> String @ self shared
is_voltaic : DiagnosticCode -> bool @ self shared
from : Code -> DiagnosticCode @ code owned
all : &[Lint]
name : Lint -> &str @ self shared
code : Lint -> Code @ self shared
allow_attribute : Lint -> String @ self shared
allow_comment : Lint -> String @ self shared
from_allow_attribute : &str -> Option<Lint> ! none @ text shared
from_allow_comment : &str -> Option<Lint> ! none @ text shared
Code.number method
Code.text method
Code.all method
Code.is_lexical method
DiagnosticCode.0 field
DiagnosticCode.text method
DiagnosticCode.is_voltaic method
DiagnosticCode.from method
Lint.all method
Lint.name method
Lint.code method
Lint.allow_attribute method
Lint.allow_comment method
Lint.from_allow_attribute method
Lint.from_allow_comment method
