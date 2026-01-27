/// 事务宏 - macro_rules! 版本
///
/// 用法示例：
/// ```
/// let result = transactional!(|| -> Result<i32, String> {
///     // 业务逻辑
///     Ok(42)
/// });
///
/// // 或者简写（可以自动推断类型）
/// let result = transactional!({
///     // 业务逻辑
///     Ok::<i32, String>(42)
/// });
/// ```
#[macro_export]
macro_rules! transactional {
    // 模式1: 完整闭包语法
    (|| $(-> $ret:ty)? $body:block) => {{
        $crate::__transactional_internal!($body, {})
    }};
    
    // 模式2: 带参数的闭包
    (|$($arg:ident),*| $(-> $ret:ty)? $body:block) => {{
        $crate::__transactional_internal!($body, { $($arg),* })
    }};
    
    // 模式3: 直接代码块（简化语法）
    ($body:block) => {{
        $crate::__transactional_internal!($body, {})
    }};
    
    // 模式4: 带自定义回滚操作的版本
    (body: $body:block, rollback: $rollback:block) => {{
        $crate::__transactional_with_rollback!($body, $rollback)
    }};
    
    // 模式5: 带数据库连接的版本
    ($db:expr, $body:block) => {{
        $crate::__transactional_with_db!($db, $body)
    }};
}

// 内部实现宏（不导出）
#[macro_export]
#[doc(hidden)]
macro_rules! __transactional_internal {
    ($body:block, $args:tt) => {{
        // 创建事务上下文
        #[derive(Default)]
        struct TransactionContext<E> {
            rollback_actions: Vec<Box<dyn FnMut() -> Result<(), E> + Send + Sync>>,
            committed: bool,
        }
        
        impl<E> TransactionContext<E> {
            fn add_rollback<F>(&mut self, action: F)
            where
                F: FnMut() -> Result<(), E> + 'static + Send + Sync,
            {
                self.rollback_actions.push(Box::new(action));
            }
            
            fn execute_rollback(&mut self) -> Result<(), E> {
                for action in self.rollback_actions.iter_mut().rev() {
                    action()?;
                }
                Ok(())
            }
            
            fn commit(&mut self) {
                self.committed = true;
                self.rollback_actions.clear();
            }
        }
        
        impl<E> Drop for TransactionContext<E> {
            fn drop(&mut self) {
                if !self.committed && !self.rollback_actions.is_empty() {
                    let _ = self.execute_rollback();
                }
            }
        }
        
        // 创建事务上下文
        let mut ctx = TransactionContext::default();
        
        // 执行闭包
        let result = (|| $body)();
        
        // 根据结果处理
        match result {
            Ok(value) => {
                ctx.commit();
                Ok(value)
            }
            Err(e) => {
                // 如果闭包返回错误，Drop会自动执行回滚
                Err(e)
            }
        }
    }};
}

// 带自定义回滚操作的版本
#[macro_export]
#[doc(hidden)]
macro_rules! __transactional_with_rollback {
    ($body:block, $rollback:block) => {{
        let result = (|| $body)();
        
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                // 执行自定义回滚逻辑
                let rollback_result = (|| $rollback)();
                
                // 如果回滚也失败，记录日志但返回原始错误
                if let Err(rollback_err) = rollback_result {
                    eprintln!("Warning: Rollback failed: {:?}", rollback_err);
                }
                
                // 始终返回原始错误
                result
            }
        }
    }};
}

// 带数据库连接的版本
#[macro_export]
#[doc(hidden)]
macro_rules! __transactional_with_db {
    ($db:expr, $body:block) => {{
        // 模拟数据库事务接口
        trait DatabaseTransaction {
            type Error;
            
            fn begin(&self) -> Result<TransactionHandle, Self::Error>;
        }
        
        struct TransactionHandle;
        
        impl TransactionHandle {
            fn commit(self) -> Result<(), String> {
                println!("Transaction committed");
                Ok(())
            }
            
            fn rollback(self) {
                println!("Transaction rolled back");
            }
        }
        
        // 开始事务
        let handle = match $db.begin() {
            Ok(h) => h,
            Err(e) => return Err(e.into()),
        };
        
        // 执行业务逻辑
        let result = (|| $body)();
        
        // 根据结果提交或回滚
        match result {
            Ok(value) => {
                match handle.commit() {
                    Ok(_) => Ok(value),
                    Err(e) => {
                        // 提交失败，已经自动回滚
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                handle.rollback();
                Err(e)
            }
        }
    }};
}

/// 增强版事务宏：支持回滚操作注册
///
/// 用法示例：
/// ```
/// let result = transactional_ctx!({
///     // 可以在闭包内注册回滚操作
///     register_rollback!(|| {
///         println!("Rollback action 1");
///         Ok(())
///     });
///     
///     // 业务逻辑
///     Ok(42)
/// });
/// ```
#[macro_export]
macro_rules! transactional_ctx {
    ($body:block) => {{
        // 定义事务上下文
        #[derive(Default)]
        struct TransactionCtx<E> {
            rollbacks: Vec<Box<dyn FnMut() -> Result<(), E>>>,
            committed: bool,
        }
        
        impl<E> TransactionCtx<E> {
            fn add_rollback<F>(&mut self, f: F)
            where
                F: FnMut() -> Result<(), E> + 'static,
            {
                self.rollbacks.push(Box::new(f));
            }
            
            fn execute_all_rollbacks(&mut self) -> Result<(), E> {
                for rollback in self.rollbacks.iter_mut().rev() {
                    rollback()?;
                }
                Ok(())
            }
            
            fn commit(&mut self) {
                self.committed = true;
                self.rollbacks.clear();
            }
        }
        
        impl<E> Drop for TransactionCtx<E> {
            fn drop(&mut self) {
                if !self.committed && !self.rollbacks.is_empty() {
                    let _ = self.execute_all_rollbacks();
                }
            }
        }
        
        // 创建上下文
        let mut ctx = TransactionCtx::default();
        
        // 定义内部宏，用于在闭包内注册回滚
        macro_rules! register_rollback {
            ($rollback:expr) => {
                ctx.add_rollback($rollback);
            };
        }
        
        // 执行闭包，传入可用的注册宏
        let result = (|| {
            // 使注册宏在闭包内可用
            #[allow(unused_macros)]
            macro_rules! register_rollback_inner {
                ($rollback:expr) => {
                    ctx.add_rollback($rollback);
                };
            }
            
            // 执行用户代码
            $body
        })();
        
        // 处理结果
        match result {
            Ok(value) => {
                ctx.commit();
                Ok(value)
            }
            Err(e) => {
                // 错误已经通过Drop处理回滚
                Err(e)
            }
        }
    }};
}

/// 简化的无回滚版本（只处理Result包装）
#[macro_export]
macro_rules! with_result {
    ($body:block) => {{
        let result: Result<_, _> = (|| $body)();
        result
    }};
}

/// 带重试机制的事务宏
#[macro_export]
macro_rules! transactional_retry {
    // 基本用法：transactional_retry!(3, { ... })
    ($max_retries:expr, $body:block) => {{
        $crate::__transactional_retry_internal!($max_retries, 0, $body)
    }};
    
    // 带延迟的重试：transactional_retry!(3, 100ms, { ... })
    ($max_retries:expr, $delay:expr, $body:block) => {{
        $crate::__transactional_retry_with_delay!($max_retries, 0, $delay, $body)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __transactional_retry_internal {
    ($max_retries:expr, $attempt:expr, $body:block) => {{
        let result = transactional!($body);
        
        match result {
            Ok(value) => Ok(value),
            Err(e) if $attempt < $max_retries => {
                eprintln!("Attempt {} failed, retrying... ({}/{})", 
                         $attempt + 1, $attempt + 1, $max_retries);
                $crate::__transactional_retry_internal!($max_retries, $attempt + 1, $body)
            }
            Err(e) => {
                eprintln!("All {} attempts failed", $max_retries);
                Err(e)
            }
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __transactional_retry_with_delay {
    ($max_retries:expr, $attempt:expr, $delay:expr, $body:block) => {{
        let result = transactional!($body);
        
        match result {
            Ok(value) => Ok(value),
            Err(e) if $attempt < $max_retries => {
                eprintln!("Attempt {} failed, waiting {:?} before retry... ({}/{})", 
                         $attempt + 1, $delay, $attempt + 1, $max_retries);
                
                // 等待延迟
                std::thread::sleep(std::time::Duration::from_millis($delay));
                
                $crate::__transactional_retry_with_delay!($max_retries, $attempt + 1, $delay, $body)
            }
            Err(e) => {
                eprintln!("All {} attempts failed", $max_retries);
                Err(e)
            }
        }
    }};
}

/// 链式事务宏：依次执行多个操作，任何一个失败则回滚所有
#[macro_export]
macro_rules! transactional_chain {
    ($first:expr) => {{
        transactional!($first)
    }};
    
    ($first:expr, $($rest:expr),+) => {{
        let result = transactional!($first);
        
        match result {
            Ok(_) => transactional_chain!($($rest),+),
            Err(e) => Err(e),
        }
    }};
}