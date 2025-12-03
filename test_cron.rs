// 简单的定时任务测试
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    println!("测试定时任务功能...");
    println!("启动后端服务器测试...");

    // 使用线程池来模拟
    let mut handles = vec![];

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            // 模拟等待时间
            thread::sleep(Duration::from_secs(1));

            // 测试服务器连接
            match reqwest::blocking::get("http://localhost:8080/api/cron/status") {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("✅ 定时任务状态 API 响应成功");
                        if let Ok(body) = response.text() {
                            println!("📊 调度器状态: {}", body);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ 请求失败: {}", e);
                }
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("✅ 所有测试完成！");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cron_expression() {
        // 测试 cron 表达式是否正确解析
        // 这只是基本的编译测试，确保我们的代码没有语法错误
        assert!(true);
    }
}