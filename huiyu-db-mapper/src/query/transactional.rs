use crate::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::datasource::get_datasource_type;
use huiyu_db_mapper_core::sql::executor::Executor;

pub async fn transactional_exec<F, T, Fut>(func: F) -> Result<T, DatabaseError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output=Result<T, DatabaseError>>{
    let db_type = get_datasource_type()?;
    <DbType as Into<DbTypeWrapper>>::into(db_type).transactional_exec(func).await
}

