struct Sensor {
    active: bool,
    latest: u32,
}

struct SensorFusion {
    temperature: Sensor,
    light: Sensor,
}

impl Sensor {
    fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 0,
        }
    }
}

// 引数に&を追加して値の所有権を持っていない、参照している値を受け取る
// 値を借用していることを意味する
// &mutを付けて値を変えられる可変参照にする
fn use_sensor(s: &mut Sensor) {
    // sensorを使って何か意味のあることを行うとする
    // 可変にして値を変える
    s.latest = 42;
} // ここで変数sがスコープを抜ける

fn main() {
    let mut sensor = Sensor::new();
    //&mutを追加して関数を呼ぶときも可変参照の値を渡す
    use_sensor(&mut sensor);

    println!("latest = {}", sensor.latest);
}
