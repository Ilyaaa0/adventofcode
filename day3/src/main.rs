use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let res_day1 = first_day(&input);
    let res_day2 = second_day(&input);

    println!("day1 result: {res_day1:?}");
    println!("day2 result: {res_day2:?}");

    Ok(())
}

fn first_day(input: &str) -> usize {
    let bytes = input.as_bytes();
    let (mut x, mut y) = (0, 0);
    let mut position_set = HashSet::from([(x, y)]);

    for i in bytes {
        match i {
            94 => x += 1,  // '^'
            118 => x -= 1, // 'v'
            62 => y += 1,  // '>'
            60 => y -= 1,  // '<'
            _ => (),
        }

        position_set.insert((x, y));
    }

    position_set.len()
}

fn second_day(input: &str) -> usize {
    let chars = input.chars().enumerate();
    let (mut x, mut y) = (0, 0);
    let mut position_set = HashSet::from([(0, 0)]);

    for (idx, ch) in chars {}

    position_set.len()
}

#[cfg(test)]
mod test {
    use crate::second_day;

    #[test]
    fn test_second_day_case1() {
        let data = "^v";
        let result = second_day(&data);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_second_day_case2() {
        let data = "^>v<";
        let result = second_day(&data);

        assert_eq!(result, 3);
    }
}