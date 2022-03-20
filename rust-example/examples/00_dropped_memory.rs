/// This demonstrates that Rust considers dropped memory invalid
fn main() {
    let my_int = 22;
    println!("my_int: {}", my_int);

    // the following commented out lines produce a compile error (mutability)
    // my_int = 87;
    // println!("my_int: {}", my_int);

    // drop my_int early
    drop(my_int);
    // the following commented out line produces a compile error (dropped memory)
    // println!("my_int: {:?}", my_int);
}
