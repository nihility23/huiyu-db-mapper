use huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::datasource::get_datasource_type;
use huiyu_db_mapper_core::sql::executor::Executor;
use crate::query::db_type_wrapper::DbTypeWrapper;

pub async fn exec_script(sql: &str) ->Result<(),DatabaseError>{
    let db_type = get_datasource_type()?;
    <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql, &vec![]).await?;
    Ok(())
}