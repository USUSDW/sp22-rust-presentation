/// This demonstrates that you can't have immutable and mutable references simultaneously
/// DOES NOT COMPILE
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
