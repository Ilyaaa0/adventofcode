fn main() -> Result<(), std::io::Error> {
    println!(
        "day1: {}",
        first_part(&std::fs::read_to_string("./src/input.txt")?)
    );
    println!(
        "day2: {}",
        second_part(&std::fs::read_to_string("./src/input.txt")?)
    );

    Ok(())
}

fn first_part(input: &str) -> i32 {
    input.chars().fold(0, |x, y| match y {
        '(' => x + 1,
        ')' => x - 1,
        _ => 0,
    })
}

fn second_part(input: &str) -> i32 {
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
    fn day1() {
        assert_eq!(first_part("()()"), 0);
    }

    #[test]
    fn day2() {
        assert_eq!(second_part("((()))()()"), 10);
    }
}
