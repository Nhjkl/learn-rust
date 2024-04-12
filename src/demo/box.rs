use crate::utils;
pub fn run() {
    utils::color_pringln_green("Hello, Box<T>");
    setup();
}

// NOTE: 使用Box<T>来指向Heap上的数据
// Box<T>是一个最简单智能指针
// - 允许你在Heap上存储数据（而不是stack）
// - stack 上是指向heap数据的指针
// - 没有其他的性能开销
// - 没有其他额外的功能
// 为什么是智能指针？
// - 实现了 Deref trait 和 Drop trait
// 使用场景
// 在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小
// 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制
// 使用某个值时，你只关心它师傅实现了特定的trait，而不关心他的具体类型
fn setup() {
    // NOTE: 使用Box
    {
        let a = Box::new(5); // 5 是存到heap上面
        let b = 4;
        let c = b;
        let d = a;
        println!("a = {}", b);
        // println!("b = {}", a); // BUG:a 的所以权，被移交给了b
    }
    // NOTE: 使用Box赋能递归类型
    // 在编译时，Rust需要知道一个类型所占的空间大小。
    // 而递归类型的大小无法在编译时确定
    // 但Box类型可以确定大小
    {
        // BUG: 1. recursive type `demo::r#box::setup::List` has infinite size [E0072]
        // enum List {
        //     Cons(i32, List),
        //     Nil,
        // }
        use List::*;
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);
    }
}
