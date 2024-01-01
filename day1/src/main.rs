fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", first_part(&input));
    println!("Second part: {}", second_part(&input));

    Ok(())
}

fn first_part(input: &String) -> i32 {
    let mut solution: i32 = 0;

    input.chars().for_each(|x| {
        (x == '(').then(|| solution += 1);
        (x == ')').then(|| solution -= 1);
    });

    solution
}

fn second_part(input: &String) -> i32 {
    let mut solution: i32 = 0;
    let mut counter = 0;

    for i in input.chars() {
        solution += 1;

        (i == '(').then(|| counter += 1);
        (i == ')').then(|| counter -= 1);

        if counter < 0 {
            break;
        }
    }

    solution
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_first_part() {
        let data = r#"()()"#.to_string();
        assert_eq!(first_part(&data), 0);
    }

    #[test]
    fn test_second_part() {
        let data = r#"((()))()()"#.to_string();
        assert_eq!(second_part(&data), 10);
    }
}
