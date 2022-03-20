/// This demonstrates that Rust considers dropped memory invalid
/// DOES NOT COMPILE
fn main() {
    let my_int = 22;
    println!("my_int: {}", my_int);

    let my_int = MyInt(85);
    println!("my_int: {:?}", my_int);

    // drop my_int early
    drop(my_int);
    // the following line produces a compile error (dropped memory)
    println!("my_int: {:?}", my_int);
}

#[derive(Debug)]
struct MyInt(i32);
