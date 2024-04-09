pub fn run() {
    println!("Hello, world! control flow");
    if_example();
    let_example();
    // loop_example();
    break_example();
}

/// NOTE: in rust, all control flow is 'if'
fn if_example() {
    let number = 7;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}

/// NOTE:Using if in a let Statement
fn let_example() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);
}

/// NOTE: loop
fn loop_example() {
    loop {
        println!("again!");
    }
}

/// NOTE: break
fn break_example() {
    println!("Let's break loop example-----------------");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);
}

/// NOTE: while 条件循环
fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        mumber = 1;
    }
    println!("LIFTOFF!!!");
}
