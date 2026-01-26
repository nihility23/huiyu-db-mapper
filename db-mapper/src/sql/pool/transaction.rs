use std::cell::RefCell;
use tokio::task_local;

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

// 主事务宏
macro_rules! transactional {
    // 情况1：修饰没有返回值的代码块
    ($tx:expr, $block:block) => {{
        let mut transaction = $tx;
        transaction.begin();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            $block
        }));

        match result {
            Ok(_) => {
                transaction.commit();
                Ok(())
            }
            Err(e) => {
                transaction.rollback();
                Err(format!("Transaction failed with panic: {:?}", e))
            }
        }
    }};

    // 情况2：修饰返回 Result<T, E> 的代码块
    ($tx:expr, result $block:expr) => {{
        let mut transaction = $tx;
        transaction.begin();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            $block
        }));

        match result {
            Ok(inner_result) => match inner_result {
                Ok(value) => {
                    transaction.commit();
                    Ok(value)
                }
                Err(e) => {
                    transaction.rollback();
                    Err(e)
                }
            },
            Err(panic_err) => {
                transaction.rollback();
                Err(format!("Transaction panicked: {:?}", panic_err).into())
            }
        }
    }};
}

// 宏的简化版本，自动创建事务管理器
macro_rules! transaction {
    ($block:block) => {{
        let tx = SimpleTransaction::new();
        transactional!(tx, $block)
    }};

    // 返回 Result 的版本
    (result $block:expr) => {{
        let tx = SimpleTransaction::new();
        transactional!(tx, result $block)
    }};
}

// 测试用例
fn main() {
    println!("=== 测试1：无返回值的事务 ===");

    let tx = SimpleTransaction::new();
    let result = transactional!(tx, {
        println!("Executing business logic...");
        // 模拟正常执行
    });

    match result {
        Ok(_) => println!("Transaction succeeded"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("\n=== 测试2：有返回值的正常事务 ===");

    let tx = SimpleTransaction::new();
    let result = transactional!(tx, result {
        println!("Executing business logic with return...");
        Ok::<i32, String>(42)
    });

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("\n=== 测试3：返回错误的业务逻辑 ===");

    let tx = SimpleTransaction::new();
    let result = transactional!(tx, result {
        println!("Executing failing business logic...");
        Err::<i32, String>("Business error occurred".to_string())
    });

    match result {
        Ok(value) => println!("Transaction succeeded with value: {}", value),
        Err(e) => println!("Business logic failed (transaction rolled back): {}", e),
    }

    println!("\n=== 测试4：发生 panic 的事务 ===");

    let tx = SimpleTransaction::new();
    let result = transactional!(tx, {
        println!("About to panic...");
        panic!("Unexpected error!");
    });

    match result {
        Ok(_) => println!("Transaction succeeded"),
        Err(e) => println!("Transaction failed with panic (rolled back): {}", e),
    }

    println!("\n=== 测试5：简化宏的使用 ===");

    let result = transaction!(result {
        println!("Using simplified transaction macro...");
        Ok::<String, String>("Success".to_string())
    });

    match result {
        Ok(msg) => println!("Result: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
}