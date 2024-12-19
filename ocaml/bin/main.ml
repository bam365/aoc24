let () = 
    match Sys.argv.(1) with
    | "1" -> Day01.part1 ()
    | _ -> print_endline "unknown day/part, idiot"
