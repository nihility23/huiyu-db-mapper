use crate::base::entity::{Entity};

use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

use tracing::error;
use std::option::Option;
use std::sync::Arc;

pub trait RowType{
    fn col_to_v_by_index(&self, col_index: usize, ) -> Result<ParamValue, DatabaseError> where Self: Sized ;

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError> where Self: Sized ;
}

#[allow(async_fn_in_trait)]
pub trait Executor{
    type Row<'a>: RowType + 'a;
    type Conn;

    async fn query<T, R, F, Q>(
        &self,
        conn: Arc<parking_lot::Mutex<Self::Conn>>,
        sql: &str,
        params: &Vec<ParamValue>,
        mapper: F,
        processor: Q,
    ) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static;
    async fn execute(
        &self,
        conn: Arc<parking_lot::Mutex<Self::Conn>>,
        sql: &str,
        params: &Vec<ParamValue>,
    ) -> Result<u64, DatabaseError>;


    async fn exec_basic(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let conn_ref = self.get_conn_ref();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            self.execute(conn_ref, sql, params).await
        } else {
            let conn: Self::Conn = self.get_conn().await?;
            self.execute(Arc::new(parking_lot::Mutex::new(conn)), sql, params).await
        }
    }


    async fn query_basic<T, R, F, Q>(
        &self,
        sql: &str,
        params: &Vec<ParamValue>,
        mapper: F,
        processor: Q,
    ) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static{

        let conn_ref = self.get_conn_ref();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            self.query(conn_ref, sql, params, mapper, processor).await // 现在可以借用
        } else {
            let conn = self.get_conn().await?;
            self.query(Arc::new(parking_lot::Mutex::new(conn)), sql, params, mapper, processor).await
        }
    }

    fn row_to_e<E>(row: &Self::Row<'_>) -> Result<E, DatabaseError> where E:Entity{
        let mut e = E::new();
        for col in E::column_names() {
            let val = row.col_to_v_by_name(col)?;
            e.set_value_by_column_name(col, val);
        }
        Ok(e)
    }

    fn get_conn_ref(&self)-> Result<Arc<parking_lot::Mutex<Self::Conn>>,DatabaseError> ;

    async fn get_conn(&self)-> Result<Self::Conn,DatabaseError>;

    async fn query_some<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity{
        self.query_basic::<E, Vec<E>, _, _>(sql, params, |row|Self::row_to_e(row), |results: Vec<E>| {
            Ok(results)
        }).await
    }

    // 查询单个结果
    async fn query_one<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity{
        {
            self.query_basic::<E, Option<E>, _, _>(sql, params, |row|Self::row_to_e(row), |results: Vec<E>| {
                Ok(results.into_iter().next())
            }).await
        }
    }

    async fn query_count(&self, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>{
        self.query_basic::<i64, u64, _, _>(
            sql,
            params,
            |row| {
                let v = (row).col_to_v_by_index(0).unwrap();
                Ok(v.into())
            },
            |results: Vec<i64>| Ok(results[0] as u64),
        ).await
    }
    // 执行插入操作，返回主键
    async fn insert<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E::K>,DatabaseError>where E:Entity{
        self.query_basic::<ParamValue, Option<E::K>, _, _>(
            sql,
            params,
            |row| {
                let val = (row).col_to_v_by_index(0);
                return match val {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        error!("Error: {}", e);
                        Ok(ParamValue::Null)
                    },
                }
            },
            |results: Vec<ParamValue>| {
                if results.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(results[0].clone().into()))
                }
            },
        ).await
    }

     async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity,
    {
        self.exec_basic(sql, params).await
    }

    async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        self.exec_basic(sql, params).await
    }

    async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        self.exec_basic(sql, params).await
    }

    async fn start_transaction(&self)->Result<(),DatabaseError>{
        Err(DatabaseError::NotSupportedError("start_transaction".to_string()))
    }
    async fn commit(&self)->Result<(),DatabaseError>{
        Err(DatabaseError::NotSupportedError("commit".to_string()))
    }
    async fn rollback(&self)->Result<(),DatabaseError>{
        Err(DatabaseError::NotSupportedError("rollback".to_string()))
    }

    async fn transactional_exec_basic<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut ,
        Fut: Future<Output = Result<T, DatabaseError>>{
            self.start_transaction().await?;
            let res = func().await;
            if res.is_err() {
                self.rollback().await?;
            }else {
                self.commit().await?;
            }
            res
    }

    async fn transactional_exec<F, T, Fut>(&self, _func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut ,  // BF 返回 Future
        Fut: Future<Output = Result<T, DatabaseError>>,{
        Err(DatabaseError::NotSupportedError("transaction_exec".to_string()))
    }

}

#[macro_export]
macro_rules! with_conn_scope {
    // 指定注册器、self、func
    ($register:expr, $self:expr, $func:expr) => {{
        use std::sync::Arc;
        use parking_lot::Mutex;
        
        let conn = $self.get_conn().await?;
        $register.scope(Arc::new(Mutex::new(conn)), async {
            $self.transactional_exec_basic($func).await
        }).await
    }};
}
