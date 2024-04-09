use crate::utils;
pub fn run() {
    println!("Hello, Option");
    // NOTE: Option 存在于Prelude中
    use_option();
    use_match();
    use_placeholder();
}

fn use_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    dbg!(some_number);
    dbg!(some_string);
    let mut anything: Option<i32> = None;
    anything = Some(1);
    dbg!(anything);

    {
        let x = 5;
        let y: Option<i32> = Some(4);
        // NOTE: 在对 Option<T> 进行 T 的运算之前必须将其转换为 T。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。
        // let sum = x + y; // cannot add `Option<i32>` to `{integer}`
        let sum = x + Option::unwrap(y);
        println!("sum = {}", sum);
    }

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            // NOTE: 必须穷举所有可能
            // match x { // missing match arm: `None` not covered
            //     Some(i) => Some(i + 1),
            // }
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("six = {:?}", six);
    }
}

// NOTE: match 语句
fn use_match() {
    #[derive(Debug)]
    enum UsState {
        Alaska,
        Arizona,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // NOTE: 绑定值的模式
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("dime!");
                10
            }
            Coin::Quarter(state) => {
                println!("quarter from {:?}", state);
                25
            }
        }
    }

    let q = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("q={}", q);
    let p = value_in_cents(Coin::Penny);
    println!("p={}", p);
}

// NOTE: 通配模式和 _ 占位符
fn use_placeholder() {
    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (), // _ 表示匹配任何值
    }
}
