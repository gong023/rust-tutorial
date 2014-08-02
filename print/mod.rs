use std::string::String;

pub fn say(num: int, says: String) {
    println!("num: {}, says: {}", num, says);
}

pub struct Hi;

impl Hi {
    pub fn execute(self) {
        println!("hi");
    }
}
