use std::{fs, io};

fn main() -> io::Result<()> {
    println!("day1: {}", day1(&fs::read_to_string("./src/input.txt")?));

    Ok(())
}

fn day1(input: &str) -> i32 {
    i32::MAX
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_day1() {
        assert!(true);
    }
}
