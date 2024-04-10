use crate::utils;
pub fn run() {
    utils::color_pringln_green("hello String!");
    create_string();
    update_string();
    format_string();
    index_string();
    len_string();
    split_string();
}

// NOTE: 字符串是Byte的集合，提供一些方法，能将byte解析为文本
// Rust 的核心语言层面，只有一个字符串类型： 字符串切片 str （或&str）
// 字符串切片：对存储在其它地方、UTF-8编码的字符串的引用－字符串字面值：存储在二进制文件中，也是字符串切片
// String 类型：
// －来自 标准库而不是核心语言
// －可增长、可修改、可拥有

/// NOTE: create String
fn create_string() {
    utils::color_pringln_green("create_string----------------------");
    let mut s = String::new();
    s.push('h');
    println!("{}", s);

    let s1 = "Hello, World!".to_string(); // NOTE: 使用to_string方法,将&str转换为String
    println!("{}", s1);

    let s2 = String::from("Hello, World!"); // NOTE: 使用from方法从字面值创建String
    println!("{}", s2);

    // NOTE: 请记住，字符串是 UTF-8 编码的，所以可以包含任何正确编码的数据
    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }
}

/// NOTE: updating String
fn update_string() {
    utils::color_pringln_green("update_string----------------------");
    let mut s1 = String::from("Hello, ");
    s1.push_str("World!"); // NOTE: push_str()在字符串后追加字符串切片
    let s2 = String::from(" 😊");
    s1.push_str(&s2); // NOTE: 为什么&String可以使用?

    s1.push('😅'); // NOTE: push()在字符串后追加单个byte

    println!("{}", s1);
    println!("{}", s2);

    let s3 = s1 + &s2;
    // NOTE: + 操作符用于字符串和字符串切片
    // 执行完这些代码之后，字符串 s3 将会包含 Hello, world!。
    // s1 在相加后不再有效的原因，和使用 s2 的引用的原因，
    // 与使用 + 运算符时调用的函数签名有关。+ 运算符使用了 add 函数，
    // 这个函数签名看起来像这样：
    // fn add(self, s: &str) -> String {

    println!("s3 = {}", s3);
    // println!("{}", s1); // NOTE: 在 + 操作符左侧，会取得 s1 的所有权
}

/// NOTE: 使用format!宏,拼接字符串
fn format_string() {
    utils::color_pringln_green("format_string----------------------");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s4 = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s1); // borrow of moved value: `s1`

    let s4 = format!("{}-{}-{}", s1, s2, s3); // NOTE: 不会移动所有权
    println!("{}", s4);
    println!("{}", s1);
}

/// NOTE: 不支持按索引访问
fn index_string() {
    utils::color_pringln_green("index_string----------------------");
    let s1 = String::from("hello");
    // let c = s1[0]; // NOTE: 不支持
}

/// NOTE: String len
fn len_string() {
    utils::color_pringln_green("len_string----------------------");
    let s = String::from("hello");
    println!("{}", s.len());

    let emoji = String::from("🤗");
    let len = emoji.len();
    println!("'🤗'.len = {}", len); // 4

    utils::color_pringln_green("以字节的形式迭代----------------------");
    for c in emoji.bytes() {
        println!("{}", c);
    }

    utils::color_pringln_green("以标量值的形式迭代----------------------");
    for c in emoji.chars() {
        println!("{}", c);
    }

    // TODO: 以字形簇的形式迭代, 比较复杂标准库未提供，可以找第三方库实现
}

/// NOTE: 字符串切割
/// 可以使用[]和一个范围来创建字符串切片
/// - 必须谨慎使用
/// - 如何切片时跨越了字符边界，程序会panic
fn split_string() {
    utils::color_pringln_green("split_string----------------------");
    let hello = "Здравствуйте";
    let s = &hello[..4];
    // let s = &hello[..3]; // NOTE: byte index 3 is not a char boundary, 该语言一个字母站在两个字节, 0..3会导致编译错误
    println!("{}", s);
}
