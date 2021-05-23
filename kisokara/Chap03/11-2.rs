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

fn main() {
    let sensors = SensorFusion{
        temperature: Sensor::new(),
        light: Sensor::new(),
    };
    // sensorsはlightの所有権を保持している
    println!("sensors.light.latest = {}", sensors.light.latest);

    // lightの所有権をムーブする
    let _l = sensors.light;
    // Compile error, sensorはlightの所有権をもう保持していない
    println!("light.sensors.latest = {}", sensors.light.latest);
}
