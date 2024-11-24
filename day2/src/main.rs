use std::{cmp::min, fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("day1: {}", first_part(&input));
    println!("day2: {}", second_part(&input));

    Ok(())
}

fn first_part(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            x.split("x")
                .map(|x| x.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .fold(0, |x, y| {
            x + 2 * (y[0] * y[1] + y[0] * y[2] + y[1] * y[2])
                + min(y[0] * y[1], min(y[1] * y[2], y[0] * y[2]))
        })
}

fn second_part(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            x.split("x")
                .map(|x| x.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .map(|mut x| {
            x.sort();
            x
        })
        .fold(0, |x, y| x + 2 * (y[0] + y[1]) + y[0] * y[1] * y[2])
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn day1() {
        assert_eq!(first_part("2x3x4"), 58);
    }

    #[test]
    fn day2() {
        assert_eq!(second_part("2x3x4"), 34);
    }
}
