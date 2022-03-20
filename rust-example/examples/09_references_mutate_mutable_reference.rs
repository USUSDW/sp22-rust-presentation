fn main() {
    let mut s = String::from("hello");
    println!("s before change: {}", s);

    change(&mut s);
    println!("s after change: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
