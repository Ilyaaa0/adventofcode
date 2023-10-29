use std::{cmp::min, error::Error, fs::File, io::Read};

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("./src/assets/raw.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn calculate() -> Result<(i32, i32), String> {
    let input = read_file().unwrap();
    let mut solutions: (i32, i32) = (0, 0);

    for line in input.lines() {
        let mut numbers = line
            .split("x")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        solutions.0 += 2
            * (numbers[0] * numbers[1] + numbers[0] * numbers[2] + numbers[1] * numbers[2])
            + min(
                numbers[0] * numbers[1],
                min(numbers[1] * numbers[2], numbers[0] * numbers[2]),
            );

        let bow = numbers[0] * numbers[1] * numbers[2];

        numbers.sort();

        solutions.1 += 2 * (numbers[0] + numbers[1]) + bow;
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
    fn test_second_day() {
        let solutions: (i32, i32) = (1586300, 3737498);
        assert_eq!(calculate(), Ok(solutions));
    }
}
