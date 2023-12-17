use std::collections::HashMap;
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_to_string("./src/assets/raw.txt")?;

    println!("{:?}", first_part(&content));
    println!("{:?}", second_part(&content));

    Ok(())
}

fn first_part(content: &String) -> i32 {
    let mut result = 0;

    for i in content.lines() {
        let mut num_vec: Vec<u32> = Vec::new();

        for j in i.chars() {
            let num = j.to_string().parse::<u32>();
            num.is_ok().then(|| num_vec.push(num.unwrap()));
        }

        let concat =
            num_vec.first().unwrap().to_string() + num_vec.last().unwrap().to_string().as_str();

        result += concat.parse::<i32>().unwrap();
    }

    result
}

fn second_part(raw: &String) -> i32 {
    let mut result = 0;

    let digits_map: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for i in raw.lines() {
        let mut chars_vec: Vec<char> = Vec::new();

        for j in i.chars() {
            chars_vec.push(j)
        }

        println!("vec: {chars_vec:?}");
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_first_part() {
        let data = std::fs::read_to_string("./src/assets/test1.txt").unwrap();
        let expected = 142;

        assert_eq!(first_part(&data), expected);
    }

    #[test]
    fn check_second_part() {
        let data = std::fs::read_to_string("./src/assets/test2.txt").unwrap();
        let expected = 281;

        assert_eq!(second_part(&data), expected);
    }
}
