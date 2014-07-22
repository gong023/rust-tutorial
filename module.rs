use std::string::String;

mod print {
    pub fn say(num: int, says: String) {
        println!("num: {}, says: {}", num, says);
    }
}

struct Plain { num: int, says: String }
impl Plain {
    fn execute(self) {
        ::print::say(self.num, self.says);
    }
}

fn main() {
    let plain = Plain { num: 3, says: String::from_str("Fizz") };
    plain.execute();
}
