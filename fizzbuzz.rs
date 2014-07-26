use std::string::String;

trait Print {
    fn says(self, i: int);
}

struct Fizz {
    criteria: int,
    word: String
}

struct Buzz {
    criteria: int,
    word: String
}

mod fizzbuzz {
    pub fn say_with_judge(i: int, criteria: &int, word: &String) {
        if i % *criteria == 0 {
            println!("{}", word);
        }
    }
}

impl Print for Fizz {
    fn says(self, i: int) {
        ::fizzbuzz::say_with_judge(i, &self.criteria, &self.word);
    }
}

impl Print for Buzz {
    fn says(self, i: int) {
        ::fizzbuzz::say_with_judge(i, &self.criteria, &self.word);
    }
}

fn main() {
    for i in range(0i, 16) {
      let f = Fizz { criteria: 3, word: "Fizz".to_string() };
      let b = Buzz { criteria: 5, word: "Buzz".to_string() };
      f.says(i);
      b.says(i);
      println!("{}", i);
    }
}
