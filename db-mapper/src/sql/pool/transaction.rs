// 独立的事务测试文件
use std::cell::RefCell;
use tokio::task_local;

// 导入事务属性宏
use db_macros::transactional;

task_local! {
    static TX_ID_REGISTRY: RefCell<Option<String>>;
}

// 定义一个事务管理器 trait
trait Transaction {
    fn begin(&mut self);
    fn commit(&mut self);
    fn rollback(&mut self);
}

// 简单的事务管理器实现
struct SimpleTransaction {
    active: bool,
}

impl SimpleTransaction {
    fn new() -> Self {
        Self { active: false }
    }
}

impl Transaction for SimpleTransaction {
    fn begin(&mut self) {
        self.active = true;
        println!("[TRANSACTION] Begin transaction");
    }

    fn commit(&mut self) {
        if self.active {
            println!("[TRANSACTION] Commit transaction");
            self.active = false;
        }
    }

    fn rollback(&mut self) {
        if self.active {
            println!("[TRANSACTION] Rollback transaction");
            self.active = false;
        }
    }
}

// 测试用例：使用方法属性宏的示例
struct Service {
    tx_manager: SimpleTransaction,
}

impl Service {
    fn new() -> Self {
        Self {
            tx_manager: SimpleTransaction::new(),
        }
    }

    // 示例1：无返回值的事务方法
    #[transactional]
    fn do_something(&mut self) -> Result<(), String> {
        println!("Executing business logic...");
        // 模拟正常执行
        Ok(())
    }

    // 示例2：有返回值的事务方法
    #[transactional]
    fn do_something_with_result(&mut self) -> Result<i32, String> {
        println!("Executing business logic with return...");
        Ok(42)
    }

    // 示例3：可能失败的事务方法
    #[transactional]
    fn do_something_that_might_fail(&mut self, should_fail: bool) -> Result<String, String> {
        println!("Executing business logic that might fail...");
        if should_fail {
            Err("Business error occurred".to_string())
        } else {
            Ok("Success".to_string())
        }
    }

    // 示例4：可能发生 panic 的事务方法
    #[transactional]
    fn do_something_that_might_panic(&mut self, should_panic: bool) -> Result<String, String> {
        println!("Executing business logic that might panic...");
        if should_panic {
            panic!("Unexpected error!");
        } else {
            Ok("Success".to_string())
        }
    }
}

// 测试用例
fn main() {
    println!("=== 测试1：无返回值的事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something();

    match result {
        Ok(_) => println!("Transaction succeeded"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("\n=== 测试2：有返回值的正常事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something_with_result();

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("\n=== 测试3：返回成功的事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something_that_might_fail(false);

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Business logic failed (transaction rolled back): {}", e),
    }

    println!("\n=== 测试4：返回错误的事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something_that_might_fail(true);

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Business logic failed (transaction rolled back): {}", e),
    }

    println!("\n=== 测试5：不发生 panic 的事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something_that_might_panic(false);

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("\n=== 测试6：发生 panic 的事务方法 ===");

    let mut service = Service::new();
    let result = service.do_something_that_might_panic(true);

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Transaction failed with panic (rolled back): {}", e),
    }
}