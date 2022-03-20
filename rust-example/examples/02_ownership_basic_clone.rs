/// This demonstrates that clone will allow you to explicitly deep copy the memory you want
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
