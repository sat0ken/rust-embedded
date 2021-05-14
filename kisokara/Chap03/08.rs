struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    // オブジェクトの値を変更しない場合、&selfを引数に取る
    fn read(&self) -> u32 {
        self.latest
    }

    // オブジェクトの値を変更する場合、&mut selfを引数に取る
    /*fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }*/

    // 関連引数は引数でselfを受け取らない。コンストラクタとして利用する
    fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 0,
        }
    }
}

fn main() {
    /*let mut sensor = Sensor {
        active: false,
        latest: 0,
    };*/
    let mut sensor = Sensor::new();
    let latest = sensor.read();
    println!("active = {}, latest = {}", sensor.active, sensor.latest);
}
