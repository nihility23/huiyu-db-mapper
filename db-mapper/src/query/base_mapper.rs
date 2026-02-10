use r2d2_mysql::MySqlConnectionManager;
use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::base::param::ParamValue;
use crate::exec_tx;
use crate::query::query_wrapper::QueryWrapper;

pub trait BaseMapper<E> where E: Entity{

    // select * from $table_name where $id = ?
     async fn select_by_key(&self, key: &E::K) -> Result<Option<E>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        let param:ParamValue = (key.clone()).into();

        exec_tx!(sql.as_str(), &vec![param],query_one)
    }

    // select * from $table_name where $id in (?,...)
    async fn select_by_keys(&self, keys: &Vec<E::K>) -> Result<Vec<E>,DatabaseError>{
        let sql = format!("select * from {} where {} in ({})", E::table_name(),E::key_name(),vec!["?";keys.len()].join(","));
        let param_vec:Vec<ParamValue> = keys.iter().map(|key| (key.clone()).into()).collect();
        exec_tx!(sql.as_str(), &param_vec,query_some)
    }

    // delete from $table_name where $id = ?
    async fn delete_by_key(&self, key: &E::K) -> Result<u64,DatabaseError>{
        let sql = format!("delete from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        exec_tx!(sql.as_str(), &vec![key.clone().into()],delete)
    }

    // delete from $table_name where $id in (?,...)
    async fn delete_by_keys(&self, keys: &Vec<E::K>) -> Result<u64,DatabaseError>{
        let sql = format!("delete from {} where {} in ({})", E::table_name(),E::key_name(),vec!["?";keys.len()].join(","));
        let param_vec:Vec<ParamValue> = keys.iter().map(|key| (key.clone()).into()).collect();
        exec_tx!(sql.as_str(), &param_vec,delete)
    }

    // update $table_name set $column_name = ? where id = ?
    async fn update_by_key(&self, e: &E) -> Result<u64,DatabaseError>{
        let sql = format!("update {} set {} where {} = ?", E::table_name(),"",E::key_name());
        // let param_vec = vec![e.into()];
        Ok(0)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    async fn insert(&self, entity: &E) -> Result<E::K,DatabaseError>{
        let sql = format!("insert into {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![];
        Ok(E::new().key())
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    async fn insert_batch(&self, entities: &Vec<E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![];
        Ok(0)
    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    async fn select_page<'a>(&self, page: Page, query_wrapper: &QueryWrapper<'a,E>) -> Result<PageRes<E>,DatabaseError>{

        // let param_vec = vec![key.into()];
        Ok(PageRes::new())
    }

    // select * from $table_name where $column = ? ...
    async fn select<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<Option<Vec<E>>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // select * from $table_name where $column = ? ... limit 1
    async fn select_one<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<Option<E>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // update $table_name set $column_name = ? where $column = ? ...
    async fn update<'a>(&self, entity: &E, query_wrapper: &QueryWrapper<'a,E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        // 获取当前数据库类型
        let db_type = DbType::Mysql;
        // 获取查询语句
        // <SqliteSqlGenerator as SqlGenerator>::gen_query_sql(&query_wrapper);
        Ok(0)
    }

    // delete from $table_name where $column = ? ...
    async fn delete<'a>(&self, query_wrapper: &QueryWrapper<'a,E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(0)
    }
}
