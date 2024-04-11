use crate::utils;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
// NOTE: 什么是闭包（closure)
// 闭包：可以捕获其所在环境的匿名函数。
// 闭包：
// - 是匿名函数
// - 保存为变量、作为参数
// - 可在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
// - 可从其定义的作用域捕获值
pub fn run() {
    utils::color_pringln_green("Hello, closures!");

    // let simulated_user_specified_value = 20;
    // let simulated_random_number = 3;
    //
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    fn_closure_syntax();
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value_map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let value = self.value_map.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        // let expensive_result = expensive_closure(intensity);
        // println!("Today, do {} pushups!", expensive_result);
        // println!("Next, do {} situps!", expensive_result);
        // TODO: 使用闭包特性优化
        let mut cacher = Cacher::new(expensive_closure);
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// NOTE: 闭包的类型推断
// 闭包不要求标注参数和返回值的类型
// 闭包通常很短小，只在狭小的上下文中工作，编译器通常能推断出类型
// 可以手动添加类型标注

// NOTE: 函数和闭包定义的语法
fn fn_closure_syntax() {
    utils::color_pringln_green("fn_closure_syntax -------------------------- ");
    // fn add_one_v1    (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
    // NOTE: 闭包的类型推断
    let example_closure = |x| x;
    let stored_closure = example_closure(String::from("hello"));
    // let n = example_closure(5); // BUG: 由于👆推断出类型为: i32, 所以不能再传入 integer
    println!("{}", stored_closure);

    // NOTE: 闭包捕获环境变量
    {
        let x = 4;
        let equal_to_x = |z| z == x;
        // fn equal_to_x2(z: i32) -> bool {
        //     z == x // BUG: 不能在fn中捕获外部动态环境
        // }
        // NOTE: 闭包可以捕获他们所在的环境
        // 闭包可以访问定义它的作用域内的变量，而普通函数则不能。
        // 会产生内存开销。
        let y = 4;
        assert!(equal_to_x(y));

        // NOTE: 闭包从所在环境捕获值的方式
        // 与函数获得参数的三种方式一样：
        // 1.取得所有权：FnOnce
        // 2.可变借用：FnMut
        // 3.不可变借用：Fn
        // 创建闭包时，通过闭包对环境值的使用，Rust推断出具体使用哪个 trait:
        // - 所有的闭包都实现了FnOnce
        // - 没有移动捕获变量的实现了FnMut
        // - 无需可变访问捕获变量的闭包实现了 Fn
    }

    // NOTE: move 关键字
    // 在参数列表前使用 move 关键字，可以强制闭包取得它所使用的环境值的所有权
    // - 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最为有用。
    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("x: {:?}", x); // BUG: 通过move关键字，将x的所有权交给了equal_to_x, 所以不能再使用x
        let y = vec![1, 2, 3];
        println!("{}", equal_to_x(y));
    }

    // NOTE: 最佳实践当指定 Fn trait bound 之一时，首先用 Fn，基于闭包体里的情况，
    // 如果需要FnOnce 或 FnMut，编译器会再告诉你。
}
