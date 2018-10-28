fn largest<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x > y {
        x
    }
    else {
        y
    }
}

fn main() {
    let first = 10;
    let second = 20;
    let result = largest(&first, &second);
    println!("Largest is {}", result);
}