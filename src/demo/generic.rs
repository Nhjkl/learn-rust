use crate::utils;

pub fn run() {
    utils::color_pringln_green("Hello, generic");
    use_generic();
    generic_struct();
    generic_enum();
    generic_method();
}

/// NOTE:  使用泛型
fn use_generic() {
    utils::color_pringln_green("use generic--------------------");

    // fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];
    //     for &item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = [
        3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
    ];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// NOTE: struct 中使用泛型
fn generic_struct() {
    utils::color_pringln_green("generic struct--------------------");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    // let integer = Point { x: 5, y: 10.0 }; // NOTE: 会报错，因为 y 为 f64
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);
    {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }
        let integer = Point { x: 5, y: 10.0 };
        println!("{:?}", integer);
    }
}

/// NOTE: enum 中使用泛型
fn generic_enum() {
    utils::color_pringln_green("generic enum--------------------");

    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }

    #[derive(Debug)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let integer = Option::Some(5);
    println!("{:?}", integer);

    let result: Result<&str, ()> = Result::Ok("Ok");
    println!("{:?}", result);
}

/// NOTE: 方法定义中的泛型
/// 为 struct 或 enum 实现方法的时候，可在定义中使用泛型
/// 注意：
/// - 把T放在 impl关键字后，表示在类型T上实现方法
/// ·例如：impl<T>Point<T>
/// - 只针对具体类型实现方法（其余类型没实现方法）：
/// ·例如：impl Point<f32>
/// struct里的泛型类型参数可以和方法的泛型类型参数不同
fn generic_method() {
    utils::color_pringln_green("generic method--------------------");

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    // struct里的泛型类型参数可以和方法的泛型类型参数不同
    {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T: Copy, U> Point<T, U> {
            fn mixup<V, W>(&self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p1: {:?}", p1);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    // NOTE: 泛型代码的性能
    {
        let integer = Some(5);
        let float = Some(5.0);
        // 当 Rust 编译这些代码的时候，它会进行单态化。
        // 编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：
        // 一个对应 i32 另一个对应 f64。为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，
        // 接着将泛型定义替换为这两个具体的定义。
        // 编译器生成的单态化版本的代码看起来像这样，并包含将泛型 Option<T>
        // 替换为编译器创建的具体定义后的用例代码：
        // enum Option_i32 {
        //     Some(i32),
        //     None,
        // }
        //
        // enum Option_f64 {
        //     Some(f64),
        //     None,
        // }
        //
        // let integer = Option_i32::Some(5);
        // let float = Option_f64::Some(5.0);
    }
}
