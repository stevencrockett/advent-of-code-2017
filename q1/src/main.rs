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

    let nums: Vec<u32> = contents.chars().map(|x| x.to_digit(10).expect("Couldn't convert to u32")).collect();

    part_one(&nums);
    part_two(&nums);
}

fn part_one(nums: &Vec<u32>) {
    let step_size = 1;

    let sum = list_sum(nums, step_size);
    println!("Part one solution: {}", sum);
}

fn part_two(nums: &Vec<u32>) {
    let step_size = nums.len() / 2;

    let sum = list_sum(nums, step_size);
    println!("Part two solution: {}", sum);
}

fn list_sum(nums: &Vec<u32>, step_size: usize) -> u32 {
    let size = nums.len();
    let mut sum: u32 = 0;
    for (i, elem) in nums.iter().enumerate() {
        let target_index = (i + step_size) % size;
        if elem == &nums[target_index] {
            sum += elem;
        }
    }
    sum
}



