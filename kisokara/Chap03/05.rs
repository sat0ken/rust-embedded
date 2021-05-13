fn main() {
    // if flow
    let mut x = -1;
    if x == 42 {
        println!("x is 42");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is not 42 but positive");
    }

    // return value by if
    x = -42;
    let abs = if x < 0 { -x } else { x };
    println!("absolute value of x = {}", abs);

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count % 2 == 1 {
            println!("odd");
            continue;
        }

        println!("even");
        
        if count == 4 {
            break;
        }
    }
    
    // while
    let mut while_count = 0;
    while while_count < 10 {
        while_count += 1;
        println!("counter = {}", while_count);
    }

    // for
    for i in 1..5 {
        println!("i = {}", i);
    }

    // iterator
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("a[i] = {}", element);
    }
}
