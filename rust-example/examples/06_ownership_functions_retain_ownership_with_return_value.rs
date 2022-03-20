/// This demonstrates how you can keep ownership and get a return value without using references
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of a String
    let length = s.len();

    (s, length)
}

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
