fn main() {
    for_range();
    infinite();
}

fn for_range () {
    for n in range(0i, 5) {
        println!("{}", n);
    }
}

fn infinite () {
    let mut i = 5i;
    loop {
        i += i - 3;
        if i % 3 == 0 { break; }
        println!("{}", i);
    }
}
