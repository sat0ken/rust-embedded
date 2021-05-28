fn main() {
    let rx;
    {
        let x = 42;
        rx = &x;
    } // xのスコープはここまで
    
    // Compile error, xは破棄されているにもかかわらずrxから値を参照しようとしているため
    println!("rx = {}", rx);
}
