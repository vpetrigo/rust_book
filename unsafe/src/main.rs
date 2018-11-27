fn main() {
    let mut x = 10;
    let y = &mut x as *mut i32;
    let k = &mut x as *mut i32;
    let z = &x as *const i32;

    unsafe {
        println!("Mutable ptr: {}", *y);
        println!("Mutable ptr: {}", *k);
        println!("Immutable ptr: {}", *z);
        *y = 15;
        println!("Mutable ptr: {}", *y);
        println!("Mutable ptr: {}", *k);
        println!("Immutable ptr: {}", *z);

        *y = *y + *z;
        *k = 24;
        println!("Mutable ptr: {}", *y);
        println!("Mutable ptr: {}", *k);
        println!("Immutable ptr: {}", *z);
    }

    println!("Origin value: {}", x);
}