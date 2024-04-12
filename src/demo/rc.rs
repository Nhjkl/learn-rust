use std::rc::Rc;

use crate::utils;

// NOTE: Rc<T> 引用计数智能指针
// 有时，一个值会有多个所有者
// 例如:
// 为了支持多重所有权：RC<T>
// - reference couting（引用计数）
// - 追踪所有到值的引用
// - 0个引用：该值可以被清理掉

pub fn run() {
    utils::color_pringln_green("Hello, Rc<T>");

    // NOTE: Rc<T>使用场景
    // 需要在heap上分配数据，这些数据被程序的多个部分读取（只读），
    // 但在编译时无法确定哪个部分最后使用完这些数据
    // RC<T>只能用于单线程场景
    // Rc::clone(&a) 函数：增加引用计数
    // Rc::strong_count(&a) 函数：获取引用计数
    // - 还有Rc::weak_count(&a)，获取弱引用计数
    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::{Cons, Nil};

        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

        let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a)); // BUG: a 的所有权被转移给 b
    }
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        {
            let b = Cons(3, Rc::clone(&a));
            let c = Cons(4, Rc::clone(&a));

            println!("a = {:?}", a);
            println!("b = {:?}", b);
            println!("c = {:?}", c);
            println!("count = {}", Rc::strong_count(&a));
        }

        println!("count = {}", Rc::strong_count(&a));
    }

    // NOTE: Rc:clone() vs 类型的 clone(） 方法
    // Rc:clone(）：增加引用，不会执行数据的深度拷贝操作
    // 类型的 clone()：很多会执行数据的深度拷贝操作。
    // Rc<T>
    // RC<T>通过不可变引I用，使你可以在程序不同部分之间共享只读数据
    // 但是，如何允许数据变化呢？
}
