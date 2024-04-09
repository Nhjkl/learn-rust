use crate::utils;

pub fn run() {
    utils::color_pringln_green("Hello, struct");
    show_user_struct();
    let build_user = build_user(String::from("Sean"));
    println!("{:#?}", build_user);
    build_user_update();
    build_user_tuple();
    unit_like();
    struct_use();
    struct_method();
}

// NOTE: 定义struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn show_user_struct() {
    utils::color_pringln_green("show_user_struct---------------");
    let mut user1 = User {
        // NOTE: 一旦声明struct为mut，所有属性都可变
        active: true,
        username: String::from("someusername123"),
        email: String::from("nWU4J@example.com"),
        sign_in_count: 19324234,
    };

    println!("{:#?}", user1);
    println!("user1 email: {}", user1.email); // NOTE: 点标记法获取属性值
    user1.email = String::from("liuxian@example.com");
    println!("user1 email: {}", user1.email);
}

fn build_user(username: String) -> User {
    utils::color_pringln_green("build_user---------------");
    // NOTE: struct 作为返回值
    User {
        username, // NOTE: 变量名称和struct属性名一致，可以简写
        email: String::from("nWU4J@example.com"),
        active: true,
        sign_in_count: 1,
    }
}

// NOTE: Creating Instances from Other Instances with Struct Update Syntax
fn build_user_update() {
    utils::color_pringln_green("build_user_update---------------");
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("nWU4J@example.com"),
        sign_in_count: 345,
    };

    let user2 = User {
        username: String::from("otherusername123"),
        email: String::from("othernWU4J@example.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // NOTE: 上面代码等价于👇的写法
    let user3 = User {
        username: String::from("otherusername123"),
        email: String::from("othernWU4J@example.com"),
        ..user1 // NOTE: struct更新语法
    };

    println!("user2: {:#?}", user2);
    println!("user3: {:#?}", user3);
}

// NOTE: Using Tuple Structs Without Named Fields to Create Different Types
fn build_user_tuple() {
    utils::color_pringln_green("build_user_tuple---------------");

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let blue = Color(0, 0, 255);
    let origin = Point(0, 0, 0);

    println!("blue: {:?}", blue);
    println!("origin: {:?}", origin);
}

// NOTE: Unit-Like Structs Without Any Fields
fn unit_like() {
    utils::color_pringln_green("unit_like---------------");

    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    println!("{:#?}", subject);
}

// NOTE: Ownership of Struct Data
fn struct_ownership() {
    utils::color_pringln_green("struct_ownership---------------");
    // NOTE: User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。
    // 这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
    // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期（lifetime），
    // 这是一个第 10 章会讨论的 Rust 功能。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。
    // 如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：

    // struct User {
    //     active: bool,
    //     username: &str, // expected named lifetime parameter
    //     email: &str,
    //     sign_in_count: u64,
    // }
    //
    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

// NOTE: 如何使用结构体
fn struct_use() {
    utils::color_pringln_green("struct_use---------------");
    {
        utils::color_pringln_green("1. 不是用结构体实现---------------");

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        let width = 30;
        let height = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width, height)
        );
    }
    {
        utils::color_pringln_green("2. 使用元组实现---------------");

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        let rect = (30, 50);

        println!("The area of the rectangle is {} square pixels.", area(rect));
    }

    {
        utils::color_pringln_green("3. 使用struct实现---------------");
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect = Rectangle {
            width: dbg!(30 * 2),
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect)
        );

        // println!("rect: {:#?}", rect); //NOTE: 关于std::fmt::Debug std::fmt::Display https://rustwiki.org/zh-CN/book/ch05-02-example-structs.html
        dbg!(&rect);
    }

    // NOTE: 知识点
    // std::fmt::Display
    // std::fmt::Debug
    // #[derive(Debug)]
    // {:?}
    // {:#?}
}

// NOTE: 结构体的方法 Method Syntax
fn struct_method() {
    utils::color_pringln_green("struct_method---------------");
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // NOTE: 为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写）
    impl Rectangle {
        // NOTE: 在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // NOTE: 每个结构体都允许拥有多个 impl 块。
    impl Rectangle {
        // 判断是否可以包含另一个矩形
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // NOTE: 关联函数(Associated Functions)
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        dbg!(rect.area());
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 35,
            height: 55,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let sq = Rectangle::square(30);
        dbg!(&sq);
    }
}
