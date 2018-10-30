use std::collections::HashMap;

fn main() {
    println!("Hello from task_one!");

    let vector1 = get_vector();

    println!("Mean: {:.2}", find_mean(&vector1));
    println!("Median: {}", find_median(&vector1));
    println!("Mode: {}", find_mode(&vector1));
}

fn get_vector() -> Vec<i32> {
    vec![1, 2, 2, 2, 3, 4, 5, 5, 9]
}

fn find_mean(vec: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;

    for elem in vec {
        sum += elem;
    }

    return sum as f64 / vec.len() as f64;
}

fn find_median(vec: &Vec<i32>) -> i32 {
    let mut copy_vec = vec.clone();

    copy_vec.sort();

    match copy_vec.get(copy_vec.len() / 2) {
        Some(num) => *num,
        None => 0
    }
}

fn find_mode(vec: &Vec<i32>) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for elem in vec.iter() {
        let mut count = hash_map.entry(*elem).or_insert(0);
        *count += 1;
    }

    let mut max_key: i32 = 0;
    let mut max_value: i32 = 0;

    for (key, value) in &hash_map {
        if max_value < *value {
            max_value = *value;
            max_key = *key;
        }
    }

    max_key
}