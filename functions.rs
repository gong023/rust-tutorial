fn main() {
    let num = line(1i, 2i, 3i);
    println!("{}", num);
    let first = get_first((1i, 1.0));
    println!("{}", first);
}

fn line(a:int, b:int, x:int) -> int {
    return a * x + b
}

fn get_first((value, _): (int, f64)) -> int { value }
