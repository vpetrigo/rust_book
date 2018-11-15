extern crate my_box as dep;

use dep::my_box;

fn main() {
    let x = 10;
    let y = my_box::MyBox::new(x);

    println!("X: {}, Y: {}", x, *y);
}