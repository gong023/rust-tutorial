fn owned_seven() -> Box<int> {
    let three: Box<int> = box 3i;
    let four: Box<int> = box 4i;

    let seven = *three + *four;
    box seven
}

fn borrowd_seven() -> int {
    let three: &int = &3;
    let four: &int = &4;
    *three + *four
}

fn move_semantics(seven: Box<int>) -> int {
    let new_seven = seven;
    *new_seven
    // *seven ダメ
}

struct Hoge { x: int }
struct Fuga { x: Box<int> }

fn struct_pointer() {
    let a = Hoge { x: 1i };
    let aa = a;
    println!("{}", a.x); // 大丈夫。値はコピーされる
    println!("{}", aa.x);

    let b = Fuga { x: box 1i };
    let bb = b;
    //println!("{}", b.x); // これはダメ。ownership はbbに移る
    println!("{}", bb.x);
}

fn main() {
    println!("{}", owned_seven());
    println!("{}", borrowd_seven());
    println!("{}", move_semantics(box 7i));
    struct_pointer();
}
