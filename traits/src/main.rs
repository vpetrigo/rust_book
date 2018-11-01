extern crate my_trait;

use my_trait::my_trait::MyTrait;

struct User;

impl MyTrait for User {
    fn my_trait_test(&self) -> String {
        format!("MyTrait for User")
    }

    fn my_trait_static() -> String {
        format!("Static MyTrait function for User")
    }
}

fn main() {
    let user = User{};

    println!("{}", user.my_trait_test());
    println!("{}", User::my_trait_static());
}