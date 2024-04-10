use std::fmt::{Debug, Display};

use crate::utils;

// NOTE: Trait
// 1. Trait 告诉 Rust 编译器：
//      - 某种类型具有哪些并且可以与其它类型共享的功能
// 2. Trait：抽象的定义共享行为
// 3. Trait bounds（约束）：泛型类型参数指定为实现了特定行为的类型
// 4. Trait与其它语言的接口（interface）类似，但有些区别。

pub fn run() {
    utils::color_pringln_green("Hello, trait");
    define_trait();
}

/// NOTE: 定义一个trait
/// Trait的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为。
/// - 关键字：trait
/// - 只有方法签名，没有具体实现
/// - trait可以有多个方法：每个方法签名占一行，以;结尾
/// - 实现该 trait的类型必须提供具体的方法实现
pub trait Summary {
    fn summarize_author(&self) -> String;

    // NOTE: 默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现
    }
}

// NewsArticle
// Tweet
// NOTE: 在类型上实现 trait
// 与为类型实现方法类似。
// 不同之处：
//                      👇指的是类
// - impl Summary for Tweet { ... }
//          👆指的是 trait
// - 在impl 的块里，需要对 Trait 里的方法签名进行具体的实现(不是绝对)
// 实现 trait 的约束
// 可以在某个类型上实现某个 trait 的前提条件是：
//      - 这个类型 或这个 trait 是在本地 crate 里定义的
// 无法为外部类型来实现外部的 trait:
//      - 这个限制是程序属性的一部分（也就是*一致性*）。
//      - 更具体地说是*孤儿规则*：之所以这样命名是因为父类型不存在。
//      - 此规则确保其他人的代码不能破坏您的代码，反之亦然。
//      - 如果没有这个规则，两个 crate 可以为同一类型实现同一个 trait，
//        Rust 就不知道应该使用哪个实现了。
// 默认实现
// 默认实现的方法可以调用trait中其它的方法，即使这些方法没有默认实现
// 请注意，无法从相同方法的重载实现中调用默认方法。

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

fn define_trait() {
    utils::color_pringln_green("define_trait-----------------");

    let news = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    notify(news);

    let tweet = Tweet {
        username: String::from("tweet username"),
        content: String::from("tweet content"),
        reply: false,
        retweet: true,
    };

    notify(tweet);

    // NOTE: trait 也可以作为参数

    // NOTE: impl Trait 语法：适用于简单情况

    // fn notify(item: impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // NOTE: Trait bound语法：可用于复杂情况
    // - impl Trait 语法是Trait bound 的语法糖
    fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    // NOTE: 使用+ 指定多个 Trait bound
    fn notify1(item: impl Summary + Display) {
        println!("Breaking news! {}", item.summarize());
    }
    fn notify2<T: Summary + Display>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    {
        fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) {
            println!("Breaking news! {}", a.summarize());
        }
    }
    // NOTE: Trait bound使用where子句优化👆代码, 在方法签名后面使用where子句
    {
        fn notify<T, U>(a: T, b: U)
        where
            T: Summary + Display,
            U: Clone + Debug,
        {
            println!("Breaking news! {}", a.summarize());
        }
    }

    // NOTE: 使用Trait 作为返回类型
    {
        fn notify() -> impl Summary {
            NewsArticle {
                headline: String::from("headline"),
                location: String::from("location"),
                author: String::from("author"),
                content: String::from("content"),
            }
        }

        // FIX: 注意：impl Trait只能返回确定的同一种类型，返回可能不同类型的代码会报错
        //
        // fn returns_summarizable(switch: bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from("Penguins win the Stanley Cup Championship!"),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best
        //     hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Tweet {
        //             username: String::from("horse_ebooks"),
        //             content: String::from("of course, as you probably already know, people"),
        //             reply: false,
        //             retweet: false,
        //         }
        //     }
        // }
    }
    // NOTE: 使用 Trait Bound有条件的实现方法
    // 在使用泛型类型参数的impl块上使用 Trait bound，我们可以有条件的为实现了特定 Trait 的类型来实现方法
    // 也可以为实现了其它Trait的任意类型有条件的实现某个Trait
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            // NOTE: 只用T类型实现了Display和PartialOrd trait, 才会有cmp_disply方法
            fn cmp_disply(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }

        let p = Pair::new(5, 10);
        p.cmp_disply();
        let p1 = Pair::new((1, 2), (1, 2));
        // p1.cmp_disply();//  FIX: (1,2) 没有实现PartialOrd trait 所以p1 没有cmp_disply方法

        // NOTE: 为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现（blanketmplementafions)
        // 只要实现了Display trait的类型都可以实现ToString trait
        // impl<T: Display> ToString for T {
        //     // --snip--
        // }
    }
}
