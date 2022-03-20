/// This demonstrates that you can have multiple mutable references as long as they're usage doesn't overlap
fn main() {
    let mut s = String::from("hello");
    println!("s: {}", s);

    {
        let r1 = &mut s;
        println!("r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r2: {}", r2);
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
