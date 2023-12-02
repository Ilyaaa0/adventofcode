use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("./src/assets/raw.txt")?;
    let mut reader = BufReader::new(&file);

    let mut content = String::new();
    let mut solutions: (i32, i32) = (0, 0);

    reader.read_to_string(&mut content)?;

    solutions.0 = first_part(&content);
    solutions.1 = second_part(&content);

    println!("{:?}", solutions);

    Ok(())
}

fn first_part(content: &String) -> i32 {
    let mut result = 0;

    for i in content.lines() {
        let mut num_vec: Vec<i32> = Vec::new();

        for j in i.chars() {
            let num = j.to_string().parse::<i32>();
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
        let mut vec: Vec<i32> = Vec::new();

        for (str, val) in digits_map.iter() {
            i.contains(str).then(|| vec.push(*val));
        }

        (!vec.is_empty()).then(|| result += vec.first().unwrap() + vec.last().unwrap());

        println!("{:?}", vec);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_first_part() -> Result<()> {
        let data = std::fs::read_to_string("./src/assets/test1.txt")?;
        let expected = 142;

        assert_eq!(first_part(&data), expected);

        Ok(())
    }

    #[test]
    fn check_second_part() -> Result<()> {
        let data = std::fs::read_to_string("./src/assets/test2.txt")?;
        let expected = 281;

        assert_eq!(second_part(&data), expected);

        Ok(())
    }
}
