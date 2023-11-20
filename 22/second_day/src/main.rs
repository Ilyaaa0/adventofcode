use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/assets/raw.txt")?;
    let mut reader = BufReader::new(file);

    let mut solutions: (i32, i32) = (0, 0);
    let mut raw_content = String::new();

    reader.read_to_string(&mut raw_content)?;

    for i in raw_content.lines() {
        let values = i.split_whitespace().collect::<Vec<&str>>();

        let (opponent, me) = (values[0], values[1]);

        match me {
            // rock
            "X" => match opponent {
                "A" => solutions.0 += 4, // draw + rock,
                "B" => solutions.0 += 1, // loss + rock,
                "C" => solutions.0 += 7, // win + rock,
                _ => {}
            },
            // paper
            "Y" => match opponent {
                "A" => solutions.0 += 8, // win + paper,
                "B" => solutions.0 += 5, // draw + paper,
                "C" => solutions.0 += 2, // loss + paper,
                _ => {}
            },
            // scissors
            "Z" => match opponent {
                "A" => solutions.0 += 3, // loss + scissors,
                "B" => solutions.0 += 9, // win + scissors,
                "C" => solutions.0 += 6, // draw + scissors,
                _ => {}
            },
            _ => (),
        }

        match opponent {
            // rock
            "A" => match me {
                "X" => solutions.1 += 3, // loss + scissors
                "Y" => solutions.1 += 4, // draw + rock
                "Z" => solutions.1 += 8, // win + paper
                _ => {}
            },
            // paper
            "B" => match me {
                "X" => solutions.1 += 1, // loss + rock
                "Y" => solutions.1 += 5, // draw + paper
                "Z" => solutions.1 += 9, // win + scissors
                _ => {}
            },
            // scissors
            "C" => match me {
                "X" => solutions.1 += 2, // loss + paper
                "Y" => solutions.1 += 6, // draw + scissors
                "Z" => solutions.1 += 7, // win + rock
                _ => {}
            },
            _ => {}
        }
    }

    println!("First solution: {}", solutions.0);

    println!("Second solution: {}", solutions.1);

    Ok(())
}
