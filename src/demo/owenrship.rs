use crate::utils;

pub fn run() {
    utils::color_pringln_green("Hello, owenrship");
    utils::color_pringln_green("Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。");
    utils::color_pringln_green("值在任一时刻有且只有一个所有者。");
    utils::color_pringln_green("当所有者（变量）离开作用域，这个值将被丢弃。");

    string_type();
    memory_and_allocation();
    ownership_and_functions();
    return_values_and_scope();
    takes_ownership_and_return_tuple();
    return_reference();
    dangling_references();
    slice();
}

/// NOTE: 使用String类型来表示字符串，因为它是在堆上分配的。
fn string_type() {
    utils::color_pringln_green("string type --------------");
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("s: {}", s);
}

/// NOTE: Memory and Allocation 内存和分配
fn memory_and_allocation() {
    utils::color_pringln_green("memory and allocation --------------");
    let s1 = String::from("hello");
    let s2 = s1;
    // NOTE: clone 会复制整个 s2，而不仅仅是 s2 的引用，这种操作比较浪费资源。
    let s3 = s2.clone();
    // println!("s1: {}", s1); // NOTE: 会报错，因为 s1 已经被移动到 s2 中了
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let x = 5;
    let y = x;
    // NOTE: “将值 5 绑定到 x ；然后复制 x 中的值并将其绑定到 y 。”我们现在有两个变量 x 和 y ，并且都等于 5 。
    // 这确实是正在发生的事情，因为整数是具有已知固定大小的简单值，并且这两个 5 值被推入堆栈。
    println!("x: {}, y: {}", x, y);
}

// NOTE: Rust 有一个叫做 Copy trait 的特殊标注，可以用在类似整型这样的存储在栈上的类型上（第 10 章详细讲解 trait）。
// 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了
// Drop trait 的类型使用 Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 标注，将会出现一个编译时错误。
// 要学习如何为你的类型添加 Copy 标注以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。
// 如下是一些 Copy 的类型：
// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

/// NOTE: Ownership and Functions 所有权和函数
fn ownership_and_functions() {
    utils::color_pringln_green("ownership and functions --------------");
    let s = String::from("hello");

    takes_ownership(s);

    // println!("s: {}", s); // NOTE: 会报错，因为 s 已经被移动到 takes_ownership 中了

    let x = 5;

    makes_copy(x);

    println!("x: {}", x); // NOTE: 正常，因为 x 仍然被复制到函数中

    fn takes_ownership(some_string: String) {
        println!("some_string: {}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("some_integer: {}", some_integer);
    }
}

/// NOTE: Return Values and Scope 返回值和作用域
fn return_values_and_scope() {
    utils::color_pringln_green("return values and scope --------------");
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
                                       // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
                                       // 所以什么也不会发生。s1 移出作用域并被丢弃
    println!("s1: {}", s1);
    // println!("s2: {}", s2); // NOTE: 会报错，因为 s2 已经被移动到 takes_and_gives_back 中了
    println!("s3: {}", s3);
    fn gives_ownership() -> String {
        // gives_ownership 将返回值移动给
        // 调用它的函数

        let some_string: String = String::from("yours"); // some_string 进入作用域

        some_string // 返回 some_string 并移出给调用的函数
    }

    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string 进入作用域

        a_string // 返回 a_string 并移出给调用的函数
    }
}

// NOTE: 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，
// 其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

/// NOTE: 让函数使用某个值，但不取得其所有权, 返回元组
fn takes_ownership_and_return_tuple() {
    utils::color_pringln_green("takes ownership and return tuple --------------");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // println!("s1: {}", s1);
    println!("The length of '{}' is {}.", s2, len);
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }
}

/// NOTE: 通过引用来返回值
fn return_reference() {
    utils::color_pringln_green("return reference --------------");
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // NOTE: &表示引用，允许你借用 s1，而不获取所有权
    println!("The length of '{}' is {}.", s1, len);
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    {
        let mut s1 = String::from("hello");
        let len = calculate_length(&mut s1); // NOTE: 把引用传递给函数,称之为借用
        println!("The length of '{}' is {}.", s1, len);
        fn calculate_length(s: &mut String) -> usize {
            // NOTE: 通过使用 &mut 语法，我们可以更改变量的值
            s.push_str(", world");
            s.len()
        }
    }

    {
        let mut s = String::from("hello");
        let s1 = &mut s;
        // let s2 = &mut s; // NOTE: cannot borrow `s` as mutable more than once at a time, 不可以在同一时刻多次引用
        // println!("s1: {}, s2: {}", s1, s2);
        // NOTE: 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它由这三个行为造成：
        // 两个或更多指针同时访问同一数据。
        // 至少有一个指针被用来写入数据。
        // 没有同步数据访问的机制。
        {
            // NOTE: 这里的代码块，不会对数据竞争造成影响
            let s2 = &mut s;
        }

        {
            let mut s = String::from("hello");
            let s1 = &s;
            let s2 = &s;
            let s3 = &s;
            // let s4 = &mut s; // NOTE: 一个不可变引用和一个可变引用同时存在
            println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // NOTE: 允许多个不可变引用
        }
    }
}

/// NOTE: Dangling References 悬空引用
fn dangling_references() {
    utils::color_pringln_green("dangling references --------------");
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing={}", reference_to_nothing);

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // NOTE: s在离开作用域后就被销毁了，所以这里返回的引用是不可用的, rust 会报错
    // }

    fn no_dangle() -> String {
        let s: String = String::from("hello");
        s
    }
}

// NOTE: 引用的规则
// 让我们概括一下之前对引用的讨论：
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。

// NOTE: 切片 slice
fn slice() {
    utils::color_pringln_green("slice --------------");
    {
        let mut s = String::from("hello world");
        let word_index = first_word(&s);
        s.clear();
        println!("word_index: {}", word_index); // NOTE: 当s.clear()时，word_index 仍旧是5，可使用切片来解决
    }

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    // NOTE: 字符串 slice（string slice）
    {
        utils::color_pringln_green("string slice --------------");

        let s = String::from("hello world");

        // let hello = &s[0..5];
        // let world = &s[6..11];
        let hello = &s[..5]; // 等同于 0..5
        let world = &s[6..]; // 等同于 6..11
        let word = &s[..]; // 等同于 0..s.len()
        println!("{} {}", hello, world);
        // NOTE: 注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
        // 如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
        // 出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；
        // 第 8 章的“使用字符串存储 UTF-8 编码的文本”部分会更加全面的讨论 UTF-8 处理问题。

        let mut s = String::from("hello world");
        let word_index = first_word(&s[..]);
        // s.clear(); //NOTE: 不可以把s借用成可变，因为它是不可变的
        println!("word_index: {}", word_index);
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }
            s
        }
    }

    // NOTE: 数组切片
    {
        utils::color_pringln_green("array slice --------------");
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("{:?}", slice);
    }
}
