fn main() {
    let s = String::from("Hello world");
    let x = 5;

    {
        let hello1 = first_word(&s);
        let hello2 = first_word(&s);

        println!("{}", hello1);
        println!("{}", hello2);
    }

    take_ownership(s);
    make_copy(x);
    println!("{}: {}", "main", x);

    let s1 = String::from("Rust");
    let (s1, size) = calculate_length(s1);

    println!("String {} has length {}", s1, size);
}

fn take_ownership(string: String) {
    println!("{}: {}", "take_ownership", string);
}

fn make_copy(some_int: i32) {
    println!("{}: {}", "make_copy", some_int);
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of a String
    let length = s.len();

    fn test_print(s: &String) {
        println!("{}", s);
    }

    test_print(&s);
    (s, length)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
