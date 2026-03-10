use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::{Executor, RowType};

// use crate::base::entity::Entity;
// use crate::base::error::DatabaseError;
// use crate::base::param::ParamValue;
// use crate::sql::executor::Executor;
// use std::sync::{Arc, Mutex};
// use tokio::task_local;
//
// task_local! {
//     // pub static TX_REGISTER : Arc<Mutex<Transaction>>;
// }
pub const MYSQL_SQL_EXECUTOR: MysqlSqlExecutor = MysqlSqlExecutor;

pub struct MysqlRowType;
impl RowType for MysqlRowType {
    fn col_to_v_by_index(&self, index: usize) -> Result<ParamValue, DatabaseError> {
        todo!()
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        todo!()
    }
}
pub struct MysqlSqlExecutor;
//
impl Executor for MysqlSqlExecutor{
    type Row<'a> = MysqlRowType;

    async fn exec_basic(sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    async fn query_basic<T, R, F, Q>(&self, sql: String, params: Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        todo!()
    }

    fn row_to_e<'a, E>(row: &Self::Row<'a>) -> Result<E, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        todo!()
    }
}
//
