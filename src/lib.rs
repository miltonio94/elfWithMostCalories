use std::{error::Error, fs};

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let chunked_numbers = parse_list(&content);

    let mut summed_chunks: Vec<i32> = chunked_numbers
        .iter()
        .map(|numbers| numbers.iter().sum::<i32>())
        .collect();

    summed_chunks.sort();

    let reverse_sorted: Vec<&i32> = summed_chunks.iter().rev().collect();

    for number in &reverse_sorted {
        println!("{}", number.to_string());
    }

    let (first, second, third) = (reverse_sorted[0], reverse_sorted[1], reverse_sorted[2]);

    println!(
        "The top highest caloric chunks are: {} {} {}",
        first.to_string(),
        second.to_string(),
        third.to_string()
    );

    println!("The sum of these are {}", (first + second + third).to_string());

    Ok(())
}

fn parse_list(content: &str) -> Vec<Vec<i32>> {
    let chunked_content: Vec<&str> = content.trim().split("\n\n").collect();

    let chunked_numbers: Vec<Vec<i32>> = chunked_content
        .iter()
        .map(|chunk| {
            let numbers: Vec<i32> = chunk
                .split("\n")
                .map(|entry| {
                    let out: i32 = entry.parse().unwrap();
                    out
                })
                .collect();
            numbers
        })
        .collect();

    chunked_numbers
}
