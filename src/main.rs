struct Person {
    age: i32,
    name: String,
}

impl Person {
    fn to_string(&self) -> String {
        format!("{} - {}", self.age, self.name)
    }

    fn println(&self) {
        println!("{}", self.to_string());
    }
}

fn main() {
    let simple_data = Person {
        age: 30,
        name: String::from("George Floyd"),
    };
    println!("Hello, world!");
    println!("{} - {}", simple_data.age, simple_data.name);
    simple_data.println();
}
