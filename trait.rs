trait Monster {
    fn attach(&self);
}

struct NormalMonster {
    strength: int
}

struct StrongMonster {
    strength: int
}

impl Monster for NormalMonster {
    fn attach(&self) {
        println!("{:d}", self.strength);
    }
}

impl Monster for StrongMonster {
    fn attach(&self) {
        println!("{:d}", self.strength + 10);
    }
}

fn main() {
    let normal_monster = NormalMonster { strength: 35 };
    normal_monster.attach();

    let strong_monster = StrongMonster { strength: 35 };
    strong_monster.attach();
}
