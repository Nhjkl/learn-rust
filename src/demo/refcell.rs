use std::{cell::RefCell, rc::Rc};

use crate::utils;

// NOTE: 内部可变性(interior mutability)
// 内部可变性是 RUst 的设计模式之一
// 它允许你在只持有不可变引用的前提下对数据进行修改
// 数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则

pub fn run() {
    utils::color_pringln_green("Hello, RefCell");
    // NOTE: 选择Box<T>, Rc<T>, RefCell<T> 的依据
    //                    | Box<T>         | Rc<T>          | RefCell<T>
    // 同一数据的所有者   |  一个          | 多个           |  一个
    // 可变性，借用检查   |可变，不可变借用|不可变借用      | 可变，不可变借用
    //                    |（编译时检查）  |（编译时检查）  | （运行时检查）

    // NOTE: 内部可变性： 可变借用和一个不可变的值
    {
        let x = 5;
        // let y = &mut x; // BUG: 无法可变的借用一个不可变的值
    }

    {
        pub trait Messenger {
            fn send(&self, msg: &str);
        }

        pub struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }

        impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
        {
            pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value;

                let percentage_of_max = self.value as f64 / self.max as f64;
                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger.send("Upgrade your account today!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!");
                }
            }
        }

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                self.sent_messages.borrow_mut().push(String::from(msg));
            }
        }

        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        // NOTE: 使用 RefCelI<T>在运行时记录借用信息
        // RefCell<T>会记录当前存在多少个活跃的Ref<T>和 RefMut<T>智能指针：
        // - 每次调用borrow：不可变借用计数加1
        // - 任何一个 Ref<T>的值离开作用域被释放时：不可变借用计数减1
        // - 每次调用 borrow_mut：可变借用计数加 1
        // - 任何一个RefMut<T>的值离开作用域被释放时：可变借用计数减1
        // 以此技术来维护借用检查规则：
        // - 任何一个给定时间里，只允许拥有多个不可变借用或一个可变借用。
    }
    // NOTE: 将 Rc<T>和 RefCell<T>结合使用
    // 来实现一个拥有多重所有权的可变数据
    {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
        let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
    // NOTE: 其它可实现内部可变性的类型
    // Cell<T>：通过复制来访问数据
    // Mutex<T>：用于实现跨线程情形下的内部可变性模式
}
