const FOURTY_TWO: i32 = 42;
static mut X: i32 = 42;
fn main() {
    let x = 42;
    println!("x = {}", x);

    let mut y  =42;
    println!("y = {}", y);
    y = 24;
    println!("y = {}", y);

    println!("FOURTY_TWO = {}", FOURTY_TWO);

    unsafe {
        X += 1;
        println!("X = {}", X);
    }
}
