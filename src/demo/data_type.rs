pub fn run() {
    println!("Hello, world! data type");

    {
        println!("integer------------------");
        let guess: u32 = "42".parse().expect("Not a Number");
        println!("guess: {}", guess);
    }

    {
        let s: u8 = 255;
        println!("{}", s);
    }

    {
        println!("bool------------------");
        let t = true;
        let f: bool = false;
        println!("foo: {}, bar: {}", t, f);
    }
    {
        println!("char------------------");
        let x = 'x';
        let y: char = 'ℤ';
        let z = '👋';
        println!("x: {}, y: {}, z: {}", x, y, z);
    }
    // 复合类型 （Compound Types）
    // 元组 Tuple
    {
        println!("tuple------------------");
        let tup: (i32, f64, u8) = (-12, 2.4, 1);
        println!("tup: {}, {}, {}", tup.0, tup.1, tup.2); // NOTE: 使用点标记法来访完元组元素
        let (x, y, z) = tup; // NOTE: 使用模式匹配来解构destructure
        println!("x: {}, y: {}, z: {}", x, y, z);
    }
    // 数组类型
    {
        println!("array------------------");
        let arr = [1, 2, 12];
        println!("arr: {:?}", arr);

        let first = arr[0];
        let second = arr[1];
        // let err = arr[3]; // NOTE: 通过下标访问数组时，下标越界, 简单来说，会直接panic
        let err1 = arr[arr[2]]; // NOTE: 这种情况build会通关，但在运行时，会出现错误
        println!("err: {}", err1);

        println!("first: {}, second: {}", first, second); // NOTE: 使用下标访问数组

        let arr2 = [3; 5]; // NOTE: 生成一个长度为5的数组，每个元素都是3
        println!("arr2: {:?}", arr2);
    }
}
