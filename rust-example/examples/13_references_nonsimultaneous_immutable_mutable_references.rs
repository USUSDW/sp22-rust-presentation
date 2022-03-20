/// This demonstrates that you can have immutable and mutable references if they aren't used at the same time
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
