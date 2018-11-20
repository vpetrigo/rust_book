trait Draw {
    fn draw(&self);
}

struct Button {
    width: usize,
    height: usize,
    label: String,
}

struct Window {
    width: usize,
    height: usize,
}

struct Screen {
    elements: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            elements: vec![]
        }
    }

    fn add(&mut self, element: Box<dyn Draw>) {
        self.elements.push(element);
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw Button - width {width}, height {height}, label \"{label}\"!",
                 width = self.width, height = self.height, label = self.label);
    }
}

impl Draw for Window {
    fn draw(&self) {
        println!("Draw Window of size {} x {}!", self.width, self.height);
    }
}

impl Draw for Screen {
    fn draw(&self) {
        for element in &self.elements {
            element.draw();
        }
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        println!("Drop Button!");
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("Drop Window!");
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        println!("Drop Screen!");
    }
}

fn main() {
    let mut screen = Screen::new();
    let button = Box::new(Button {
        width: 100,
        height: 50,
        label: String::from("Hello"),
    });
    let window = Box::new(Window {
        width: 800,
        height: 480,
    });

    screen.add(button);
    screen.add(window);
    screen.draw();
}
