use crate::utils;
pub fn run() {
    utils::color_pringln_green("Hello, if let");
    use_if_let();
}

fn use_if_let() {
    let v = Some(0u8);

    // NOTE: 等价于 if let
    // match v {
    //     3 => println!("three"),
    //     _ => (), // _ 表示匹配任何值
    // }

    fn match_three(v: Option<u8>) {
        if let Some(3) = v {
            println!("three")
        } else {
            println!("other")
        }
    }

    dbg!(match_three(Some(3)));
    dbg!(match_three(Some(4)));
}

// NOTE: 总结
// if let 处理只关心一种匹配而忽略其他匹配的情况
// 更少的代码
// 放弃了穷举的可能
// 可以把if let 看作是match 的语法糖
