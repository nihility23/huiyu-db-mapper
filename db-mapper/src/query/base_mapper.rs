use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::exec_tx;
use crate::pool::datasource::get_datasource_type;
use crate::query::query_wrapper::QueryWrapper;
use crate::sql::sql_generator::{BaseSqlGenerator, QueryWrapperSqlGenerator};

#[allow(async_fn_in_trait)]
pub trait BaseMapper<E> where E: Entity{

    // select * from $table_name where $id = ?
    async fn select_by_key(&self, key: &E::K) -> Result<Option<E>,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_select_by_key_sql::<E>(key.clone());
        exec_tx!(db_type, sql.as_str(), &vec![param_vec], query_one)
    }

    // select * from $table_name where $id in (?,...)
    async fn select_by_keys(&self, keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_select_by_keys_sql::<E>(keys.clone());
        exec_tx!(db_type, sql.as_str(), &param_vec, query_some)
    }

    // delete from $table_name where $id = ?
    async fn delete_by_key(&self, key: &E::K) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_delete_by_key_sql::<E>(&key);
        exec_tx!(db_type, sql.as_str(), &vec![param_vec], delete)
    }

    // delete from $table_name where $id in (?,...)
    async fn delete_by_keys(&self, keys: &Vec<E::K>) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_delete_by_keys_sql::<E>(keys);
        exec_tx!(db_type, sql.as_str(), &param_vec, delete)
    }

    // update $table_name set $column_name = ? where id = ?
    async fn update_by_key(&self, e: &E) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_update_by_key_sql(e,false);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    async fn insert(&self, e: &E) -> Result<E::K,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_insert_one_sql(e);
        exec_tx!(db_type, sql.as_str(), &param_vec, E, insert)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    async fn insert_batch(&self, entities: &Vec<E>) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql,param_vec) = db_type.gen_insert_batch_sql(entities);
        exec_tx!(db_type, sql.as_str(), &param_vec, E, insert_batch)
    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    async fn select_page<'a>(&self, page: Page, query_wrapper: &QueryWrapper<'a,E>) -> Result<PageRes<E>,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (query_sql,total_sql,param_vec) = db_type.gen_page_sql(&page,query_wrapper);
        let p1 = param_vec.clone();
        let total = exec_tx!(db_type, total_sql.as_str(), &param_vec, query_count)?;
        let list = exec_tx!(db_type, query_sql.as_str(), &p1, query_some)?;
        Ok(PageRes::new_from_records(total,page.page_size,list))
    }

    // select * from $table_name where $column = ? ...
    async fn select<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<Vec<E>,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (query_sql,param_vec) = db_type.gen_query_sql(query_wrapper);
        exec_tx!(db_type, query_sql.as_str(), &param_vec, query_some)
    }

    // select * from $table_name where $column = ? ... limit 1
    async fn select_one<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<Option<E>,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (query_sql,param_vec) = db_type.gen_query_sql(query_wrapper);
        exec_tx!(db_type, query_sql.as_str(), &param_vec, query_one)
    }

    // update $table_name set $column_name = ? where $column = ? ...
    async fn update<'a>(&self, entity: &E, query_wrapper: &QueryWrapper<'a,E>) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql, param_vec) = db_type.gen_update_sql(entity, query_wrapper,false);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    async fn update_with_null<'a>(&self, entity: &E, query_wrapper: &QueryWrapper<'a,E>) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql, param_vec) = db_type.gen_update_sql(entity, query_wrapper,true);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    // delete from $table_name where $column = ? ...
    async fn delete<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<u64,DatabaseError>{
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError("datasource type is null".to_string()))?;
        let (sql, param_vec) = db_type.gen_delete_sql(query_wrapper);
        exec_tx!(db_type, sql.as_str(), &param_vec, delete)
    }
}
