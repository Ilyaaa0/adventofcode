use md5;
use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_day(&input));
    println!("{}", second_day(&input));

    Ok(())
}

fn first_day(input: &str) -> i32 {
    let mut result_hash = String::new();
    let mut result = 0;
    let input = input.trim();

    while !result_hash.starts_with("00000") {
        result += 1;

        let hash = format!("{input}{result}");
        result_hash = format!("{:?}", md5::compute(hash));
    }

    result
}

fn second_day(input: &str) -> i32 {
    let mut result_hash = String::new();
    let mut result = 0;
    let input = input.trim();

    while !result_hash.starts_with("000000") {
        result += 1;

        let hash = format!("{input}{result}");
        result_hash = format!("{:?}", md5::compute(hash));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_first_day_case1() {
        let input = "abcdef";
        assert_eq!(first_day(input), 609043);
    }

    #[test]
    fn test_second_day_case2() {
        let input = "pqrstuv";
        assert_eq!(first_day(input), 1048970);
    }
}
