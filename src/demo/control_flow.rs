use colored::Colorize;

pub fn run() {
    println!("Hello, world! control flow");
    if_example();
    let_example();
    // loop_example();
    break_example();
    while_example();
    for_example();
    for_range();
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
    println!("{}", "Let's break loop example-----------------".green());
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
    println!("{}", "Let's while loop example-----------------".green());
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

/// NOTE: 使用for循环，遍历集合
fn for_example() {
    println!("{}", "Let's for loop example-----------------".green());
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

/// NOTE: 使用Range，for循环
fn for_range() {
    println!(
        "{}",
        "Let's for_range loop example-----------------".green()
    );
    for number in (1..4).rev() {
        // NOTE: 这里是从 1 到 4，但是不包括 4, rev() 是倒序
        println!("{}!", number);
    }
}
