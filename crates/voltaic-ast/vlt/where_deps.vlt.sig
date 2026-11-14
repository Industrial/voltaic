# voltaic 2 signature sidecar — generated, do not edit
where_cycle : &[Stmt] -> Option<Vec<String>> ! none @ entries shared
walk : usize -> &[Vec<usize>] -> &mut [u8] -> &mut Vec<usize> -> Option<Vec<usize>> ! none + mut state @ at shared, graph shared, state unique, path shared
where_edges : &[Stmt] -> Vec<Vec<usize>> @ entries shared
mentions : &Expr -> &[(usize, String)] -> &mut Vec<usize> -> () @ e shared, named shared, into shared
operands : &Expr -> Vec<&Expr> @ e shared
