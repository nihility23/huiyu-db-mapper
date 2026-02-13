use std::cmp::PartialEq;
use crate::base::db_type::DbType;
use crate::base::entity::{Entity, KeyGenerateType};
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::base::param::ParamValue;
use crate::exec_tx;
use crate::pool::datasource::get_datasource_type;
use crate::query::query_wrapper::QueryWrapper;
use crate::sql::sql_generator::{BaseSqlGenerator, QueryWrapperSqlGenerator};

#[allow(async_fn_in_trait)]
pub trait BaseMapper<E>
where
    E: Entity,
{
    // select * from $table_name where $id = ?
    async fn select_by_key(&self, key: &E::K) -> Result<Option<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_select_by_key_sql::<E>(key.clone());
        exec_tx!(db_type, sql.as_str(), &vec![param_vec.clone()], query_one)
    }

    // select * from $table_name where $id in (?,...)
    async fn select_by_keys(&self, keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_select_by_keys_sql::<E>(keys.clone());
        exec_tx!(db_type, sql.as_str(), &param_vec, query_some)
    }

    // delete from $table_name where $id = ?
    async fn delete_by_key(&self, key: &E::K) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_delete_by_key_sql::<E>(&key);
        exec_tx!(db_type, sql.as_str(), &vec![param_vec.clone()], delete)
    }

    // delete from $table_name where $id in (?,...)
    async fn delete_by_keys(&self, keys: &Vec<E::K>) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_delete_by_keys_sql::<E>(keys);
        exec_tx!(db_type, sql.as_str(), &param_vec, delete)
    }

    // update $table_name set $column_name = ? where id = ?
    async fn update_by_key(&self, e: &E) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_update_by_key_sql(e, false);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    async fn insert(&self, e: &mut E) -> Result<Option<E::K>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;

        let key_info = E::key_info();
        if key_info.is_none() {
            let (sql, param_vec) = db_type.gen_insert_one_sql(e);
            return exec_tx!(db_type, sql.as_str(), &param_vec, E, insert);
        }
        let key_info = key_info.unwrap();
        let mut key = None;
        let key_generate_type = key_info.key_generate_type;

        // 有自增
        if key_info.is_auto_increment{
            let (sql, param_vec) = db_type.gen_insert_and_get_id_sql(e);
            return exec_tx!(db_type, sql.as_str(), &param_vec, E, insert);
        }

        // 无自增:uuid
        if key_generate_type == KeyGenerateType::UUID {
            let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
            e.set_value_by_column_name(key_info.column_name, uuid.clone().into());
            let (sql, param_vec) = db_type.gen_insert_one_sql(e);
            exec_tx!(db_type, sql.as_str(), &param_vec, E, insert);
            key = Some(ParamValue::String(uuid).into());
        }
        Ok(key)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    async fn insert_batch(&self, entities: &Vec<E>) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_insert_batch_sql(entities);
        exec_tx!(db_type, sql.as_str(), &param_vec, E, insert_batch)
    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    async fn select_page<'a>(
        &self,
        page: Page,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<PageRes<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (query_sql, total_sql, param_vec) = db_type.gen_page_sql(&page, query_wrapper);
        let p1 = param_vec.clone();
        let total = exec_tx!(db_type, total_sql.as_str(), &param_vec, query_count)?;
        let list = exec_tx!(db_type, query_sql.as_str(), &p1, query_some)?;
        Ok(PageRes::new_from_records(total, page.page_size, list))
    }

    // select * from $table_name where $column = ? ...
    async fn select<'a>(
        &self,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<Vec<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (query_sql, param_vec) = db_type.gen_query_sql(query_wrapper);
        exec_tx!(db_type, query_sql.as_str(), &param_vec, query_some)
    }

    // select * from $table_name where $column = ? ... limit 1
    async fn select_one<'a>(
        &self,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<Option<E>, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (query_sql, param_vec) = db_type.gen_query_sql(query_wrapper);
        exec_tx!(db_type, query_sql.as_str(), &param_vec, query_one)
    }

    // update $table_name set $column_name = ? where $column = ? ...
    async fn update<'a>(
        &self,
        entity: &E,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_update_sql(entity, query_wrapper, false);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    async fn update_with_null<'a>(
        &self,
        entity: &E,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_update_sql(entity, query_wrapper, true);
        exec_tx!(db_type, sql.as_str(), &param_vec, update)
    }

    // delete from $table_name where $column = ? ...
    async fn delete<'a>(&self, query_wrapper: &QueryWrapper<'a, E>) -> Result<u64, DatabaseError> {
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_delete_sql(query_wrapper);
        exec_tx!(db_type, sql.as_str(), &param_vec, delete)
    }
}
