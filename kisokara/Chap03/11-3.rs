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

// 所有権を返すために引数を毎回明示するのは不便
fn use_sensor(s: Sensor) -> Sensor {
    // sensorを使って何か意味のあることを行うとする
    println!("latest = {}", s.latest);
    s
} // ここで変数sがスコープを抜ける

fn main() {
    let sensor = Sensor::new();
    let sensor = use_sensor(sensor);

    // Compile error, 引き続きmain側でもsensorを使いたいが,コンパイルエラーとなる
    // use_sensor()関数が所有権を返さないといけない
    println!("latest = {}", sensor.latest);
}
