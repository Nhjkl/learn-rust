use crate::utils;

// NOTE: 宏macro
// 宏在 Rust里指的是一组相关特性的集合称谓：
// - 使用macro_rules!构建的声明宏（declarative macro）
// - 3 种过程宏
//   - 自定义 #[derive]宏，用于 struct或 enum，可以为其指定随 derive 属性添加的代码
//   - 类似属性的宏，在任何条目上添加自定义属性
//   - 类似函数的宏，看起来像函数调用，对其指定为参数的token 进行操作
// 函数与宏的差别
// 本质上，宏是用来编写可以生成其它代码的代码（元编程，metaprogramming）
// 函数在定义签名时，必须声明参数的个数和类型，宏可处理可变的参数
// 编译器会在解释代码前展开宏宏的定义比函数复杂得多，难以阅读、理解、维护
// 在某个文件调用宏时，必须提前定义宏或将宏引入当前作用域；
// 函数可以在任何位置定义并在任何位置使用
pub fn run() {
    utils::color_pringln_green("Hello, macro!");
    // NOTE: macro_rules!声明宏
    // Rust中最常见的宏形式：声明宏
    // - 类似match的模式匹配
    // - 需要使用marco_rules!
}

#[macro_export] // NOTE: 标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域。
macro_rules! vec { // 接着使用 macro_rules! 和宏名称开始宏定义，且所定义的宏并 不带 感叹号。名字后跟大括号表示宏定义体，在该例中宏名称是 vec
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// NOTE: 基于属性来生成代码的过程宏
// 这种形式更像函数（某种形式的过程）一些
// - 接收并操作输入的 Rust 代码
// - 生成另外一些Rust代码作为结果
// 三种过程宏：
// - 自定义派生
// - 属性宏
// - 函数宏
// 创建过程宏时：
// - 宏定义必须单独放在它们自己的包中，并使用特殊的包类型

// use proc_macro;
//
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

// NOTE: 类似属性的宏
// 属性宏与自定义 derive 宏类似
// - 允许创建新的属性
// - 但不是为derive属性生成代码
// 属性宏更加灵活：
// - derive只能用于struct和enum
// - 属性宏可以用于任意条目，例如函数

// #[route(GET, "/")]
// fn index() {

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// NOTE: 类似函数的宏
// 函数宏定义类似于函数调用的宏，但比普通函数更加灵活
// 函数宏可以接收TokenStream作为参数
// 与另外两种过程宏一样，在定义中使用 Rust代码来操作TokenStream
//
// let sql = sql!(SELECT * FROM posts WHERE id=1);
//
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
