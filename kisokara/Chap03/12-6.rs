//構造体Sensorのインターフェイスとしてトレイトを定義
trait Sensor {
    // fn メソッド名(引数, ...) -> 戻り値型;
    // トレイト定義内ではメソッド本体は定義しない
    // トレイトを実装する構造体でメソッド本体は定義される
    fn read(&self) -> u32;

    // トレイト定義内のメソッドでデフォルト実装を定義
    // 上書きされない限りデフォルト実装が呼び出される
    fn fill(&self, buffer: &mut [u32]) {
        for element in buffer.iter_mut() {
            *element = self.read();
        }
    }
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
    
    let mut buf = [0u32; 4];
    light_sensor.fill(&mut buf);
    println!("buf = {:?}", buf);
}
