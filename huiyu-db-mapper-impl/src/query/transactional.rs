use crate::query::db_type_wrapper::DbTypeWrapper;
use futures_util::future::FutureExt;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::datasource::get_datasource_type;
use huiyu_db_mapper_core::sql::executor::Executor;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use tracing::error;

pub async fn transactional_exec<F, Fut, T>(func: F) -> Result<T, DatabaseError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, DatabaseError>>,
{
    let db_type = get_datasource_type()?;
    let db_wrapper: DbTypeWrapper = db_type.into();
    
    db_wrapper.transaction_basic_exec(
        async|| {
            do_transaction(db_wrapper,func).await
        }
    ).await
}

async fn do_transaction<F, Fut, T>(db_wrapper: DbTypeWrapper, func: F) -> Result<T, DatabaseError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, DatabaseError>>,
{
    // 开启事务
    if let Err(e) = db_wrapper.start_transaction().await {
        error!("start_transaction failed: {:?}", e);
        return Err(e);
    }

    // 在当前任务内捕获 panic（不丢失 task-local），把 panic 转为 Err 走回滚分支，
    // 避免 func() 直接 panic 时 unwind 跳过 match 导致事务无法回滚
    let result = AssertUnwindSafe(func()).catch_unwind().await;

    match result {
        Ok(Ok(value)) => {
            // 提交失败时也尝试回滚，避免事务悬挂（仍在 async 上下文，task-local 完好）
            if let Err(commit_err) = db_wrapper.commit().await {
                error!("commit failed: {:?}", commit_err);
                if let Err(rollback_err) = db_wrapper.rollback().await {
                    error!("rollback after commit failure failed: {}", rollback_err);
                }
                return Err(commit_err);
            }
            Ok(value)
        }
        Ok(Err(e)) => {
            // func 返回 Err：仍在 async 上下文，task-local 连接上下文完好，直接 await 回滚
            if let Err(rollback_err) = db_wrapper.rollback().await {
                error!("transaction rollback failed: {}", rollback_err);
            }
            Err(e)
        }
        Err(_) => {
            // func 发生了 panic：仍尝试在当前任务（task-local 完好）回滚正确连接
            if let Err(rollback_err) = db_wrapper.rollback().await {
                error!("transaction rollback after panic failed: {}", rollback_err);
            }
            Err(DatabaseError::CommonError("transaction closure panicked; rolled back".to_string()))
        }
    }
}