use crate::utils;
use std::collections::HashMap;

pub fn run() {
    utils::color_pringln_green("hello HashMap");
    use_hashmap();
}

// NOTE: HashMap<K, V>
// 键值对的形式来存储数据, 一个建（key）对于一个值（Value）
// Hash函数：决定了如何在内存中存放K和V
// HashMap是同构的，
// - 所有的K必须是同一类型
// - 所有的V必须是同一类型

/// NOTE: use hashmap
fn use_hashmap() {
    utils::color_pringln_green("Use HashMap-----------------------");
    let mut scores = HashMap::new();
    // NOTE: 单独些句会报错，因为rust无法推断类型,
    // 可以显式指定看👇
    // let mut scores: HashMap<&str, i32> = HashMap::new();
    // 或者给scores添加数据如下👇
    scores.insert("Blue", 10);
    println!("{:?}", scores);

    // NOTE: 另外一种创建HashMap的方式： collect方法
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    println!("teams = {:?}", teams);
    let initial_scores = vec![10, 50];
    println!("initial_scores = {:?}", initial_scores);
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores = {:?}", scores);

    // NOTE: HashMap的所有权
    // - 对于实现了Copy trait 的类型（例如 i32），值会被复制到 HashMap 中
    // - 对于拥有所有权的值（例如 String），值会被移动，所有权会转移给HashMap
    // - (例子)
    // 如果将值的引用插入到HashMap，值本身不会移动－在 HashMap 有效的期间，
    // 被引用的值必须保持有效
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();

        map.insert(field_name, field_value);
        // map.insert(&field_name, &field_value); // NOTE: 通过引用,可以使所有权不被移动

        println!("map = {:?}", map);

        // println!("field_name = {}", field_name); // NOTE: 错误的，field_name已经被move了
    }

    // NOTE: 访问值
    // get方法
    // - 参数：K
    // - 返回：Option<&V>
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        match score {
            Some(s) => println!("Score = {}", s),
            None => println!("No score"),
        }

        // NOTE: 遍历HashMap
        utils::color_pringln_green("遍历HashMap----------------------");
        for (k, v) in &scores {
            println!("{}: {}", k, v);
        }
        for tp in &scores {
            println!("{}: {}", tp.0, tp.1);
        }
    }

    // NOTE: 更新值
    // HashMap 大小可变
    // 每个K同时只能对应一个V
    // 更新HashMap中的数据
    // - K 已经存在，对应一个V
    //   . 替换现有的V
    //   . 保留现有的V，忽略新的V.
    //   . 合并现有的V和新的V.
    // - K 不存在，
    //   . 添加一对新的K和V
    {
        utils::color_pringln_green("更新HashMap----------------------");

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        scores.entry(String::from("Yellow")).or_insert(50); // NOTE: 如果没有这个key，就添加
        scores.entry(String::from("Blue")).or_insert(50);
        println!("scores = {:?}", scores); // Blue 25

        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("map = {:?}", map);
    }

    // NOTE: Hash函数
    // 默认情况下，HashMap 使用加密功能强大的 Hash 函数，可以抵抗拒绝服务（DoS）攻击。
    // - 不是可用的最快的 Hash 算法
    // - 但具有更好安全性。
    // 可以指定不同的 hasher 来切换到另一个函数。
    // - hasher 是实现 BuildHasher trait 的类型
}
