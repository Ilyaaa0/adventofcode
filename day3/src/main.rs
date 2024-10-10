use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result = first_day(&input);

    println!("day1 result: {result:?}");

    let result = first_day_func(&input);

    println!("{result}");

    Ok(())
}

fn first_day(input: &str) -> usize {
    let bytes = input.as_bytes();
    let (mut x, mut y) = (0, 0);
    let mut position_set = HashSet::from([(x, y)]);

    for i in bytes {
        match i {
            &94 => x += 1,  // '^'
            &118 => x -= 1, // 'v'
            &62 => y += 1,  // '>'
            &60 => y -= 1,  // '<'
            _ => {}
        }

        position_set.insert((x, y));
    }

    position_set.len()
}

fn first_day_func(input: &str) -> usize {
    let mut position_set = HashSet::from([(0, 0)]);

    input
        .as_bytes()
        .into_iter()
        .fold((0, 0), |(mut x, mut y), v| {
            match v {
                &94 => x += 1,
                &118 => x -= 1,
                &62 => y += 1,
                &60 => y -= 1,
                _ => (),
            };
            position_set.insert((x, y));

            (x, y)
        });

    position_set.len()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_second_day() {
        assert!(true);
    }
}
