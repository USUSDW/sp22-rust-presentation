/// This demonstrates that you can't keep a reference to invalid memory
/// DOES NOT COMPILE
fn main() {
    let reference_to_nothing = dangle();
}

// dangle returns a reference to a String
fn dangle() -> &String {
    // s is a new String
    let s = String::from("hello");

    // we return a reference to the String, s
    &s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
