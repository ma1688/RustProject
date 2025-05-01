use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    // 示例1：基本线程创建和连接
    println!("示例1：基本线程创建和连接");
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("线程：计数 {}", i);
        }
    });
    
    for i in 1..6 {
        println!("主线程：计数 {}", i);
        thread::sleep(Duration::from_millis(300));
    }
    
    // 等待生成的线程完成
    handle.join().unwrap();
    
    println!("\n示例2：使用互斥锁在线程之间共享数据");
    
    // 示例2：使用互斥锁在线程之间共享数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("线程 {}：计数器增加到 {}", i, *num);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数器值：{}", *counter.lock().unwrap());
    
    println!("\n示例3：使用通道进行线程通信");
    
    // 示例3：使用通道进行线程通信
    let (tx, rx) = mpsc::channel();
    
    // 创建多个发送者线程
    for i in 1..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("来自线程 {} 的消息", i);
            thread::sleep(Duration::from_millis(i * 200));
            tx_clone.send(message).unwrap();
        });
    }
    
    // 删除原始发送者以在所有发送者克隆被丢弃时正确关闭通道
    drop(tx);
    
    // 接收并打印消息
    while let Ok(received) = rx.recv() {
        println!("接收到：{}", received);
    }
    
    println!("\n示例4：并行数据处理");
    
    // 示例4：并行数据处理 - 模拟大数据计算任务
    // 创建一个数据向量
    let data: Vec<i32> = (1..1001).collect();
    println!("需要处理的数据量: {}", data.len());
    
    // 记录开始时间
    let start = std::time::Instant::now();
    
    // 单线程处理
    let sum_single = data.iter().map(|&x| {
        // 模拟复杂计算
        thread::sleep(Duration::from_micros(100));
        x * x
    }).sum::<i32>();
    
    let single_thread_time = start.elapsed();
    println!("单线程处理耗时: {:?}, 结果: {}", single_thread_time, sum_single);
    
    // 多线程并行处理相同的数据
    let start = std::time::Instant::now();
    
    // 确定线程数量（这里使用4个线程作为示例）
    let num_threads = 4;
    let chunk_size = data.len() / num_threads;
    
    let data_arc = Arc::new(data);
    let results = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let data_ref = Arc::clone(&data_arc);
        let results_ref = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let start_idx = thread_id * chunk_size;
            let end_idx = if thread_id == num_threads - 1 {
                data_ref.len()
            } else {
                (thread_id + 1) * chunk_size
            };
            
            let mut thread_sum = 0;
            for i in start_idx..end_idx {
                // 模拟复杂计算
                thread::sleep(Duration::from_micros(100));
                thread_sum += data_ref[i] * data_ref[i];
            }
            
            // 将结果存储在共享的HashMap中
            let mut results_map = results_ref.lock().unwrap();
            results_map.insert(thread_id, thread_sum);
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 汇总所有线程的结果
    let final_results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    let sum_multi: i32 = final_results.values().sum();
    
    let multi_thread_time = start.elapsed();
    println!("多线程处理耗时: {:?}, 结果: {}", multi_thread_time, sum_multi);
    println!("速度提升: {:.2}倍", single_thread_time.as_secs_f64() / multi_thread_time.as_secs_f64());
    
    println!("\n所有示例已完成！");
}
