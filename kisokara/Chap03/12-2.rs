// ジェネリックな構造体を定義
// Point<T>は任意の型Tのxとyをフィールドに持つ
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 関連関数
    fn new(x: T, y: T) -> Point<T> {
        Point{x, y}
    }
    
    // メソッド
    fn x(&self) -> &T {
        &self.x
    }

}

//ジェネリック関数
fn make_point<T>(x: T, y: T) -> Point<T> {
    Point{x, y}
}

fn main() {
    let p_i32 = make_point::<i32>(300, 400);
    let p_i8  = make_point::<i8>(10, 20);
    let p = Point::new(10, 2);
    let p2 = make_point(1, 2);
    
    use std::mem::size_of_val;
    println!("size of x in p_i32 = {}", size_of_val(&p_i32.x));
    println!("size of x in p_i8 = {}", size_of_val(&p_i8.x));
    println!("p = {}", p.x);
    println!("p2 = {}", p2.x);

}
