let sorted_pair_diff xs ys = 
    let open CCList in
    let xs_sorted = sort (CCInt.compare) xs in
    let ys_sorted = sort (CCInt.compare) ys in
    let distance pair = abs (fst pair - snd pair) in
    combine xs_sorted ys_sorted |> map distance |> fold_left ( + ) 0

let pair_parser =
    let open CCParse in
    let* n1 = U.int in
    let* _ = skip_white in
    let* n2 = CCParse.U.int in
    pure (n1, n2)

let part1 () = 
    let pairs = Aoclib.parse_lines pair_parser in
    sorted_pair_diff (List.map fst pairs) (List.map snd pairs)
    |> Printf.printf "%d\n"
    
