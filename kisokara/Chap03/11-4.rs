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
fn use_sensor(s: &Sensor) {
    // sensorを使って何か意味のあることを行うとする
    println!("latest = {}", s.latest);
} // ここで変数sがスコープを抜ける

fn main() {
    let sensor = Sensor::new();
    //&を追加して関数を呼ぶときも参照の値を渡す
    use_sensor(&sensor);

    println!("latest = {}", sensor.latest);
}
