use core::fmt;
use std::{io::Error, ops::Add};

use colored::Colorize;

use crate::utils;

pub fn run() {
    utils::color_pringln_green("Hello, Advanced trait!");
    // NOTE: 在 trait 定义中使用关联类型来指定占位类型
    // 关联类型（associated type）是Trait中的一个占位符， 可以可用于Trait的方法签名中
    // - 可以定义出包含某些类型的Trait，而在实现前无需知道这些类型是什么
    {
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }

        pub trait Iterator2<T> {
            fn next(&mut self) -> Option<T>;
        }

        struct Counter {}

        // NOTE: 无需标注类型
        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

        // impl Iterator for Counter { // BUG: 无法为单个类型多次实现某个trait
        //     type Item = String;
        //
        //     fn next(&mut self) -> Option<Self::Item> {
        //         None
        //     }
        // }

        impl Iterator2<u32> for Counter {
            // NOTE: 需要标明泛型参数
            fn next(&mut self) -> Option<u32> {
                None
            }
        }
        // NOTE: 可以为一个类型多次实现某个trait（不同的泛型参数）
        impl Iterator2<String> for Counter {
            fn next(&mut self) -> Option<String> {
                None
            }
        }
    }
    // NOTE:默认泛型参数和运算符重载
    // 可以在使用泛型参数时为泛型指定一个默认的具体类型
    // 语法：<PlaceholderType=ConcreteType>
    // 这种技术常用于运算符重载（operator overloading）
    // Rust 不允许创建自己的运算符及重载任意的运算符
    // 但是可以通过实现std::ops 中列出的那些trait来重载一部分相应的运算符
    {
        println!("{}", "默认泛型参数和运算符重载".green());

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Self::Output {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        println!("p1+p2 = {:?}", p1 + p2);

        {
            #[derive(Debug, PartialEq)]
            struct Millimeters(u32);

            #[derive(Debug, PartialEq)]
            struct Meters(u32);

            impl Add<Meters> for Millimeters {
                type Output = Millimeters;

                fn add(self, rhs: Meters) -> Self::Output {
                    Millimeters(self.0 + rhs.0 * 1000)
                }
            }

            let mm = Millimeters(1);
            let m = Meters(1);

            println!("mm+m = {:?}", mm + m);
            // println!("m+mm = {:?}", m + mm); BUG: Meters 没有实现 Add<Millimeters>
        }
    }

    // NOTE: 默认泛型参数的主要应用场景
    // 扩展一个类型而不破坏现有代码
    // 允许在大部分用户都不需要的特定场景下进行自定义

    // NOTE: 完全限定语法（Fully Qualified Syntax）如何调用同名方法
    {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        person.fly();
        Pilot::fly(&person); // 完全限定语法（Fully Qualified Syntax）如何调用同名方法
        Wizard::fly(&person);
    }

    {
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());
        // println!("A baby dog is called a {}", Animal::baby_name()); BUG: 无法确定类型
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        // NOTE: 完全限定语法：<Type as Trait>::function(receiver_if_method, next_arg, ...);
        // - 可以在任何调用函数或方法的地方使用
        // - 允许忽略那些从上下问推倒出来的部分
        // - 当 Rust 无法区分你期望调用哪个具体实现的时候，在需要使用这种语法
    }

    // NOTE:使用 supertrait 来要求 trait 附带其它 trait 的功能
    // 需要在一个 trait 中使用其它 trait 的功能:
    // - 需要被依赖的 trait 也被实现
    // - 那个被间接依赖的 trait 就是当前 trait 的 supertrait
    {
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl OutlinePrint for Point {}

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}]", self.x, self.y)
            }
        }

        let point = Point { x: 1, y: 2 };

        point.outline_print()
    }

    // NOTE: 使用newtype 模式在外部类型上实现外部trait
    // 孤儿规则：只有当 trait 或类型定义在本地包时，才能为该类型实现这个 trait
    // 可以通过 newtype 模式来绕过这一规则
    // - 利用 tuple struct（元组结构体）创建一个新的类型
    {
        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "@@@ {} @@@", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }

    // NOTE: 高级类型
    // 使用newtype 模式实现类型安全和抽象
    // newtype 模式可以:
    // - 用来静态的保证各种值之间不会混淆并表明值的单位
    // - 为类型的某些细节提供抽象能力－通过轻量级的封装来隐藏内部实现细节
    // 使用类型别名创建类型同义词
    // Rust提供了类型别名的功能：
    // - 为现有类型生产另外的名称尔（同义词）
    // - 并不是一个独立的类型
    // - 使用 type 关键字
    // 主要用途：减少代码字符重复
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
    }
    {
        // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
        // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        //     Box::new(|| println!("hi"))
        // }
        // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

        // NOTE: 使用类型别名优化👆代码
        type Thunk = Box<dyn Fn() + Send + 'static>;

        fn takes_long_type(f: Thunk) {}
        fn returns_long_type() -> Thunk {
            Box::new(|| println!("hi"))
        }
        let f: Thunk = Box::new(|| println!("hi"));

        // pub trait Write {
        //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        //     fn flush(&mut self) -> Result<(), Error>;
        //
        //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
        // }

        // NOTE: 使用类型别名优化👆代码
        type Result<T> = std::result::Result<T, std::io::Error>;
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }

    // NOTE: Never 类型
    // 有一个名为！的特殊类型：
    // - 它没有任何值，行话称为空类型（empty type）
    // - 我们倾向于叫它 never类型，因为它在不返回的函数中充当返回类型
    // 不返回值的函数也被称作发散函数（diverging function)
    {
        // fn bar() -> ! {}

        let guess = "42";

        // loop {
        //     let guess: u32 = match guess.trim().parse() {
        //         Ok(num) => num,
        //         Err(_) => continue, // NOTE: 这里continue的类型是Never 也就是! 可以被强制的转换为任何类型
        //     };
        // }

        // NOTE: never type 的另一个用途是 panic!
        // Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。
        // 这能工作是因为 panic! 并不产生一个值；它会终止程序。对于 None 的情况，unwrap 并不返回一个值，所以这些代码有效。
        //
        // impl<T> Option<T> {
        //     pub fn unwrap(self) -> T {
        //         match self {
        //             Some(val) => val,
        //             None => panic!("called `Option::unwrap()` on a `None` value"),
        //         }
        //     }
        // }

        // NOTE: 最后一个有着 ! 类型的表达式是 loop：
        // 这里，循环永远也不结束，所以此表达式的值是 !。但是如果引入 break 这就不为真了，因为循环在执行到 break 后就会终止。

        // print!("forever ");
        //
        // loop {
        //     print!("and ever ");
        // }
    }

    // NOTE: 动态大小和 Sized Trait
    // Rust需要在编译时确定为一个特定类型的值分配多少空间。
    // 动态大小的类型型（Dynamically Sized Types，DST）的概念:
    // - 编写代码时使用只有在运行时才能确定大小的值
    // str是动态大小的类型（注意不是&str）：只有运行时才能确定字符串的长度
    // - 下列代码无法正常工作：
    // let sl: str = "Hello there!";
    // let s2: str = "How's it going?";
    // - 使用 &str 来解决:
    //   str 的地址
    //   str 的长度

    // NOTE: Rust使用动态大小类型的通用方式
    // 附带一些额外的元数据来存储动态信息的大小
    // - 使用动态大小类型时总会把它的值放在某种指针后边

    // NOTE: 另外一种动态大小的类型：trait
    // 每个trαit 都是一个动态大小的类型，可以通过名称对其进行引用
    // 为了将 trait 用作 trait对象，必须将它放置在某种指针之后
    //  - 例如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait>）之后

    // NOTE: Sized trait
    // 为了处理动态大小的类型，Rust提供了一个 Sized trait 来确定一个类型的大小在编译时是否已知
    // - 编译时可计算出大小的类型会自动实现这一trait
    // - Rust 还会为每一个泛型函数隐式的添加 Sized 约束
    // 默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法解除这一限制
    {
        fn generic<T>(t: T) {}
        // 👆这行代码会被隐式转换成👇
        fn generic2<T: Sized>(t: T) {}
        // ?Sized trait 约束
        fn generic3<T: ?Sized>(t: &T) {}
        // NOTE: ?只能用于Sized前面，T 由于时不确定大小的所以只能使用指针 &T
        // T可能是也可能不是Sized
        // 这个语法只能用在 Sized上面，不能被用于其它trait
    }

    // NOTE: 函数指针
    // 可以将函数传递给其它函数
    // 函数在传递过程中会被强制转换成 fn 类型
    // fn 类型就是“函数指针(function pointer)"
    {
        utils::color_pringln_green("函数指针");

        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }

    // NOTE: 函数指针与闭包的不同
    // fn是一个类型，不是一个trait
    //  - 可以直接指定fn为参数类型，不用声明一个以Fn trait为约束的泛型参数
    // 函数指针实现了全部3 种闭包trait（Fn，FnMut，FnOnce）：
    // - 总是可以把函数指针用作参数传递给一个接收闭包的函数
    // - 所以，倾向于搭配闭包trait的泛型来编写函数：可以同时接收闭包和普通函数
    // 某些情景，只想接收fn 而不接收闭包:
    // - 与外部不支持闭包的代码交互：C函数
    {
        let numbers = vec![1, 2, 3];
        let strings: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
        // NOTE:                                        👆👇 闭包和普通函数都可以支持
        let numbers = vec![1, 2, 3];
        let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();

        enum Status {
            Value(u32),
            Stop,
        }

        let v = Status::Value(3); // 也是一个函数

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    // NOTE: 返回闭包
    // 闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了该trait 的具体类型作为返回值。
    {
        // fn returns_closure() -> Fn(i32) -> i32 { // BUG: 这种情况返回闭包无法确定其大小
        //     |x| x + 1
        // }
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
