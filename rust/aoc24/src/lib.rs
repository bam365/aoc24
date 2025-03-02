pub mod aoclib {
    use nom::{error::Error, Err, IResult};
    use std::io::{self, Read};

    pub fn read_all_stdin() -> String {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read input");
        input
    }

    pub fn parse_all_stdin<'a, O, F>(parser: F) -> O
    where
        F: Fn(&str) -> IResult<&str, O>,
    {
        let input = read_all_stdin();
        parser(&input).map(|v| v.1).unwrap()
    }

    pub fn parse_all_stdin_err<O, F>(parser: F) -> Result<O, Err<Error<String>>>
    where
        F: Fn(&str) -> IResult<&str, O>,
    {
        let input = read_all_stdin();
        parser(&input)
            .map(|v| v.1)
            .map_err(|e| e.map_input(String::from))
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct Coord {
        pub x: usize,
        pub y: usize,
    }

    impl Coord {
        pub fn change(&self, delta: (i32, i32)) -> Option<Coord> {
            let old_x: i32 = self.x.try_into().unwrap();
            let old_y: i32 = self.y.try_into().unwrap();
            let new_x: usize = (old_x + delta.0).try_into().ok()?;
            let new_y: usize = (old_y + delta.1).try_into().ok()?;
            Some(Coord { x: new_x, y: new_y })
        }
    }

    pub fn concat_vecs<T: Copy>(xs: &Vec<T>, ys: &Vec<T>) -> Vec<T> {
        let mut r: Vec<T> = vec![];
        for x in xs {
            r.push(*x);
        }
        for y in ys {
            r.push(*y);
        }
        r
    }

    pub fn permute_choices<T: Copy>(choices: Vec<Vec<T>>) -> Vec<Vec<T>> {
        if choices.len() == 0 {
            vec![]
        } else if choices.len() == 1 {
            choices[0].iter().map(|v| vec![*v]).collect()
        } else {
            let first = choices[0].clone();
            let rest: Vec<Vec<T>> = choices.into_iter().skip(1).collect();
            let mut r: Vec<Vec<T>> = vec![];
            let rest_choices = permute_choices(rest);
            for head in first.iter() {
                for tail in rest_choices.iter() {
                    r.push(concat_vecs(&vec![*head], &tail));
                }
            }
            r
        }
    }
}
