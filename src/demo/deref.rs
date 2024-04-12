use std::ops::Deref;

use crate::utils;
// NOTE: Deref trait
// 实现 Deref trait 使我们可以自定义解引用运算符*的行为。
// 通过实现Deref trait， 智能指针可以像普通指针一样处理
pub fn run() {
    utils::color_pringln_green("Hello, Deref trait");

    // NOTE: 解引用运算符
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // NOTE: Box<T> 也实现了 Deref
    }

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, *y);
        assert_eq!(5, *(y.deref()));
    }

    // NOTE: 函数和方法的隐式解引用转化（Deref Conversion）
    // 隐式解引用（Deref Conversion）是为函数和方法提供的一种便捷特性
    // 假设T实现了 Deref trait:
    // - Deref Coercion可以把T的引用转化为T经过Deref操作后生成的引用
    // 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配：
    // - Deref Coercion 就会自动发生
    // - 编译器会对deref进行一系列调用，来把它转为所需的参数类型
    // 在编译时完成，没有额外性能开销
    {
        fn hello(name: &str) {
            println!("Hello, {}", name);
        }

        let m = MyBox::new(String::from("Rust"));
        // &m &MyBox<String>
        // deref &String
        // deref &str
        hello(&m);
        hello(&(*m)[..]);

        hello("Rust");
    }
    // NOTE: 解引用与可变性
    // 可使用DerefMut trait重载可变引用的*运算符
    // 在类型和trait在下列三种情况发生时，Rust 会执行deref coercion:
    // - 当T:Deref<Target=U>，允许 &T 转换为 &U
    // - 当T:DerefMut<Target=U>，允许 &mut T 转换为 &mut U
    // - 当T:Deref<Target=U>，允许 &mut T 转换为 &U
}
