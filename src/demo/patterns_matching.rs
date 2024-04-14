use crate::utils;
// NOTE: 模式
// 模式是Rust中的一种特殊语法，用于匹配复杂和简单类型的结构
// 将模式与匹配表达式和其他构造结合使用，可以更好地控制程序的控制流模
// 式由以下元素（的一些组合）组成：
// - 字面值
// - 解构的数组、enum、struct和 tuple
// - 变量
// - 通配符
// - 占位符
// 想要使用模式，需要将其与某个值进行比较：
// - 如果模式匹配，就可以在代码中使用这个值的相应部分
//
// NOTE: match 的 Arm
//  match VALUE {
//    PATTERN =>EXPRESSION,
//    PATTERN=>EXPRESSION,
//    PATTERN=>EXPRESSION,
//  }
// match表达式的要求;
// - 详尽（包含所有的可能性）
// 一个特殊的模式：_（下划线）：
// - 它会匹配任何东西
// - 不会绑定到变量
// - 通常用于 match 的最后一个arm；或用于忽略某些值。
//
// NOTE: 条件 if let表达式
// if let表达式主要是作为一种简短的方式来等价的代替只有一个匹配项的 match
// if let 可选的可以拥有 else，包括:
// - else if
// - else if let
// 但，if let 不会检查穷尽性
pub fn run() {
    utils::color_pringln_green("Hello, Concurrency");

    // NOTE: while let 条件循环
    // 只要模式继续满足匹配条件，那就允许while 循环一直运行

    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        println!("{:?}", stack.pop());
    }

    // NOTE: let 语句
    // let 语句也是模式
    // let PATTERN = EXPRESSION
    {
        let a = 5;
        let (x, y, z) = (4, 5, 6);
        // let (x, y) = (4, 5, 6); // BUG: this expression has more than 3 elements
    }

    // NOTE: 函数的参数
    // 函数的参数也可以时模式
    //
    {
        fn foo(x: i32) {}

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }

        let point = (3, 5);

        print_coordinates(&point);
    }

    // NOTE: 模式的两种形式
    // 模式有两种形式：可辨驳的、无可辩驳的
    // 能匹配任何可能传递的值的模式：无可辩驳的
    // - 例如：let x = 5;
    // 对某些可能的值，无法进行匹配的模式：可辨驳的
    // - 例如：if let Some(x)= a_value
    // 函数参数、let语句、for 循环只接受无可辩驳的模式
    // if let和while let接受可辨驳和无可辩驳的模式
    {
        let a: Option<i32> = Some(5);
        // let Some(x) = a; // BUG: pattern None not covered
        if let Some(x) = a {
            println!("{}", x);
        }

        if let x = 5 {} // WARNING: 可接受不可辩驳的，但没有意义
    }

    // NOTE: 匹配字面值
    // 模式可以直接匹配字面值
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            _ => println!("anything"),
        }
    }

    // NOTE: 匹配命名变量
    // 命名变量时匹配任何值的无可辩驳模式
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    // NOTE: 多重模式
    // 在 match表达式中，使用 | 语法，可以匹配多种模式
    {
        let x = 5;
        let y = 10;

        match y {
            10 | 5 => println!("Got 5 or 10"),
            _ => {
                println!("Default case, x = {:?}", x);
            }
        }
    }

    // NOTE: 通过 ..= 匹配值的范围
    {
        let x = 5;
        let y = 'y';
        let z = '🀄';

        match x {
            0.. => println!("one through five"),
            _ => println!("something else"),
        }

        match y {
            'a'..='z' => println!("a through z"),
            _ => println!("something else"),
        }
    }

    // NOTE: 解构以分解值
    // 可以使用模式来解构struct， enum， tuple， 从而引用这些类型值的不同部分
    {
        utils::color_pringln_green("解构以分解值");
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis:({}, {})", x, y),
        }
    }
    // NOTE: 解构 enum
    {
        utils::color_pringln_green("解构 enum");

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let m = Message::ChangeColor(0, 160, 255);

        match m {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }

    // NOTE: 解构嵌套的 struct 和 enum
    {
        utils::color_pringln_green("解构嵌套的 struct 和 enum");
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let m = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match m {
            // Message::ChangeColor(Color::Rgb(r, g, b)) => {
            //     println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            // }
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            Message::ChangeColor(_) => (),
            _ => (),
        }
    }

    // NOTE: 解构struct 和 tuple
    {
        utils::color_pringln_green("解构struct 和 tuple");

        struct Point {
            x: i32,
            y: i32,
        }
        let ts = ((3, 10), Point { x: 3, y: -10 });
        let ((feet, inches), Point { x, y }) = ts;
        println!("feet: {}, inches: {}", feet, inches);
        println!("x: {}, y: {}", x, y);
    }

    // NOTE: 在模式中忽略值
    // 有几种方式可以模式中忽略整个值或部分值：
    // - _
    // - _ 配合其它模式
    // - 使用以_开头的名称
    // - .. (忽略值的剩余部分)
    {
        {
            fn foo(_: i32, y: i32) {
                println!("This code only uses the y parameter: {}", y);
            }

            foo(3, 4);
        }

        {
            let mut setting_value = Some(5);
            let new_setting_value = Some(10);

            match (setting_value, new_setting_value) {
                (Some(_), Some(_)) => {
                    // NOTE: 值关注 两个值是Some类型
                    println!("Can't overwrite an existing customized value");
                }
                _ => {
                    setting_value = new_setting_value;
                }
            }
        }

        // NOTE: 忽略值的某个部分
        {
            utils::color_pringln_green("忽略值的某个部分");
            let numbers = (2, 4, 8, 16, 32);
            let (first, _, third, _, fifth) = numbers;
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
        // NOTE: 通过只用下划线开头命名来忽略未使用的变量
        {
            let _x = 5;
            let y = 10;
            println!("y: {}", y);

            let s = Some(String::from("Hello!"));

            if let Some(_s) = s {
                println!("found a string");
            }

            // println!("s: {:?}", s); // BUG: 👆代码使用Some(_s)会移交所有权, 可以使用Some(_),来修复
        }
        // NOTE: 使用..来忽略值的剩余部分
        {
            utils::color_pringln_green("使用..来忽略值的剩余部分");
            struct Point {
                x: i32,
                y: i32,
            }

            let origin = Point { x: 0, y: 0 };
            let Point { x, .. } = origin;
            println!("x: {}", x);

            let numbers = (2, 4, 8, 16, 32);
            let (first, .., last) = numbers;
            println!("Some numbers: {}, {}", first, last);

            // let (.., second, ..) = numbers; // BUG: 不能确定时中期的哪一个
        }

        // NOTE: 使用 match 守卫来提供额外的条件
        // match守卫就是matcharm 模式后额外的if条件，想要匹配该条件也必须满足
        // match守卫适用于比单独的模式更复杂的场景
        {
            utils::color_pringln_green("使用 match 守卫来提供额外的条件");
            let num = Some(4);
            match num {
                Some(x) if x < 5 => println!("less than five: {}", x),
                Some(x) => println!("{}", x),
                None => (),
            }

            let x = Some(5);
            let y = 10;

            match x {
                Some(50) => println!("Got 50"),
                Some(n) if n == y => println!("Matched, n = {}", n),
                _ => println!("Default case, x = {:?}", x),
            }

            {
                let x = 4;
                let y = false;
                match x {
                    4..=6 if y => println!("yes"),
                    _ => println!("no"),
                }
            }
        }

        // NOTE: @绑定
        // @符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值
        {
            utils::color_pringln_green("使用 @绑定");
            enum Message {
                Hello { id: i32 },
            }

            let msg = Message::Hello { id: 5 };

            match msg {
                Message::Hello {
                    id: id_variable @ 3..=7,
                } => println!("Found an id in range: {}", id_variable),
                Message::Hello { id: 10..=12 } => {
                    println!("Found an id in another range")
                }
                Message::Hello { id } => println!("Found some other id: {}", id),
            }
        }
    }
}
