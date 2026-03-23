use crate::base::db_type::DbType;
use crate::base::error::DatabaseError;
use crate::pool::datasource::get_datasource_type;

#[allow(async_fn_in_trait)]
pub trait TransactionalExecutor {

    async fn exec<F,P,BF,Fut,T>(f: F, bf: BF) -> Result<T, DatabaseError>
    where
        F: FnOnce(DbType) -> P,
        BF: FnOnce(P) -> Fut ,  // BF 返回 Future
        Fut: Future<Output = Result<T, DatabaseError>>,

    {
        let db_type = get_datasource_type()?;
        let p = f(db_type);
        bf(p).await  // 直接 await 异步函数
    }
    
    // async fn transactional_exec_basic<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    // where
    //     F: FnOnce() -> Fut ,
    //     Fut: Future<Output = Result<T, DatabaseError>>{
    // 
    //     self.start_transaction().await?;
    //     let res = func().await;
    //     if res.is_err() {
    //         self.rollback().await?;
    //     }else {
    //         self.commit().await?;
    //     }
    //     res
    // 
    // }
    // 
    // async fn transactional_exec<F, T, Fut>(&self, _func: F) -> Result<T, DatabaseError>
    // where
    //     F: FnOnce() -> Fut ,  // BF 返回 Future
    //     Fut: Future<Output = Result<T, DatabaseError>>,{
    //     Err(DatabaseError::NotSupportedError("transaction_exec".to_string()))
    // }
} 