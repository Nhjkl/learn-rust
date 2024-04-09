pub fn run() {
    println!("Hello, world! functions");
    another_function();
    function_with_parameters(5); // argument
    statement_expression();
    let v = function_with_return(6);
    println!("v = {}", v);
}

/// NOTE: snake case
fn another_function() {
    println!("Another function.");
}

/// NOTE: 参数（parameter）
fn function_with_parameters(x: i32) {
    println!("function_with_parameters---------------------------------");
    // parameter
    println!("x = {}", x);
}

/// NOTE: 语句和表达式 (Statements and Expressions)
fn statement_expression() {
    println!("statement_expression---------------------------------");
    // let y = (let x = 3); // NOTE: 不能将语句作为表达式
    let y = {
        let x = 3;
        x + 1 // 返回表达式
    };
}

/// NOTE: 带有返回值的函数 Functions with Return Values
fn function_with_return(x: i32) -> i32 {
    println!("function_with_return---------------------------------");
    if x > 5 {
        return 0; // NOTE: return 语句提前返回
    }
    x // NOTE: 返回千万不要加分号否则会报错
}
