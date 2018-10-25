fn main() {
    let opt1 = Some(5);
    let opt2: Option<i32> = None;
    let x = 6;

    let result = x + match opt2 {
        Some(val) => val,
        None => 0
    };

    println!("Result: {}", x + opt1.unwrap_or(0));
    println!("Result: {}", result);
}