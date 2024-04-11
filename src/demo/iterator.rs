use crate::utils;
// NOTE: 什么是迭代器
// ·迭代器模式：对一系列项执行某些任务
// ·迭代器负责：
// - 遍历每个项
// - 确定序列（遍历）何时完成
// Rust 的迭代器：
// - 懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果。

pub fn run() {
    utils::color_pringln_green("Hello, iterator!");

    {
        let mut v1 = vec![1, 2, 3];

        v1.push(4);

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("{:?}", val);
        }
    }

    // NOTE: Iterator trait 和 next 方法
    {
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;

            // 此处省略了方法的默认实现
        }

        let mut v1 = [1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        // NOTE: 几个选代方法
        // iter方法：在不可变引用上创建迭代器
        // into_iter方法：创建的迭代器会获得所有权
        // iter_mut方法：迭代可变的引用
    }

    // NOTE: 消耗迭代器的方法
    // 在标准库中，Iterator trait 有一些带默认实现的方法
    // 其中有一些方法会调用 next
    // 方法－ 实现 Iterator trait 时必须实现 next 方法的原因之一
    // 调用 next的方法叫做“消耗型适配器
    // - 因为调用它们会把迭代器消耗尽
    // 例如：sum 方法（就会耗尽迭代器）
    // - 取得迭代器的所有权
    // - 通过反复调用 next，遍历所有元素
    // - 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
    {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        println!("total: {:?}", total);
    }
    // NOTE:
    // 产生其它迭代器的方法
    // 定义在Iterator trait 上的另外一些方法叫做“迭代器适配器"
    // - 把迭代器转换为不同种类的迭代器
    // 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性较高。
    // 例如：map
    // - 接收一个闭包，闭包作用于每个元素
    // - 产生一个新的迭代器
    // - collect 方法: 消耗型适配器，把结果搜集到一个集合类型中
    {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();
        let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
        println!("v2: {:?}", v2);
    }

    // NOTE: 迭代器使用闭包来捕获环境
    // filter 方法：
    // - 接收一个闭包
    // - 这个闭包在遍历迭代器的每个元素时，返回bool类型
    // - 如果闭包返回 true：当前元素将会包含在 filter 产生的迭代器中
    // - 如果闭包返回false：当前元素将不会包含在 filter 产生的迭代器中
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == size).collect()
        }

        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 11,
                style: String::from("boot"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        println!("in_my_size: {:?}", in_my_size);
    }

    // NOTE: 使用Iterator trait 来创建自定义迭代器
    {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut c = Counter::new();

        // for i in c {
        //     println!("i: {:?}", i);
        // }
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), Some(4));
        assert_eq!(c.next(), Some(5));
        assert_eq!(c.next(), None);

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);

        println!("sum: {:?}", sum);
    }

    // NOTE: 总结 https://rustwiki.org/zh-CN/book/ch13-04-performance.html
    // 闭包和迭代器是 Rust 受函数式编程语言观念所启发的功能。
    // 他们对 Rust 以底层的性能来明确的表达高级概念的能力有很大贡献。
    // 闭包和迭代器的实现达到了不影响运行时性能的程度。
    // 这正是 Rust 竭力提供零成本抽象的目标的一部分。
}
