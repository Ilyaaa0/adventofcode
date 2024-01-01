use std::cmp::min;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", first_part(&input));
    println!("Second part: {}", second_part(&input));

    Ok(())
}

fn first_part(input: &String) -> i32 {
    let mut solution = 0;

    input.lines().for_each(|x| {
        let numbers = x
            .split("x")
            .map(|x| x.trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        solution += 2
            * (numbers[0] * numbers[1] + numbers[0] * numbers[2] + numbers[1] * numbers[2])
            + min(
                numbers[0] * numbers[1],
                min(numbers[1] * numbers[2], numbers[0] * numbers[2]),
            );
    });

    solution
}

fn second_part(input: &String) -> i32 {
    let mut solution = 0;

    input.lines().for_each(|x| {
        let mut numbers = x
            .split("x")
            .map(|x| x.trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        numbers.sort();

        solution += 2 * (numbers[0] + numbers[1]) + numbers[0] * numbers[1] * numbers[2]
    });

    solution
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_first_part() {
        let data = r#"2x3x4"#.to_string();
        assert_eq!(first_part(&data), 58);
    }

    #[test]
    fn test_second_part() {
        let data = r#"2x3x4"#.to_string();
        assert_eq!(second_part(&data), 34);
    }
}
