use std::io;

fn main() {
    println!("Hello from task_two");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Enter a word");
    println!("Pig word: {}", convert_to_pig(&user_input.trim().to_string()));
}

fn convert_to_pig(s: &String) -> String {
    let mut chars = s.chars().peekable();
    let first_char = chars.peek().cloned();

    let suffix = match first_char {
        Some(ch) => match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' |
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => String::from("-hay"),
            'a'...'z' | 'A'...'Z' => {
                chars.next();
                format!("-{}ay", ch)
            },
            _ => String::new(),
        }
        None => String::new(),
    };

    if suffix.len() != 0 {
        return format!("{}{}", chars.collect::<String>(), suffix);
    }

    suffix
}