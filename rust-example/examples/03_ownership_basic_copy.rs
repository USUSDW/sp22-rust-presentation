/// This demonstrates that types that implement the Copy trait don't get moved.
fn main() {
    let x = 5;
    let y = x; // since i32 implements Copy, there's no need for a move

    println!("x = {}, y = {}", x, y);
}
