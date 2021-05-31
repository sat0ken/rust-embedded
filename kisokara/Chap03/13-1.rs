fn main() {
    let x: i32 = 42;
    // Compile error, i32とi64は別の型
    //let y: i64 = x;
    
    // into()メソッドを使って型変換
    //let y: i64 = x;

    //型名::from(値)で変換することも可能
    let y = i64::from(x);

    // コンパイラが知っている範囲での安全な型変換ではasが利用できる
    // let y = x as i64;

    println!("y = {}", y);
}
