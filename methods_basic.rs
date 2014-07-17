struct Monster {
    attack: int,
    defence: int
}

impl Monster {
    fn print_attack(&self) {
        println!("attack is {}", self.attack);
    }

    fn print_defence(&self) {
        println!("defence is {}", self.defence);
    }

    fn print_num(num: int) {
        println!("{}", num);
    }
}

fn main() {
    let mon = Monster { attack: 10, defence: 20 };
    mon.print_attack();
    mon.print_defence();
    Monster::print_num(5);
}
