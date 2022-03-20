/// This demonstrates how moves and copies work with function parameters
fn main() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function
    // and so is no longer valid here
    takes_ownership(s);

    // uncomment to verify that s is no invalid
    // println!("main | {}", s);

    // x comes into scope
    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    makes_copy(x);
    println!("main | {}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("takes_ownership | {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("makes_copy | {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
