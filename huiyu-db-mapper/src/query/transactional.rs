use crate::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::datasource::get_datasource_type;
use huiyu_db_mapper_core::sql::executor::Executor;
use std::future::Future;

pub async fn transactional_exec<F, Fut, T>(func: F) -> Result<T, DatabaseError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, DatabaseError>>,
{
    let db_type = get_datasource_type()?;

    // 开始事务
    let db_wrapper: DbTypeWrapper = db_type.into();
    let mut guard = TransactionGuard {
        db_wrapper,
        rollback_needed: true,
    };
    let res = db_wrapper.start_transaction().await;
    if res.is_err() {
        guard.rollback_needed = false;
        return Err(res.err().unwrap());
    }
    // 执行用户函数
    let result = func().await;

    // 根据结果决定是否提交
    match result {
        Ok(value) => {
            guard.db_wrapper.commit().await?;
            guard.rollback_needed = false;
            Ok(value)
        }
        Err(e) => {
            // 不需要在这里回滚事务，因为 guard 的 Drop 实现会自动执行回滚
            Err(e)
        }
    }
}

struct TransactionGuard {
    db_wrapper: DbTypeWrapper,
    rollback_needed: bool,
}

impl Drop for TransactionGuard {
    fn drop(&mut self) {
        // 在 Drop 实现中，我们不能直接使用 await，所以我们需要使用 tokio::spawn
        if self.rollback_needed {
            let db_wrapper = self.db_wrapper.clone();
            tokio::spawn(async move {
                if let Err(e) = db_wrapper.rollback().await {
                    eprintln!("回滚失败: {}", e);
                }
            });
        }
    }
}