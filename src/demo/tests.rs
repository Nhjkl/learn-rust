use crate::utils;
pub fn run() {
    utils::color_pringln_green("Hello, tests");
}

// NOTE: cargo test 可执行这个项目所有的测试

#[test]
fn is_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_panic() {
    panic!("test panic");
}

// NOTE: 使用assert! 宏检查测试结果
// assert!宏，来自标准库，用来确定某个状态是否为true
// - true: 测试通关
// - false: 调用panic!,测试失败
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[test]
fn small_cannot_hold_big() {
    let small = Rectangle {
        height: 10,
        width: 10,
    };
    let big = Rectangle {
        height: 20,
        width: 20,
    };
    assert!(small.can_hold(&big));
}

#[test]
fn big_can_hold_small() {
    let small = Rectangle {
        height: 10,
        width: 10,
    };
    let big = Rectangle {
        height: 20,
        width: 20,
    };
    assert!(big.can_hold(&small));
}

// NOTE: 使用 assert_eq! 和 assert_ne! 来测试相等性
fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn is_add_two() {
    // assert_eq!(4, add_two(2));
    assert_ne!(5, add_two(2));
    // NOTE: 添加自定义的错误信息
    // 可以向assert!, assert_eq!, assert_ne! 天际可选的自定义信息
    // - 这些自定义信息和失败信息都会打印出来
    // - assert!: 第一个参数必填，自定义信息作为第2个参数
    // - assert_eq!, assert_ne!: 前2个参数必填，自定义信息作为第3个参数
    // - 自定义信息参数会被传递给format!,可以使用{}占位符
    assert!(
        4 == add_two(1),
        "assert 4 == add_two(1) -> {}, 4 != {}",
        add_two(1),
        add_two(1)
    );
}

// NOTE: 使用 should_panic 来检查panic
#[test]
#[should_panic]
fn should_panic() {
    panic!("test panic");
}

#[test]
#[should_panic(expected = "test panic")] // NOTE: panic信息包含expected指定的信息，表示通关
fn should_panic_expected() {
    panic!("test 😡 panic ");
}

// NOTE: 使用Result<T, E>，可以用来检查测试结果
#[test]
fn test_result() -> Result<(), String> {
    // 返回Result的方式不会发生panic, 所有使用should_panic没有意义
    if 2 == 2 {
        Ok(())
    } else {
        Err("test error".to_string())
    }
}

// NOTE: cargo test 命令行参数
// cargo test --help 显式可以跟在 test 之后的所有参数
// cargo test -- --help 显式可以跟在 -- 之后的所有参数
// 并行运行测试
// 运行多个测试：默认使用多个线程并行运行，
// - 运行快
// 确保测试之间：
// - 不会互相依赖
// - 不依赖于某个共享状态（环境、工作目录、环境变量等等)
// --test-threads 参数
// 传递给二进制文件
// 不想以并行方式运行测试，或想对线程数进行细粒度控制
// 可以使用--test-threads 参数，后边跟着线程的数量
// 例如: cargo test -- --test-threads=1

// NOTE: 显式函数输出
// 默认成功的测试，不会打印函数中的输出信息，如果需要可加加上--show-output 参数
// 如：cargo test -- --show-output

// NOTE: 按名称运行测试
// 如：cargo test test_name
// test_name 是模糊匹配的如果传 test_,就可以匹配 test_ 开头的函数

// NOTE: 忽略测试,运行剩余的测试
// example：
// 可以单独运行标记ignore的测试，通过 cargo test -- --ignored
#[test]
#[ignore]
fn test_ignore() {}

// NOTE: 测试的分类
// 单元测试
// - 小，专注
// - 一次对一个模块进行隔离的测试
// - 可以测试private接口
// 集成测试
// - 在库外部。和其他外部代码一样使用你的代码
// - 只能使用public接口

// NOTE:
// #[cfg(test)] 标注
// tests 模块上的 #[cfg(test)] 标注:
// - 只有运行 cargo test 才编译和运行代码
// - 运行 cargo build 则不会
// 集成测试在不同的目录，它不需要 #[cfg(test)] 标注
// cfg：configuration（配置）
// - 告诉 RUst下面的条目只有在指定的配置选项下才被包含
// - 配置选项 test：由 Rust提供，用来编译和运行测试。
//   ·只有cargo test才会编译代码，包括模块中的 helper 函数和 #[test]标注的函数

// NOTE: 测试私有函数
pub fn adds_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod my_tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// WARNING: 如果项目中由单元测未通过，是不会执行 tests/ 文件夹下的集成测试的

// WARNING:
// 针对 binary crate 的集成测试
// 如果项目是 binary crate，只含有 src/main.rs 没有 src/lib.rs:
// - 不能在 tests 目录下创建集成测试
// - 无法把 main.rs 的函数导入作用域
// 只有 library crate 才能暴露函数给其它 crate 用
// binary crate 意味着独立运行
