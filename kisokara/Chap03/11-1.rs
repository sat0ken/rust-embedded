struct Sensor {
    active: bool,
    latest: u32,
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
    // Compile Error, Xの所有権はYにムーブされておりその値を借用できない
    let x = Sensor::new();
    let y = x;
    println!("x.latest = {}", x.latest);
    println!("y.latest = {}", y.latest);

}
