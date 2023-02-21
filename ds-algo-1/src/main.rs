// Udemy 1.3s

pub struct Person {
    name: String, // String is a pointer
    age: u8,
    children: u8,
}

impl Person {
    pub fn print(self) -> String {
        let mut sp = "children";
        if self.children == 1 {
            sp = "child";
        }
        format!(
            "Name: {}, age: {} has {} {}.",
            self.name, self.age, self.children, sp
        )
    }
}

fn main() {
    let _person = Person {
        name: "Paul".to_string(),
        age: 39,
        children: 1,
    };
    println!("Hello, World!"); // deliberate error to test rust-analyzer
}
