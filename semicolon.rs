fn main () {
    let item = "a";
    let price = if item == "a" { 1.0 }
                else if item == "b" { 2.0 }
                else { 3.0 };
    if price == 1.0 {
        println!("ok");
    }

    if is_four(4) {
        println!("ok");
    }
}

fn is_four (x: int) -> bool { x == 4 }
