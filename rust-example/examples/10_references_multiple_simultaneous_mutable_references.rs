/// This demonstrates that you can't have 2 simultaneously active mutable references
/// DOES NOT COMPILE
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
