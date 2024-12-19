let parse_lines line_parser =
    CCIO.read_lines_l In_channel.stdin
    |> CCList.map (CCParse.parse_string_exn line_parser)
