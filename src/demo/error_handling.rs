use crate::utils;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

// NOTE: Rust错误处理概述
// Rust 的可靠性：错误处理
// - 大部分情况下：在编译时提示错误，并处理错误的分类：
// - 可恢复
//   例如文件未找到，可再次尝试
// - 不可恢复
//   bug,例如访问的索引超出范围
// Rust没有类似异常的机制
// - 可恢复错误：Result<T,E>
// - 不可恢复：panic!宏

// NOTE: 不可恢复的错误与 panic！
// 当 panic!宏执行：
// - 你的程序会打印一个错误信息
// - 展开（unwind）、清理调用栈(Stack)
// - 退出程序

// NOTE: 为应对 panic，月展开或中止（abort)调用栈
// 默认情况下，当 panic 发生:
// - 程序展开调用栈 (工作量大)
//   ·Rust沿着调用栈往回走
//   ·清理每个遇到的函数中的数据
// - 或立即中止调用栈:
//   ·不进行清理，直接停止程序
//   ·内存需要 OS 进行清理
// 想让二进制文件更小，把设置从“展开”改为“中止”：
// - 在 Cargo.toml 中适当的 profile 部分设置:
//   · panic = 'abort'

pub fn run() {
    utils::color_pringln_green("hello Error Handling!");
    use_panic();
    use_result();
    unwrap_or_else();
    use_unwrap();
    propagating_errors();
    use_unwrap1();
    use_custom_type();
}

fn use_panic() {
    // panic!("crash and burn!");
    let v = vec![1, 2, 3];
    // v[999]; // NOTE: RUST_BACKTRACE=1 cargo run 查看错误回溯
    println!("{:?}", v);
}

/// NOTE: 使用Result<T,E>，来处理错误
fn use_result() {
    utils::color_pringln_green("use_result-----------------------");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("f: {:#?}", f);

    // fs::remove_file("hello.txt").unwrap_or_else(|error| {
    //     panic!("Problem removing the file: {:?}", error);
    // })
}

/// NOTE: use unwrap_or_else()，来处理错误
fn unwrap_or_else() {
    utils::color_pringln_green("unwrap_or_else-----------------------");

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("f: {:#?}", f);
    // fs::remove_file("hello.txt").unwrap_or_else(|error| {
    //     panic!("Problem removing the file: {:?}", error);
    // })
}

/// NOTE: use unwrap()，来处理错误
fn use_unwrap() {
    utils::color_pringln_green("use_unwrap-----------------------");

    // let f = File::open("hello.txt").unwrap();
    // NOTE: 👆当没有找到文件时，会 panic, 可以使用 expect() 来自定义错误
    // let f = File::open("hello.txt").expect("无法打开 hello.txt");

    // println!("f: {:#?}", f);
}

/// NOTE: 传播错误 Propagating Errors
fn propagating_errors() {
    utils::color_pringln_green("propagating_errors-----------------------");

    let s = read_username_from_file().unwrap_or_else(|error| {
        panic!("Error: {:?}", error);
    });

    println!("s: {:?}", s);

    // 使用传统的方式传播错误
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    //
    //     let mut s = String::new();
    //
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }

    // NOTE: 使用?传播错误
    // ？运算符
    // ·？运算符：传播错误的一种快捷方式
    // ·如果 Result 是 Ok：Ok 中的值就是表达式的结果，然后继续执行程序
    // ·如果 Result 是 Err：Err 就是整个函数的返回值，就像使用了return
    fn read_username_from_file() -> Result<String, io::Error> {
        // let mut f = File::open("hello.txt")?;
        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        // NOTE: 使用链式调用优化👆代码
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    //  NOTE: ? 只能应在函数返回值是 Result 类型的情况下使用

    // fn test_return_type() {
    //     File::open("hello.txt")?; // 1. the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::FromResidual`)
    //                               //the trait `std::ops::FromResidual<std::result::Result<std::convert::Infallible, std::io::Error>>` is not implemented for `()` [E0277]
    // }
    // 可以修改为👇代码
    fn test_return_type() -> Result<(), Box<dyn Error>> {
        // TODO: Box<dyn Error> 是 trait 对象 暂时简单理解: "为任何可能的错误"
        File::open("hello.txt")?;
        Ok(())
    }
}

// NOTE: ? 与 from 函数
// . Trait std::convert::From 上的 from 函数:
//    - 用于错误之间的转换
// . 被？所应用的错误，会隐式的被 from 函数处理
// . 当？调用 from 函数时:
//    - 它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
// . 用于：针对不同错误原因，返回同一种错误类型
//    - 只要每个错误类型实现了转换为所返回的错误类型的 from 函数

// NOTE: 什么时候因该使用 panic
// 总体原则
// ·在定义一个可能失败的函数时，优先考虑返回 ResuIt
// ·否则就 panic!
// 编写示例、原原型代码、测试
// .可以使用 panic!
//      - 演示某些概念：unwrap
//      - 原型代码：: unwrap、expect
//      - 测试： unwrap、expect
// 有时你比编译器掌握更多的信息
// . 你可以确定 Result 就是 Ok：unwrap
// 错误处理的指导性建议
// ·当代码最终可能处于损坏状态时，最好使用 panic!
// .损坏状态（Bad state）：某些假设、保证、约定或不可变性被打破
// - 例如非法的值、矛盾的值或空缺的值被传入代码
// - 以及下列中的一条：
//  ·这种损坏状态并不是预期能够偶尔发生的事情。
//  ·在此之后，您的代码如果处于这种损坏状态就无法运行。
//  ·在您使用的类型中没有一个好的方法来将这些信息（处于损坏状态）进行编码。

fn use_unwrap1() {
    utils::color_pringln_green("use_unwrap-----------------------");
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {:?}", home);
}

// NOTE: 场景建议
// .调用你的代码，传入无意义的参数值：panic!
// .调用外部不可控代码，返回非法状态，你无法修复：panic!
// .如果失败是可预期的：Result
// .当你的代码对值进行操作，首先应该验证这些值：panic!

/// NOTE: 为验证创建自定义类型
fn use_custom_type() {
    utils::color_pringln_green("use_custom_type-----------------------");

    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if !(1..=100).contains(&value) {
                panic!("Value must be between 1 and 100, got {}", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = "100";
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Not a number");
        }
    };
    let guess = Guess::new(guess);
    println!("value: {}", guess.value());
}
