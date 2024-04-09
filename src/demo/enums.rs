use crate::utils;
pub fn run() {
    println!("Hello, Enums");
    define_enum();
}

// NOTE: Defining an Enum
fn define_enum() {
    utils::color_pringln_green("Defining an Enum");

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    fn route(ip_kind: IpAddrKind) {
        println!("{:?}", ip_kind);
    }

    // NOTE: 将数据附加到枚举的变体中
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        dbg!(home);
        dbg!(loopback);
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // NOTE: 为枚举定义方法
    impl Message {
        fn call(&self) {
            println!("self={:?}", self);
        }
    }

    {
        let q = Message::Quit;
        let m = Message::Move { x: 1, y: 2 };
        let w = Message::Write(String::from("hello"));
        let c = Message::ChangeColor(1, 2, 3);

        q.call();
        m.call();
        w.call();
        c.call();
    }
}
