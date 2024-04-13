use crate::utils;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn run() {
    utils::color_pringln_green("Hello, Leak memory test!");
    // NOTE: Rust可能发生内存泄漏
    // Rust的内存安全机制可以保证很难发生内存泄漏，但不是不可能
    // 例如使用Rc<T>和RefCell<T>就可能创造出循环引用，从而发生内存泄漏：
    // -每个项的引用数量不会变成0，值也不会被处理掉。
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        use List::{Cons, Nil};

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
        // println!("a = {:?}", &a);
        // println!("b = {:?}", &b);

        // NOTE: 防止内存泄漏的解决办法
        // ·依靠开发者来保证，不能依靠RUst
        // ·重新组织数据结构：一些引用来表达所有权，一些引用不表达所有权
        // - 循环引用中的一部分具有所有权关系，另一部分不涉及所有权关系
        // - 而只有所有权关系才影响值的清理
    }

    // NOTE: 防止循环引用
    // 把 Rc<T>换成 Weak<T>
    // Rc::clone 为 Rc<T>实例的 strong_count加 1，Rc<T>的实例只有在
    // strong_count为 O的时候才会被清理
    // Rc<T>实例通过调用Rc:downgrade 方法可以创建值的Weak Reference（弱引用)
    // - 返回类型是Weak<T>（智能指针)
    // - 调用 Rc:downgrade 会为weak_count 加1。
    // Rc<T>使用weak_count 来追踪存在多少 Weak<T>。
    // weak_count不为O并不影响Rc<T>实例的清理
    // Strong vs Weak
    // Strong Reference（强引用）是关于如何分享Rc<T>实例的所有权
    // Weak Reference（弱引用）并不表达上述意思使用 Weak Reference并不会创建循环引用：
    // - 当Strong Reference数量为O的时候，Weak Reference会自动断开
    // 在使用Weak<T>前，需保证它指向的值仍然存在：
    // - 在 Weak<T>实例上调用 upgrade 方法，返回 Option<Rc<T>>
    {
        utils::color_pringln_green("Strong vs Weak ---------------");
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
