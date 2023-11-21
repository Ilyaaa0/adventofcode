use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/assets/raw.txt")?;
    let mut reader = BufReader::new(file);

    let mut raw_content = String::new();

    reader.read_to_string(&mut raw_content)?;

    let mut solutions: (i32, i32) = (0, 0);

    solutions.0 = calculate_duplicate_chars(raw_content).unwrap();

    println!("First solution: {}", solutions.0);

    Ok(())
}

fn calculate_duplicate_chars(raw_content: String) -> Result<i32, String> {
    let mut result = 0;
    let mut duplicate_chars_vec = Vec::<char>::new();

    for i in raw_content.lines() {
        let (first_part, second_part) = i.split_at(i.len() / 2);

    //     for ch in first_part.chars() {
    //         if second_part.contains(ch) {
    //             duplicate_chars_vec.push(ch);
    //         }
    //     }
    // }

    let lowercase_map = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ]);

    let uppercase_map = HashMap::from([
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    for i in duplicate_chars_vec {
        let mut x = lowercase_map.get(&i);

        if x.is_none() {
            x = uppercase_map.get(&i);
        }

        result += x.unwrap();
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn check_calculate_duplicate_char() {
    //     let test_data = "IlyaIlya".to_owned();
    //     let expected = 73;

    //     assert_eq!(calculate_duplicate_chars(test_data), Ok(expected));
    // }

    #[test]
    fn check_data_from_file() {
        let test_data = std::fs::read_to_string("./src/assets/test.txt").unwrap();
        let expected = 157;

        assert_eq!(calculate_duplicate_chars(test_data), Ok(expected));
    }
}
