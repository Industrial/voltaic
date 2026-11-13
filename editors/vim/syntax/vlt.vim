" Voltaic syntax highlighting for Vim. Plan 49 leaf 2.
"
" Regex only. This is the fallback for an editor with neither tree-sitter nor a
" built-in LSP client, so it is the one client here that highlights by pattern
" rather than by parse — and therefore the one that can be wrong about a
" keyword inside a string or a comment, which `syn region` mitigates and does
" not eliminate.
"
" The keyword list is checked against `tree-sitter-voltaic/grammar.js` by
" `editors.rs`'s `every_grammar_keyword_is_coloured`, so this file and the
" tree-sitter and TextMate highlighters cannot drift apart. Three highlighters
" that disagree are worse than one.

if exists('b:current_syntax')
  finish
endif

" Comments are **regions**, not matches, and that is forced rather than
" stylistic. In Vim a `syn keyword` outranks a `syn match` wherever both apply,
" so with `syn match` comments the word `data` inside `-- mentioning data`
" stays a keyword and the leading `--` is claimed by the operator pattern. A
" region's `contains` is what excludes an item from its interior, and only
" `@Spell` is listed here.
"
" Found by `editors.rs`, which asserts the group *inside* a comment and not
" only at its start — the assertion that would have passed either way.
"
" `-- |` and `-- !` are the doc forms (Haskell's); `--` alone is ordinary.
" Both start at the same column, and among items starting together Vim prefers
" the one defined last, so the doc region comes second.
syn region  vltComment      start="--" end="$" oneline keepend contains=@Spell
syn region  vltDocComment   start="--\s*[|!]" end="$" oneline keepend contains=@Spell

" Strings. The triple-quoted form is Voltaic's verbatim block (plan 45's inline
" Rust and plan 23's raw strings), and must be matched before the single form
" or the opening `"""` reads as an empty string followed by a quote.
syn region  vltVerbatim     start=+"""+ end=+"""+ keepend
syn region  vltString       start=+"+ skip=+\\\\\|\\"+ end=+"+ oneline
syn match   vltChar         "'\([^'\\]\|\\.\)'"
syn match   vltLifetime     "'[a-z_][a-zA-Z0-9_]*\>'\@!"

syn match   vltNumber       "\<\d[0-9_]*\(\.\d[0-9_]*\)\?\([eE][-+]\?\d\+\)\?\(_\?[iuf]\(8\|16\|32\|64\|128\|size\)\)\?\>"
syn match   vltNumber       "\<0[xX][0-9a-fA-F_]\+\>"
syn match   vltNumber       "\<0[bB][01_]\+\>"
syn match   vltNumber       "\<0[oO][0-7_]\+\>"
syn keyword vltBoolean      true false
syn keyword vltSelf         self Self

" Keywords. One list, derived from the grammar — see the header.
"
" `then` left at plan 55 leaf 7, which retired the construct it named and
" freed the word for `Ordering::then`. `do` stayed: the leaf retired its
" construct too and kept the word **reserved**, so it is still worth colouring
" and `editors.rs` excuses it from the grammar cross-check — the tree-sitter
" grammar declares it through `reserved` rather than as a literal in a rule.
syn keyword vltKeyword      import module data newtype type class instance
syn keyword vltKeyword      errors test const static deriving from as where given pattern template infix infixl infixr over
syn keyword vltKeyword      otherwise if else while until for in loop
syn keyword vltKeyword      break continue return guard is with case of
syn keyword vltKeyword      do async try catch mut move or
syn keyword vltKeyword      rust unsafe extern union
syn keyword vltKeyword      export

" A type is any `UpperCamel` name; a syntax file carries no type information, so it
" certainly cannot tell a type from a constructor, and colouring both the same
" is the honest answer rather than a heuristic that is sometimes wrong.
syn match   vltType         "\<\u[a-zA-Z0-9_]*\>"
syn match   vltAttribute    "@[a-zA-Z_][a-zA-Z0-9_]*"
" Plan 52's two tokens. `:` introduces a type in every position that has one —
" a signature, a typed binding, an ascription, a field, a lambda's binder — and
" `::` separates path segments and nothing else. The `:\(:\)\@!` lookahead is
" the same idiom as the `-` match below, and for the same reason: it makes the
" result independent of which item Vim happens to prefer.
syn match   vltTypeColon    ":\(:\)\@!"
syn match   vltPathSep      "::"
" `-` is matched with a negative lookahead for a second `-`, and that is the
" second half of the comment fix above. Voltaic has no `--` operator, and among
" items starting at the same column Vim prefers the one defined **last** — so
" a `[-+*…]` class here outranks the comment region at column 1 and the whole
" line stops being a comment. Excluding `--` is both correct on its own terms
" and what makes the ordering stop mattering.
" Plan 66's `>>>` and `<<<` are listed **before** the shifts: Vim takes the
" first alternative that matches at a position, so `>>` first would colour two
" of the three characters and leave the last one to the `[<>]` class.
syn match   vltOperator     "->\|=>\|\\\|??\||>\|>>>\|<<<\|<<\|>>\|[+*/%<>=!&|^~]\|-\(-\)\@!"

hi def link vltComment      Comment
hi def link vltDocComment   SpecialComment
hi def link vltString       String
hi def link vltVerbatim     String
hi def link vltChar         Character
hi def link vltLifetime     Label
hi def link vltNumber       Number
hi def link vltBoolean      Boolean
hi def link vltSelf         Identifier
hi def link vltKeyword      Keyword
hi def link vltType         Type
hi def link vltAttribute    PreProc
hi def link vltTypeColon    Delimiter
hi def link vltPathSep      Delimiter
hi def link vltOperator     Operator

let b:current_syntax = 'vlt'
