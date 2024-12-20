module Report = struct
    type t = int list

    let is_safe t = 
        let open CCList in
        let diffs = 
            combine_shortest t (drop 1 t) 
            |> map (fun pair -> fst pair - snd pair) 
        in
        let all_increasing = not (exists (fun v -> v <= 0) diffs) in
        let all_decreasing = not (exists (fun v -> v >= 0) diffs) in
        let safe_distances = not (exists (fun v -> (abs v) > 3) diffs) in
        (all_increasing || all_decreasing) && safe_distances

    let (parser : t CCParse.t) = CCParse.(sep ~by:(white) U.int)
end

let part1 () = 
    Aoclib.parse_lines_exn Report.parser
    |> CCList.filter Report.is_safe
    |> CCList.length
    |> Printf.printf "%d\n"

