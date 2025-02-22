type page = int

type rule = 
    { before: page
    ; after: page
    }

module Update = struct
    type t = page list

    let of_list xs : t = xs 

    let is_ordered t rules = 
        let eval_rule rule = 
            let open CCOption.Infix in
            let* p1 = List.find_index ((=) rule.before) t in
            let* p2 = List.find_index ((=) rule.after) t in
            Some (p1 < p2)
        in
        rules |> List.filter_map(eval_rule) |> CCList.for_all (fun x -> x)

    let middle t = CCList.get_at_idx_exn ((List.length t) / 2) t
end

type problem = 
    { rules: rule list
    ; updates: Update.t list
    }

module Parsers = struct
    open CCParse
    open CCParse.Infix

    let rule =
        return (fun before after -> { before; after }) <*> U.int <*> char '|' *> U.int

    let update = sep ~by:(char ',') U.int |> map Update.of_list
    
    let problem =
        let* rules = sep ~by:(endline) rule in
        let* _ = endline in
        let* updates = 
            sep ~by:(endline) update
            |> map (CCList.filter (fun u -> List.length u > 0))
        in
        return { rules ; updates }
end

let part1 () = 
    let problem = Aoclib.parse_input_exn Parsers.problem in
    problem.updates
    |> CCList.filter_map (fun u -> 
        if Update.is_ordered u problem.rules then Some(Update.middle u) else None
    )
    |> List.fold_left ( + ) 0
    |> Printf.printf "%d\n"
