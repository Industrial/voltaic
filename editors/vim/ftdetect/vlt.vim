" Plan 49 leaf 2.
"
" `.vlt` is Voltaic. `.vrs` is **ordinary Rust** — that is the compiler's whole
" premise, so the generated file gets Vim's own Rust support rather than a
" second Voltaic mode that would highlight it wrongly.
autocmd BufRead,BufNewFile *.vlt setfiletype vlt
autocmd BufRead,BufNewFile *.vrs setfiletype rust
