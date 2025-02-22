let sorted_pair_diff xs ys = 
    let open CCList in
    let (xs_sorted, ys_sorted) = (sort CCInt.compare xs, sort CCInt.compare ys) in
    let distance pair = abs (fst pair - snd pair) in
    combine_shortest xs_sorted ys_sorted |> map distance |> fold_left ( + ) 0

let pair_parser =
    let open CCParse in
    let* x = U.int <* skip_white in
    let* y = U.int in
    pure (x, y)

let part1 () = 
    let pairs = Aoclib.parse_lines_exn pair_parser in
    sorted_pair_diff (List.map fst pairs) (List.map snd pairs)
    |> Printf.printf "%d\n"
    
