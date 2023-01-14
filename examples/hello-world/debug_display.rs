use std::fmt::Display;

#[derive(Debug)]
pub struct Boy {
    pub name: String,
    pub age: i32,
}

impl Display for Boy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I'am boy : ( name: {}, age: {})", self.name, self.age)
    }
}

pub fn show_boy() {
    let boy = Boy {
        name: "bob".to_string(),
        age: 13,
    };

    println!("pretty print: this is a boy: {:#?}", boy);
    println!("common print: this is a boy: {:?}", boy);
    println!("display print: this is a boy: {}", boy);
}


