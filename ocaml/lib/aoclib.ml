let parse_lines_exn line_parser =
    CCIO.read_lines_l In_channel.stdin
    |> CCList.map (CCParse.parse_string_exn line_parser)

let parse_input_exn parser =
    CCIO.read_all In_channel.stdin
    |> CCParse.parse_string_exn parser

let rec permute_choices (xs: 'a list list): 'a list list =
    let open CCList.Infix in
    match xs with
    | [] -> []
    | [x] -> CCList.map (fun v -> [v]) x
    | first::rest -> 
        let other_choices = permute_choices rest in
        let* head = first in
        let* tail = other_choices in
        [head :: tail]

module ParserUtils = struct
    open CCParse 

    let uint64: Int64.t t = 
        chars1_if is_num
        |> map Int64.of_string
end

module Monoid = struct
    type 'a t = 
        { mempty: 'a
        ; mappend: 'a -> 'a -> 'a
        }

    let int_sum = { mempty = 0; mappend = ( + ) }

    let float_sum = { mempty = 0.; mappend = ( +. ) }

    let list_append = { mempty = []; mappend = List.append }

    let concat t xs = List.fold_left t.mappend t.mempty xs
end

