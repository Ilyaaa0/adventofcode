use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", first_part(&input));
    println!("Second part: {}", second_part(&input));

    Ok(())
}

fn first_part(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut position_set = HashSet::from([(x, y)]);

    for i in input.as_bytes() {
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

fn second_part(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let (mut rx, mut ry) = (0, 0);

    let mut position_set_santa = HashSet::from([(x, y)]);
    let mut position_set_robosanta = HashSet::from([(rx, ry)]);

    for (idx, ch) in input.chars().enumerate() {
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
    fn case1() {
        assert_eq!(second_part("^v"), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(second_part("^>v<"), 3);
    }

    #[test]
    fn case3() {
        assert_eq!(second_part("^v^v^v^v^v"), 11);
    }
}
