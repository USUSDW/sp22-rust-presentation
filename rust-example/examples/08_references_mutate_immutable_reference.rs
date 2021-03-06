/// This demonstrates that references are immutable by default, just like normal variables
/// DOES NOT COMPILE
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
