use crate::utils;

pub fn run() {
    utils::color_pringln_green("Rust 的面向对象编程特性");

    {
        pub struct AveragedCollection {
            list: Vec<i32>,
            average: f64,
        }

        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    }
                    None => None,
                }
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }
    // NOTE: 继承
    // ·继承：使对象可以沿用另外一个对象的数据和行为，且无需重复定义相关代码
    // Rust：没有继承·使用继承的原因:
    // - 代码复用·Rust：默认 trait 方法来进行代码共享
    // - 多态
    // Rust：泛型和trait约束（限定参数化多态boundedparametric）

    // NOTE: 使用trait对象来存储不同类型的值

    // NOTE: 有这样一个需求
    // 创建一个GUI工具：
    // - 它会遍历某个元素的列表，依次调用元素的draw方法进行绘制
    // - 例如：Button、TextField 等元素在面向对象语言里：
    // - 定义一个Component父类，里面定义了draw方法
    // - 定义 Button、TextField 等类，继承与 Component 类
    // 为共有行为定义一个trait
    // Rust避免将struct或enum 称为对象，因为它们与impl块是分开的
    // trait对象有些类似于其它语言中的对象：
    // - 它们某种程度上组合了数据与行为trait对象与传统对象不同的地方：
    // - 无法为 trait对象添加数据
    // trait对象被专门用于抽象某些共有行为，它没其它语言中的对象那么通用

    {
        trait Draw {
            fn draw(&self);
        }

        struct Screen {
            pub components: Vec<Box<dyn Draw>>, // NOTE: 表示只要实现了Draw trait就可以
        }

        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        #[derive(Debug)]
        struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }

        impl Button {
            fn new(label: &str) -> Button {
                Button {
                    width: 100,
                    height: 100,
                    label: label.to_string(),
                }
            }
        }

        impl Draw for Button {
            fn draw(&self) {
                println!("Draw Button: {:?}", self);
            }
        }

        #[derive(Debug)]
        struct SelectBox {
            pub width: u32,
            pub height: u32,
            pub options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                println!("Draw SelectBox: {:?}", self);
            }
        }

        {
            let button = Button::new("OK");
            let select_box = SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("yes"), String::from("no")],
            };
            let screen = Screen {
                components: vec![Box::new(button), Box::new(select_box)],
            };
            screen.run();
        }

        // NOTE: Trait对象执行的是动态派发
        // ·将 trait 约束作用于泛型时，Rust编译器会执行单态化：
        // - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
        // 通过单态化生成的代码会执行静态派发（static dispatch），在编译过程中确定调用的具体方
        // 法动态派发（dynamic dispatch）:
        // - 无法在编译过程中确定你调用的究竟是哪一种方法
        // - 编译器会产生额外的代码以便在运行时找出希望调用的方法
        // 使用 trait 对象，会执行动态派发：
        // - 产生运行时开销
        // - 阻止编译器内联方法代码，使得部分优化操作无法进行
        // Trait对象必须保证对象安全
        // 只能把满足对象安全（object-safe）的 trait 转化为 trait 对象
        // Rust采用一系列规则来判定某个对象是否安全，只需记住两条：
        // - 方法的返回类型不是Self
        // - 方法中不包含任何泛型类型参数
        {
            pub trait Clone {
                fn clone(&self) -> Self;
            }

            pub struct Screen {
                // pub components: Vec<Box<dyn Clone>>, // BUG: object-safe, 不允许返回Self
            }
        }
    }

    // NOTE: 状态模式
    // 状态模式（state pattern）是一种面向对象设计模式:
    // - 一个值拥有的内部状态由数个状态对象（state object）表达而成，而值的行为则随着内部状态的改变而改变
    // 使用状态模式意味着：
    // - 业务需求变化时，不需要修改持有状态的值的代码，或者使用这个值的代码
    // - 只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象
    {
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        pub struct PendingReviewPost {
            content: String,
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        impl Post {
            fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            let post = post.request_review();

            let post = post.approve();

            assert_eq!("I ate a salad for lunch today", post.content());
            println!("post: {}", post.content());
        }
    }
}
