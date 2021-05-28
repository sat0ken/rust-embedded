// ライフタイムパラメータを構造体とフィールドにセット
struct Image<'a> {
    raw: &'a [u8; 256],
}

fn main() {
    let image;
    {
        let bytes = [0; 256];
        image = Image {
            raw: &bytes,
        };
    } // bytesのスコープはここまで
    println!("the first byte in Image = {}", image.raw[0]);
} // imageのスコープはここまで
