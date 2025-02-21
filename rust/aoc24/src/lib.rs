pub mod aoclib {
    use std::io::{self, Read};
    use nom::{Err, error::Error, IResult};

    pub fn read_all_stdin() -> String {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read input");
        input
    }

    pub fn parse_all_stdin<'a, O, F>(parser: F) -> O 
    where
        F: Fn(&str) -> IResult<&str, O>
    {
        let input = read_all_stdin();
        parser(&input).map(|v| { v.1 }).unwrap()
    }

    pub fn parse_all_stdin_err<O, F>(parser: F) -> Result<O, Err<Error<String>>>
    where
        F: Fn(&str) -> IResult<&str, O>
    {
        let input = read_all_stdin();
        parser(&input)
            .map(|v| { v.1 })
            .map_err(|e| e.map_input(String::from))
    }

}
