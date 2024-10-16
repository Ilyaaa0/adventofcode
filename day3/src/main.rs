use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    println!(
        "day1: {}",
        second_day(&fs::read_to_string("./src/input.txt")?)
    );
    println!(
        "day2: {}",
        first_day(&fs::read_to_string("./src/input.txt")?)
    );

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
    let (mut rx, mut ry) = (0, 0);

    let mut position_set_santa = HashSet::from([(x, y)]);
    let mut position_set_robosanta = HashSet::from([(rx, ry)]);

    for (idx, ch) in chars {
        match idx % 2 {
            0 => {
                match ch {
                    '^' => x += 1,
                    'v' => x -= 1,
                    '>' => y += 1,
                    '<' => y -= 1,
                    _ => (),
                }

                position_set_santa.insert((x, y));
            }
            _ => {
                match ch {
                    '^' => rx += 1,
                    'v' => rx -= 1,
                    '>' => ry += 1,
                    '<' => ry -= 1,
                    _ => (),
                }

                position_set_robosanta.insert((rx, ry));
            }
        }
    }

    position_set_santa
        .union(&position_set_robosanta)
        .collect::<HashSet<&(i32, i32)>>()
        .len()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_day2_case1() {
        assert_eq!(second_day("^v"), 3);
    }

    #[test]
    fn test_day3_case2() {
        assert_eq!(second_day("^>v<"), 3);
    }

    #[test]
    fn test_day2_case3() {
        assert_eq!(second_day("^v^v^v^v^v"), 11);
    }
}
