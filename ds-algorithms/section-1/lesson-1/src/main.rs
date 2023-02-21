pub struct Person {
    name: String, // String is a pointer
    age: u8,
    children: u8,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "Name: {}, age: {} has {} children.",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let person = Person {
        name: "Paul",
        age: 39,
        children: 1,
    };
    println!("Hello, world!");
}
