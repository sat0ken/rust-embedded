// ジェネリックな構造体を定義
// Point<T>は任意の型Tのxとyをフィールドに持つ
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p_i32 = Point::<i32> {x: 300, y: 400};
    let p_i8  = Point::<i8> {x: 10, y: 20};

    use std::mem::size_of_val;
    println!("size of x in p_i32 = {}", size_of_val(&p_i32.x));
    println!("size of x in p_i8 = {}", size_of_val(&p_i8.x));

}
