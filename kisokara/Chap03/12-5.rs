//構造体Sensorのインターフェイスとしてトレイトを定義
trait Sensor {
    // fn メソッド名(引数, ...) -> 戻り値型;
    // トレイト定義内ではメソッド本体は定義しない
    // トレイトを実装する構造体でメソッド本体は定義される
    fn read(&self) -> u32;
}

struct LightSensor {
    value: u32,
}

// 定義したトレイトを構造体に実装
impl Sensor for LightSensor {
    fn read(&self) -> u32 {
        self.value
    }
}

struct TemperatureSensor {
    value: u32,
}

// 定義したトレイトを構造体に実装
impl Sensor for TemperatureSensor {
    fn read(&self) -> u32 {
        self.value as u32
    }
}

// トレイト境界を使った実装
// トレイト境界(:Sensor)は型パラメータに制約を追加する
// 型SにSensorトレイトが実装されていなければならない
fn print_sensor_value<S: Sensor>(sensor: S) {
    println!("sensor value = {}", sensor.read());    
}

fn main() {
    let light_sensor = LightSensor{value: 42};
    let temp_sensor  = TemperatureSensor{value: 35};
    
    print_sensor_value(light_sensor);
    print_sensor_value(temp_sensor);
}
