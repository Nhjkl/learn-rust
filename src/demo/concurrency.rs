use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use colored::Colorize;

use crate::utils;
// NOTE: 并发
// ·Concurrent：程序的不同部分之间独立的执行
// ·Parallel：程序的不同部分同时运行
// ·Rust 无畏并发：允许你编写没有细微 Bug 的代码，并在不引I入新 Bug 的情况下易于重构
// ·注意：本课程中的“并发”泛指 concurrent和 parallel。
pub fn run() {
    utils::color_pringln_green("Hello, Concurrency");
    // use_thread();
    // use_thread_move();
    use_channel();
    use_mutex();
    use_mutex_multi();
}

fn use_thread() {
    let hande = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the {} thread!", i, "spawned".green());
            thread::sleep(Duration::from_millis(1));
        }
    });

    hande.join().unwrap();

    println!("{}", "Main thread finished ✌️".yellow());

    for i in 0..5 {
        println!("hi number {} from the {} thread!", i, "main".red());
        thread::sleep(Duration::from_millis(1));
    }
}

// NOTE: 使用 move 闭包move闭包
// 通常和thread::spawn函数一起使用，它允许你使用其它线程的数据
// 创建线程时，把值的所有权从一个线程转移到另一个线程
fn use_thread_move() {
    let v = [1, 2, 3];

    let hande = thread::spawn(move || {
        println!("{:?}", v);
    });

    hande.join().unwrap();
}

// NOTE: 消息传递
// 一种很流行且能保证安全并发的技术就是：消息传递。
// - 线程（或Actor）通过彼此发送消息（数据）来进行通信
// Go语言的名言：不要用共享内存来通信，要用通信来共享内存。
// Rust：Channel（标准库提供）
// Channel
// Channel 包含：发送端、接收端
// 调用发送端的方法，发送数据
// 接收端会检查和接收到达的数据
// 如果发送端、接收端中任意一端被丢弃了，那么Channel就“关闭”了

// NOTE: 创建 Channel
// 使用 mpsc:channel 函数来创建 Channel
// - mpsc 表示 multiple producer, single consumer（多个生产者、一个消费者)
// - 返回一个tuple（元组）：里面元素分别是发送端、接收端(例子)
fn use_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // NOTE:  Channel 和所有权转移
        // 所有权在消息传递中非常重要：能帮你编写安全，并发的代码

        // println!("val is {}", val); // BUG: val 的所有权被转移了
    });
    // NOTE: 接收端的方法
    // recv方法：阻止当前线程执行，直到Channel 中有值被送来
    // - 一旦有值收到，就返回Result<T,E>
    // - 当发送端关闭，就会收到一个错误
    // try_recv 方法：不会阻塞,
    // - 立即返回 Result<T,E>:
    //      ·有数据达到：返回Ok，里面包含着数据
    //      ·否则，返回错误
    // - 通常会使用循环调用来检查 try_recv 的结果
    let received = rx.recv().unwrap();

    println!("Got: {}", received);

    {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx); // NOTE: 使用clone方法克隆发送端

        thread::spawn(move || {
            let msgs = vec![
                String::from("1: hi"),
                String::from("1: from"),
                String::from("1: the"),
                String::from("1: thread"),
            ];

            for msg in msgs {
                tx1.send(msg).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        thread::spawn(move || {
            let msgs = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for msg in msgs {
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}

// NOTE: 使用共享来实现并发
// Go语言的名言：不要用共享内存来通信，要用通信来共享内存。
// Rust支持通过共享状态来实现并发。
// Channel 类似单所有权：一旦将值的所有权转移至Channel，就无法使用它了
// 共享内存并发类似多所有权：多个线程可以同时访问同一块内存
// 使用 Mutex来每次只允许一个线程来访问数据
// Mutex是mutual exclusion（互斥锁）的简写
// 在同一时刻，Mutex只允许一个线程来访问某些数据想要访问数据：
// - 线程必须首先获取互斥锁 (lock)
//   ·lock 数据结构是 mutex的一部分，它能跟踪谁对数据拥有独占访问权
// - mutex通常被描述为：通过锁定系统来保护它所持有的数据
// Mutex 的两条规则
// 在使用数据之前，必须尝试获取锁（lock）
// 使用完 mutex 所保护的数据，必须对数据进行解锁，以便其它线程可以获取锁。

// NOTE: Mutex<T>的API
// 通过Mutex::new(数据）来创建Mutex<T>
//  - Mutex<T>是一个智能指针
// 访问数据前，通过lock方法来获取锁
//  - 会阻塞当前线程
//  - lock可能会失败
//  - 返回的是MutexGuard（智能指针，实现了Deref和Drop）
fn use_mutex() {
    utils::color_pringln_green("use_mutex ---------------");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// NOTE: 多线程共享Mutex<T>
fn use_mutex_multi() {
    utils::color_pringln_green("use_mutex_multi ---------------");
    // NOTE: 使用 Arc<T>来来进行原子引用计数
    // Arc<T>和Rc<T>类似，它可以用于并发情景
    // - A：atomic，原子的
    //为什么所有的基础类型都不是原子的，为什么标准库类型不默认使用ArC<T>?
    // - 需要性能作为代价
    // Arc<T>和 Rc<T>的 API 是相同的
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("num: {}", num);
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
}
// NOTE: RefCell<T>/Rc<T> vs Mutex<T>/Arc<T>
// Mutex<T>提供了内部可变性，和 Cell家族一样我们
// 使用RefCell<T>来改变 Rc<T>里面的内容
// 我们使用Mutex<T>来改变Arc<T>里面的内容
// 注意：Mutex<T>有死锁风险

// NOTE: Send 和 Sync trait
// Rust语言的并发特性较少，目前讲的并发特新都来自标准库（而不是语言本身)
// 无需局限于标准库的并发，可以自己实现并发
// 但在 Rust语言中有两个并发概念：
// - std::marker:Sync 和 std::marker:Send 这两个 trait
// Send: 允许线程间转移所有权
// 实现Send trait的类型可在线程间转移所有权
// Rust中几乎所有的类型都实现了Send
// - 但Rc<T>没有实现Send，它只用于单线程情景
// 任何完全由Send类型组成的类型也被标记为Send
// 除了原始指针之外，几乎所有的基础类型都是Send

// NOTE: Sync:允许从多线程访问
// 实现 Sync 的类型可以安全的被多个线程引用
// 也就是说：如果T是Sync，那么 &T就是Send
//  - 引用可以被安全的送往另一个线程
// 基础类型都是 Sync
// 完全由 Sync 类型组成的类型也是Sync
// - 但，Rc<T>不是 Sync 的
// - RefCell<T>和Cell<T>家族也不是Sync的
// - 而，Mutex<T>是Sync 的

// NOTE: 手动来实现 Send 和 Sync 是不安全的记住这句话即可
