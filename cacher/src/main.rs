extern crate cacher;

use std::thread;
use std::time::Duration;

use cacher::sup;

fn main() {
    const HARDCODED_INTENSITY: u32 = 10;
    const HARDCODED_RANDOM_NUMBER: u32 = 2;

    generate_workout(HARDCODED_INTENSITY, HARDCODED_RANDOM_NUMBER);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = sup::Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
