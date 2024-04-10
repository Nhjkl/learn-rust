use std::fmt::Display;

use crate::utils;
// NOTE: 生命周期
// Rust的每个引用都有自己的生命周期。
// 生命周期：引用保持有效的作用域。
// 大多数情况：生命周期是隐式的、可被推断的
// 当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期。
pub fn run() {
    utils::color_pringln_green("Hello, lifetimes");
    use_lifetimes();
}

fn use_lifetimes() {
    // NOTE: 生命周期的主要目标： 避免悬垂引用(dangling reference)
    {
        // BUG:
        //{
        //    let r;                // ---------+-- 'a
        //                          //          |
        //    {                     //          |
        //        let x = 5;        // -+-- 'b  |
        //        r = &x;           //  |       |
        //    }                     // -+       |
        //                          //          |
        //    println!("r: {}", r); //          |
        //}                         // ---------+

        // TODO:
        // {
        //     let x = 5;            // ----------+-- 'b
        //                           //           |
        //     let r = &x;           // --+-- 'a  |
        //                           //   |       |
        //     println!("r: {}", r); //   |       |
        //                           // --+       |
        // }                         // ----------+
    }

    // NOTE: 函数中的泛型生命周期
    {
        // NOTE: 生命周期标注语法
        // 生命周期的标注不会改变引用的生命周期长度
        // 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
        // 生命周期的标注：描述了多个引用的生命周期间的关系，但不影响生命周期
        // 泛型生命周期参数声明在：函数名和参数列表之间的<>里
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        // ·生命周期‘α 的实际生命周期是：×和y两个生命周期中较小的那个
        //
        // {
        //     let string1 = String::from("abcd");
        //     let mut result;
        //     {
        //         let string2 = "xyz".to_string();
        //         result = longest(string1.as_str(), string2.as_str()); //  BUG: string2的生命周期不够长
        //     }
        //     // BUG: `string2` does not live long enough
        //
        //     println!("The longest string is {}", result); // BUG: borrow later used here
        // }

        // NOTE: 指定生命周期参数的方式依赖于函数所做的事情
        //从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配:
        {
            fn longest<'a>(x: &'a str, y: &str) -> &'a str {
                // 直接返回x，那么y的生命周期标注就不需要了
                x
            }
        }
        // NOTE: 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值:
        // - 这就是悬垂引用：该值在函数结束时就走出了作用域
        // {
        //     fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        //         let result = String::from("really long string");
        //         result.as_str() // BUG: cannot return reference of local variable `result`
        //     }
        // }
    }
    // NOTE:
    // 生命周期参数名：
    // - 以＇开头
    // - 通常全小写且非常短
    // - 很多人使用'a
    // 生命周期标注的位置：
    // - 在引用的&符号后
    // - 使用空格将标注和引用类型分开
    // &i32 // 是一个引用
    // &'a i32 // 是一个生命周期为'a的引用
    // &'a mut i32 // 是一个生命周期为'a的可变引用
    // 单个生命周期标注本身没有意义

    // NOTE: Struct定义中的生命周期标注
    // Struct里可包括：
    // - 自持有的类型
    // - 引用：需要在每个引用上添加生命周期标注
    // example:
    {
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            pub part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");

        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("i: {:?}", i);
    }
    // NOTE: 生命周期的省略我们知道：
    // - 每个引用都有生命周期
    // - 需要为使用生命周期的函数或 struct指定生命周期参数
    // 在Rust引用分析中所编入的模式称为*生命周期省略规则*
    // - 这些规则无需开发者来遵守
    // - 它们是一些特殊情况，由编译器来考虑
    // - 如果你的代码符合这些情况，那么就无需显式标注生命周期
    // 生命周期省略规则不会提供完整的推断：
    // - 如果应用规则后，引用的生命周期仍然模糊不清→编译错误
    // - 解决办法：添加生命周期标注，表明引用间的相互关系
    {
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

    // NOTE: 输入、车输出生命周期生命周期在:
    // - 函数/方法的参数：输入生命周期
    // - 函数/方法的返回值：输出生命周期

    // NOTE: 生命周期省略的三个规则
    // ·编译器使用3个规则在没有显式标注生命周期的情况下，来确定引用的生命周期
    // - 规则 1 应用于输入生命周期
    // - 规则 2、3 应用于输出生命周期－如果编译器应用完3个规则之后，仍然有无法确定生命周期的引用→报错
    // - 这些规则适用于 fn 定义和 impl 块
    // 规则1：每个引用类型的参数都有自己的生命周期
    // 规则2：如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
    // 规则3：如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self（是方法），
    // 那么self的生命周期会被赋给所有的输出生命周期参数
    // example: 1
    // fn first_word(s: &str) -> &str {
    // fn first_word<'a>(s: &'a str) -> &str { // 规则1
    // fn first_word<'a>(s: &'a str) -> &'a str { // 规则2
    // example: 2
    // fn longest(x: &str, y: &str) -> &str {
    // fn longest<'a>(x: &'a str, y: &'a str) -> &str { // 规则1
    // fn longest<'a>(x: &'a str, y: &'a str) -> &str { // 规则2, 3 规则无法确定输出生命周期

    // NOTE: 方法定义中的生命周期标注
    //·在struct上使用生命周期实现方法，语法和泛型参数的语法一样
    // 在哪声明和使用生命周期参数，依赖于:
    // - 生命周期参数是否和字段、方法的参数或返回值有关
    // struct字段的生命周期名:
    // - 在impl后声明
    // - 在struct名后使用
    // - 这些生命周期是 struct 类型的一部分
    // impl块内的方法签名中：
    // - 引用必须绑定于 struct字段引用的生命周期，或者引用是独立的也可以
    // - 生命周期省略规则经常使得方法中的生命周期标注不是必须的
    // example:
    {
        struct ImportantExcerpt<'a> {
            pub part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }

            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    // NOTE:静态生命周期
    // 'static是一个特殊的生命周期：整个程序的持续时间。
    // - 例如：所有的字符串字面值都拥有‘static 生命周期
    //      let s: &'static str = "I have a static lifetime.";
    // 为引用指定'static 生命周期前要三思：
    // - 是否需要引用在程序整个生命周期内都存活

    {
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        // NOTE: <'a, T> 泛型参数类型、Trait Bound、生命周期
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
