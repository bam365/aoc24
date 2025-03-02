open Aoclib

module Op = struct
    type t =
        | Add
        | Mul

    let eval t x y = 
        match t with 
        | Add -> Int64.add x y
        | Mul -> Int64.mul x y

    let to_str = function
        | Add -> "+"
        | Mul -> "*"
end

let eval_string xs all_ops = 
    let rec aux acc ops ys = 
        match (ops, ys) with 
        | (op::ops', y::ys') -> aux (Op.eval op acc y) ops' ys'
        | _ -> acc
    in
    match xs with 
    | [] -> Int64.zero 
    | head::tail -> aux head all_ops tail
    

let eval_all_strings = function
    | [] -> [Int64.zero]
    | xs ->
        CCList.init (CCList.length xs - 1) (fun _ -> [Op.Add; Op.Mul])
        |> permute_choices
        |> CCList.map (eval_string xs)

module Equation = struct
    type t = 
        { test: Int64.t
        ; values: Int64.t list
        }

    let is_possible t =
        eval_all_strings t.values
        |> CCList.exists (Int64.equal t.test)
end

module Parsers = struct
    open CCParse
    open CCParse.Infix

    let number = ParserUtils.uint64

    let equation =
        let* test = number in
        let* _ = string ": " in
        let* values = sep ~by:(chars1_if is_space) number in
        pure Equation.({ test; values })
end

let part1 () = 
    parse_lines_exn Parsers.equation
    |> CCList.filter Equation.is_possible 
    |> CCList.map (fun v -> Equation.(v.test))
    |> CCList.fold_left (Int64.add) Int64.zero 
    |> fun answer -> Printf.printf "%s\n" (Int64.to_string answer)
