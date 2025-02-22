let () = 
    match Sys.argv.(1) with
    | "1" -> Aoc.Day01.part1 ()
    | "2" -> Aoc.Day02.part1 ()
    | "3" -> Aoc.Day03.part1 ()
    | "5" -> Aoc.Day05.part1 ()
    | _ -> print_endline "unknown day/part, idiot"
