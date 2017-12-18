use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let target = args[1].parse::<i32>().unwrap();
    println!("Read target value: {}", target);

    let total_distance = compute_distance(target);
    

    println!("total distance: {}", total_distance);
}

fn compute_distance(target: i32) -> i32 {
    if target <= 1 {
        0;
    }

    let mut n: i32  = 1;
    let mut current_value: i32 = 1;
    let mut spiral_layer: i32 = 1;

    // find out what layer of spiral number lies on
    while target > current_value {
        spiral_layer += 1;
        n += 2;
        current_value = n * n;
        println!("n is now {}\ncurrent value is now {}", n, current_value);
    }

    // find out what side of the sprial the number is on in that layer
    let step = n - 1;

    let mut next_value = current_value - step;

    while target < next_value {
        current_value = next_value;
        next_value = current_value - step;
        println!("current value is now {}\nnext value is now {}", current_value, next_value);
    }

    // compute 'middle number for row'
    let middle_number = next_value + (n / 2);
    println!("middle number is {}", middle_number);

    let distance_to_middle_of_side = i32::abs(target - middle_number);
    distance_to_middle_of_side + spiral_layer - 1
}
