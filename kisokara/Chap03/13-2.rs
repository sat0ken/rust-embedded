fn main() {
    // 明示的にインポートが必要
    use std::convert::TryInto;
    let x: i64 = 42;

    // 失敗するかもしれない変換ではTryIntoとTryFromトレイトを利用する
    // try_into()とtry_from()の戻り値はResult型なので、unwrap()を呼んで中身を取得
    let y: i32 = x.try_into().unwrap();
    // let y = i32::try_from(x).unwrap();

    println!("y = {}", y);
}
