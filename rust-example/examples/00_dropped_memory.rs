fn main() {
    let my_int = 22;
    println!("my_int: {}", my_int);

    // the following commented out lines produce a compile error
    // my_int = 87;
    // println!("my_int: {}", my_int);

    let my_int = MyInt(22);
    println!("my_int: {:?}", my_int);

    // drop my_int early
    drop(my_int);
    // the following commented out lines produce a compile error
    // println!("my_int: {:?}", my_int);
}

#[derive(Debug)]
struct MyInt(i32);

impl Drop for MyInt {
    fn drop(&mut self) {
        println!("dropping {:?}", self);
    }
}
