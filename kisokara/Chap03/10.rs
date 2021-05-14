#[derive(Debug)]
enum Error {
    Overflow,
    Zero,
}

fn double(number: u32) -> Result<u32, Error> {
    if number == 0 {
        Err(Error::Zero)
    }
    else if number > (u32::MAX / 2) {
        Err(Error::Overflow)
    } else {
        Ok(number * 2)
    }
}

fn func() -> Result<u32, Error> {
    // エラーが発生すると、main関数にエラーをそのまま伝播する
    let doubled = double(u32::MAX)?;
    Ok(doubled)
}

fn main() {
    let doubled = double(10).unwrap();
    println!("doubled = {}", doubled);

    match double(10) {
        Ok(x) => println!("doubled = {}", x),
        Err(_e) => println!("double failed"),
    }
    if let Ok(x) = double(u32::MAX) {
        println!("doubled is {}", x)
    }
    
    if let Err(e) = func() {
        println!("func failed with {:?}", e);
    }

    /*match double(10) {
        Some(x) => println!("doubled = {}", x),
        None => println!("double failed"),
    }

    if let Some(x) = double(10) {
        println!("doubled = {}", x);
    }
    if let Some(x) = double(u32::MAX) {
        println!("doubled = {}", x);
    }*/
}

