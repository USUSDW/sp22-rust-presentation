/// This demonstrates that memory is invalid after it is moved
fn main() {
    let s1 = String::from("hello");
    let _s2 = s1; // underscore to ignore unused variable warning

    // compile error: s1 was moved and is no longer valid
    println!("{}, world!", s1);
}
