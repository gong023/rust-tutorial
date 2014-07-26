struct ScubaArgentine {
    strength: int
}

struct IndustrialRaverMonkey {
    strength: int
}

enum Monster {
    ScubaArgentine(int),
    IndustrialRaverMonkey(int)
}

impl Monster {
    fn attach(&self) {
        match *self {
            ScubaArgentine(s) => println!("The ScubaArgentine attacks for {:d} damage.", s),
            IndustrialRaverMonkey(s) => println!("The IndustrialRaverMonkey for {:d} damage.", s)
        }
    }
}

fn main() {
      let irm = IndustrialRaverMonkey(46);
      irm.attach();

      let sca = ScubaArgentine(64);
      sca.attach();
}
