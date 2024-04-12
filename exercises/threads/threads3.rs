// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    // 同的线程中移动同一个 Arc 实例 qc，这会导致编译错误，因为移动后你无法再次访问它。
    // 其次，需要在主线程中等待所有子线程完成任务，然后关闭通道，以确保主线程在接收到所有数据后不会阻塞。
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }).join().unwrap();

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }).join().unwrap();
}

fn main() {
    // 创建一个异步通道，用于在不同的任务之间传递数据
    let (tx, rx) = mpsc::channel();
    // 创建一个新的队列
    let queue = Queue::new();
    // 获取队列的长度
    let queue_length = queue.length;

    // 调用 send_tx 函数，将队列 queue 和发送端 tx 作为参数传入
    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
