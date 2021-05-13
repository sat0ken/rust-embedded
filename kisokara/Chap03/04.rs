fn main() {
    let x = 4.24242;  // f64
    let y: f32 = 2.42424; // f32

    println!("x = {}, y = {:.2}", x, y);

    // array
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array = {:?}", array);
    println!("array[0] = {}", array[0]);

    // slice
    let slice: &[i32] = &array[1..3];
    println!("slice = {:?}", slice);
    println!("slice[0] = {}", slice[0]);

    // mutable slice
    let slice2 = &mut array[1..3];
    slice2[0] = 6;
    println!("slice2 = {:?}", slice2);
    println!("array = {:?}", array);

    // tuple
    let t: (u8, i32, usize) = (1, -42, 1_000);
    println!("i32 = {}", t.1);

    // string
    let greeting = "hello";
    println!("{}", greeting);

    let byte_greeting = b"hello";
    println!("{:?}", byte_greeting);
}
