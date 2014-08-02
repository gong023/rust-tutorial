use std::string::String;

mod print;

struct Plain { num: int, says: String }
impl Plain {
    fn execute(self) {
        ::print::say(self.num, self.says);
    }
}

fn main() {
    let plain = Plain { num: 3, says: String::from_str("Fizz") };
    plain.execute();

    let hi = ::print::Hi;
    hi.execute();
}
