struct Foo {
    x: int,
    y: Box<int>
}

fn a_scope() {
    Foo { x: 5, y: box 10 };
}

fn main() {
    let mut b = Foo { x: 5, y: box 10 };
    b.x = 10;
    assert_eq!(b.x, 10i);
    println!("{}", b.y);
    println!("{}", a_scope())
}
