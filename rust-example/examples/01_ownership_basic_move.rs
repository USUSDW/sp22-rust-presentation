/// This demonstrates that memory is invalid after it is moved
/// DOES NOT COMPILE
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // compile error: s1 was moved and is no longer valid
    println!("using s1: {}, world!", s1);
    println!("using s2: {}, world!", s2);
}

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
