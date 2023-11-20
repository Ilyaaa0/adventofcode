use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/assets/raw.txt")?;
    let mut reader = BufReader::new(file);

    let mut solutions: (i32, i32) = (0, 0);
    let mut raw_content = String::new();

    reader.read_to_string(&mut raw_content)?;

    let mut vec_sum: Vec<i32> = Vec::new();

    let mut sum = 0;

    for i in raw_content.lines() {
        if i.is_empty() {
            vec_sum.push(sum);
            sum = 0;
            continue;
        }

        let x = i.trim().parse::<i32>().unwrap();
        sum += x;
    }

    vec_sum.sort();

    solutions.0 = *vec_sum.last().unwrap();

    vec_sum.reverse();

    solutions.1 = vec_sum[0] + vec_sum[1] + vec_sum[2];

    println!("First solution: {}", solutions.0);

    println!("Second solution: {}", solutions.1);

    Ok(())
}
