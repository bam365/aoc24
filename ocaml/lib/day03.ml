module Instruction = struct
    type t = int * int

    let eval (x, y) = x * y

    module Parsers = struct
        open CCParse

        let single = 
            pure (fun x y -> (x, y))
                <*> (string "mul(") *> U.int <* string "," <*> U.int <* string ")"

        let interspersed = many (take_until_success single) |> map (List.map snd)
    end
end

let part1 () = 
    Aoclib.parse_input_exn Instruction.Parsers.interspersed
    |> List.map Instruction.eval
    |> List.fold_left ( + ) 0
    |> Printf.printf "%d\n"