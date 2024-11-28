use std::{collections::HashSet, fs, io, iter::zip};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_day(&input));
    println!("{}", second_day(&input));

    Ok(())
}

#[inline]
fn first_day(input: &str) -> i32 {
    let mut result = 0;
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    for line in input.lines() {
        let list = line.chars().collect::<Vec<char>>();

        let double_char_vec = list
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                list.get(*i)
                    .is_some_and(|x| list.get(i + 1).is_some_and(|y| x.eq(y)))
            })
            .collect::<Vec<_>>();

        let vowels_count = list
            .iter()
            .filter_map(|x| vowels.get(x))
            .collect::<Vec<_>>();

        if vowels_count.len() >= 3
            && double_char_vec.len() != 0
            && (line.contains("ab")
                || line.contains("cd")
                || line.contains("pq")
                || line.contains("xy"))
                == false
        {
            result += 1;
        }
    }

    result
}

#[inline]
fn second_day(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let list = line.chars().collect::<Vec<char>>();

        let duplicates_chars_vec = list
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                list.get(*i).is_some_and(|x| {
                    list.get(i + 1).is_some_and(|y| {
                        list.get(i + 2).is_some_and(|z| x == z && y != x && y != z)
                    })
                })
            })
            .collect::<Vec<_>>();

        let chars_pairs_vec = zip(
            list.iter().enumerate().filter_map(|(idx, _)| list.get(idx)),
            list.iter()
                .enumerate()
                .filter_map(|(idx, _)| list.get(idx + 1)),
        )
        .collect::<Vec<_>>();

        let chars_pairs_set = chars_pairs_vec.iter().collect::<HashSet<_>>();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_first_day_first_case() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(first_day(&input), 1);
    }

    #[test]
    fn test_first_day_second_case() {
        let input = "aaa";
        assert_eq!(first_day(&input), 1);
    }

    #[test]
    fn test_first_day_third_case() {
        let input = "haegwjzuvuyypxyu";
        assert_eq!(first_day(&input), 0);
    }

    #[test]
    fn test_first_day_fourth_case() {
        let input = "haegwjzuvuyypxyu";
        assert_eq!(first_day(&input), 0)
    }

    #[test]
    fn test_first_day_fifth_case() {
        let input = "dvszwmarrgswjxmb";
        assert_eq!(first_day(&input), 0);
    }
}
