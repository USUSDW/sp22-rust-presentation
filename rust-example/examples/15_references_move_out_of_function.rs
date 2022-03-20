/// This demonstrates that you need to move values out of functions instead of returning a reference
fn main() {
    let string = no_dangle();
    println!("main | {}", string);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    println!("no_dangle | {}", s);

    s
}
