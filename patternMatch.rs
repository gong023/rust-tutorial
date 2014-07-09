fn main() {
    num_case(1i);
}

fn num_case(num: int) -> char {
    match num {
        0     => { 'a' },
        1 | 2 => { 'b' },
        3..10 => { 'c' },
        _     => { 'd' }
    }
}
