use crate::utils;
// NOTE: Drop Trait
// 实现 Drop Trait，可以让我们自定义当值将要离开作用域时发生的动作
// - 例如：文件、网络资源释放等
// - 任何类型都可以实现 Drop trait
// Drop trait 只要求你实现 drop 方法
// - 参数：对 self 的可变引用
// Drop trait在预导入模块里（prelude）

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

pub fn run() {
    utils::color_pringln_green("Hello, drop trait");

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };

        println!("CustomSmartPointer created.");
    }

    // NOTE: 使用 std::mem::drop 来提前drop值
    // 很难直接禁用自动的drop功能，也没必要
    // - Drop tarit 的目的就是进行自动的释放处理逻辑
    // Rust 不允许手动调用Drop trait 的drop方法
    // 但可以调用标准库的std::mem::drop函数，提前drop值
    {
        let c = CustomSmartPointer {
            data: String::from("提前 drop 的值"),
        };
        // c.drop(); // BUG: explicit destructor calls not allowed
        drop(c);
        println!("CustomSmartPointer created.");
    }
}
