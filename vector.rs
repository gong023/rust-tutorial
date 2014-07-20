fn print_vec(v: &[int]) {
    for i in v.iter() {
        println!("{}", i);
    }
}

fn main(){
    let vec1 = vec!(1i, 2i, 3i);
    let vec2 = vec!(4i, 5i, 6i);
    let vec3 = vec1 + vec2;

    for i in range(0u, 6u) {
        println!("{}", vec3.get(i));
    }

    let vec = [1i, 2i, 3i];
    print_vec(vec);
}
