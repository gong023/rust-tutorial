extern crate debug;

fn apply(i: int, f: |int|->int) -> int {
    f(i)
}

fn apply_with_none(f: ||) -> () {
    f();
}

fn apply_with_10(f: |int|->int) -> int {
    f(10)
}

fn rpc_call(p: proc(v: int)) {
    p(10)
}

fn each(v: &[int], f: |v: &int|) {
    let mut i = 0;
    while i < v.len() {
        f(&v[i]);
        i += 1;
    }
}

fn main() {
    let should_four = apply(2, { |x| x * x });
    assert_eq!(4i, should_four);

    let closure = || println!("Hello");
    apply_with_none(closure);

    let should_twenty = apply_with_10({ |i| i * 2 });
    assert_eq!(20i, should_twenty);

    let mut max = 0i;
    {
        let find_max = |x: int| if x > max { max = x };
        for x in [1, 2, 3].iter() {
            find_max(*x);
        }
    }
    assert_eq!(3i, max);

    [1, 2, 3].iter().map(|x| if *x > max { max = *x });
    assert_eq!(3i, max);

    rpc_call(proc(n) { println!("{:?}", n); });

    each([1, 2, 3], |v: &int| {
        println!("{:?}", v);
    });
}
