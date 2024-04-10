use crate::utils;
pub fn run() {
    utils::color_pringln_green("hello, vector!");
    create_vector();
    update_vector();
    delete_vector();
    traverse_vector();
    vector_example();
}

// NOTE: 使用Vector存储多个值
// Vec<T>，叫做Vector
// 由标准库提供
// 可存储多个值
// 只能存储相同类型的数据
// 值在内存中连续存放

/// NOTE: create vector
fn create_vector() {
    utils::color_pringln_green("create_vector----------------");
    let v: Vec<i32> = Vec::new(); // NOTE: 用Vec::new创建
    println!("v = {:?}", v);
    let v = vec![1, 2, 3]; // NOTE: 用vec!宏创建
    println!("v = {:?}", v);
}

/// NOTE: update vector
fn update_vector() {
    utils::color_pringln_green("update_vector----------------");
    let mut v = Vec::new();
    if v.is_empty() {
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
    }
    println!("v = {:?}", v);
}

/// NOTE: delete vector
/// 当Vector离开作用域后
/// 它就被清理掉
/// 它所有的元素也被清理掉
fn delete_vector() {
    utils::color_pringln_green("delete_vector----------------");
    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    // let third = &v[100]; // NOTE: panic index out of bounds
    let third = &v[2];
    // NOTE: 因为在👆 &v[2] 已经借用为不可变的应用 ，所以不可以修改
    // v.push(4);
    println!("The third element is {}", third);

    // NOTE: get 返回Option
    match v.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

/// NOTE: 遍历vector
fn traverse_vector() {
    utils::color_pringln_green("traverse_vector----------------");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // NOTE: 修改vector的值
    {
        utils::color_pringln_green("modify_vector----------------");
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50; // NOTE: *i 解引用
        }

        for i in &v {
            println!("{}", i);
        }
    }
}

/// NOTE: vector 例子
fn vector_example() {
    utils::color_pringln_green("vector_example----------------");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
