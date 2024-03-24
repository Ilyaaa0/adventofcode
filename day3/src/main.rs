use std::{collections::HashSet, fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_part(&input));
    // println!("{}", second_part(&input));

    Ok(())
}

fn first_part(input: &String) -> usize {
    let mut position_set = HashSet::new();

    let mut position = (0, 0);

    position_set.insert(position);

    for i in input.chars() {
        if let '^' = i {
            position.1 += 1;
        }

        if let 'v' = i {
            position.1 -= 1;
        }

        if let '<' = i {
            position.0 -= 1;
        }

        if let '>' = i {
            position.0 += 1;
        }

        position_set.insert(position);
    }

    position_set.len()
}

// fn second_part(input: &String) -> usize {
//     let mut posion_set = HashSet::new();

//     let mut position = (0, 0);

//     for i in input.chars() {}

//     position_set.len()
// }
