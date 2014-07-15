fn apply(i: int, f: |int|->int) -> int {
    f(i)
}

fn apply_with_none(f: ||) -> () {
    f();
}

fn apply_with_10(f: |int|->int) -> int {
    f(10)
}

fn main() {
    let should_four = apply(2, { |x| x * x });
    assert_eq!(should_four, 4i);

    let closure = || println!("Hello");
    apply_with_none(closure);

    let should_twenty = apply_with_10({ |i| i * 2 });
    assert_eq!(should_twenty, 20i);

    let mut max = 0i;
    {
        let find_max = |x: int| if x > max { max = x };
        for x in [1, 2, 3].iter() {
            find_max(*x);
        }
    }
    println!("max={}", max);

    [1, 2, 3].iter().map(|x| if *x > max { max = *x });
    println!("max={}", max);
}
