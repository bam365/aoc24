let parse_lines_exn line_parser =
    CCIO.read_lines_l In_channel.stdin
    |> CCList.map (CCParse.parse_string_exn line_parser)

let parse_input_exn parser =
    CCIO.read_all In_channel.stdin
    |> CCParse.parse_string_exn parser

