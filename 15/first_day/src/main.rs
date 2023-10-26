use std::{error::Error, fs::File, io::Read};

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("./src/assets/raw.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn calculate() -> Result<(i32, i32), String> {
    let input = read_file().unwrap();
    let mut solutions: (i32, i32) = (0, 0);
    let mut counter = 0;

    for i in input.chars() {
        match i {
            '(' => {
                solutions.0 += 1;
            }
            ')' => {
                solutions.0 -= 1;
            }
            _ => {}
        }
    }

    for i in input.chars() {
        solutions.1 += 1;
        match i {
            '(' => {
                counter += 1;
            }
            ')' => {
                counter -= 1;
            }
            _ => {}
        }
        if counter < 0 {
            break;
        }
    }

    println!("{}, {}", solutions.0, solutions.1);

    Ok(solutions)
}

fn main() {
    calculate().unwrap();
}

#[cfg(test)]
mod test {
    use crate::calculate;

    #[test]
    fn test_first_day() {
        let solutions: (i32, i32) = (74, 1795);
        assert_eq!(solutions, calculate().unwrap());
    }
}
