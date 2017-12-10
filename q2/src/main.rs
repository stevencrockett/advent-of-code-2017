use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file = &args[1];
    println!("Will attempt to read input from file: {}", file);

    let mut f = File::open(file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error reading file");
    let contents = contents.trim();

    println!("Read text from file: \n{}", contents);

    let matrix: Vec<Vec<i32>> = contents.lines().map(|line| to_number_vec(line)).collect();

    println!("{:?}", matrix);

    part_one(&matrix);
    part_two(&matrix);
}

fn to_number_vec(line : &str) -> Vec<i32> {
    let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().expect("Couldn't convert to i32")).collect();
    return nums;
}

fn part_one(matrix: &Vec<Vec<i32>>) {
    let sum: i32 = matrix.into_iter().map(|row| part_one_solve_for_row(row)).sum();
    println!("Part one solution: {}", sum);
}

fn part_one_solve_for_row(row: &Vec<i32>) -> i32 {
    let max = row.iter().max().unwrap_or(&0);
    let min = row.iter().min().unwrap_or(&0);
    max - min
}


fn part_two(matrix: &Vec<Vec<i32>>) {
    let sum: i32 = matrix.into_iter().map(|row| part_two_solve_for_row(row)).sum();
    println!("Part two solution: {}", sum);
}

fn part_two_solve_for_row(row: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for (i, item) in row.into_iter().enumerate() {
        for item2 in &row[(i + 1)..] {
            if item % item2 == 0 {
                sum += item / item2;
                break;
            }

            if item2 % item == 0 {
                sum += item2 / item;
                break;
            }
        }
    }
    sum
}