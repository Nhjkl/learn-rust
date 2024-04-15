use core::fmt;

// https://rustwiki.org/zh-CN/rust-by-example/hello/print.html
pub fn run() {
    println!("{} days", 31);

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    // 改正 ^ 补上漏掉的参数："James"
    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。

    // NOTE: Debug
    #[derive(Debug)]
    struct UnPrintable(i32);
    println!("{:?} unwraps to {:?}", UnPrintable(2), 2);

    // NOTE: Display
    {
        // 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
        // `Structure`，包含一个 `i32` 元素。
        struct Structure(i32);

        // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
        impl fmt::Display for Structure {
            // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
                // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
                write!(f, "{}", self.0)
            }
        }

        let s = Structure(10);
        println!("{}", s);
    }

    {
        // 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
        #[derive(Debug)]
        struct MinMax(i64, i64);

        // 实现 `MinMax` 的 `Display`。
        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 使用 `self.number` 来表示各个数据。
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        // 为了比较，定义一个含有具名字段的结构体。
        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        // 类似地对 `Point2D` 实现 `Display`
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 自定义格式，使得仅显示 `x` 和 `y` 的值。
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }

        let minmax = MinMax(0, 14);

        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!(
            "The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range
        );

        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
        // 得到实现。这语句不能运行。
        // println!("What does Point2D look like in binary: {:b}?", point);
    }

    {
        struct Point3D {
            x: f64,
            y: f64,
            z: f64,
        }

        impl fmt::Display for Point3D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Self { x, y, z } = self;
                write!(f, "{{ x: {}, y: {}, z: {} }}", x, y, z)
            }
        }

        let point = Point3D {
            x: 3.3,
            y: 7.2,
            z: 10.5,
        };

        println!("Compare points: {}", point);
    }

    // List
    {
        #[derive(Debug)]
        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let vec = &self.0;
                write!(f, "List([")?;

                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", count, v)?;
                }

                write!(f, "])")
            }
        }

        let v = List(vec![1, 2, 3]);

        println!("{}", v.0[2]);
        println!("{}", v);
        // println!("{:?}", v);
    }

    // Format
    {
        struct City {
            name: &'static str,
            lat: f32,
            lon: f32,
        }

        impl fmt::Display for City {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
                let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

                // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
                // 一个缓冲区（即第一个参数f）中。
                write!(
                    f,
                    "{}: {:.3}°{} {:.3}°{}",
                    self.name,
                    self.lat.abs(),
                    lat_c,
                    self.lon.abs(),
                    lon_c
                )
            }
        }

        for city in [
            City {
                name: "Dublin",
                lat: 53.347778,
                lon: -6.259722,
            },
            City {
                name: "Oslo",
                lat: 59.95,
                lon: 10.75,
            },
            City {
                name: "Vancouver",
                lat: 49.25,
                lon: -123.1,
            },
        ]
        .iter()
        {
            println!("{}", *city);
        }
    }
}
