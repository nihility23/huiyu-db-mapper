use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::query::query_wrapper::QueryWrapper;

pub trait BaseMapper<E> where E: Entity{

    // select * from $table_name where $id = ?
    fn select_by_key(&self, key: &E::K) -> Result<Option<E>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // select * from $table_name where $id in (?,...)
    fn select_by_keys(&self, keys: &Vec<E::K>) -> Result<Vec<E>,DatabaseError>{
        let sql = format!("select * from {} where {} in ({})", E::table_name(),E::key_name(),vec!["?";keys.len()].join(","));
        // let param_vec = keys.iter().map(|key|<&<E as Entity>::K as Into<T>>::into(key));
        Ok(Vec::new())
    }

    // delete from $table_name where $id = ?
    fn delete_by_key(&self, key: &E::K) -> Result<u32,DatabaseError>{
        let sql = format!("delete from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(0)
    }

    // delete from $table_name where $id in (?,...)
    fn delete_by_keys(&self, keys: &Vec<E::K>) -> Result<u32,DatabaseError>{
        let sql = format!("delete from {} where {} in ({})", E::table_name(),E::key_name(),vec!["?";keys.len()].join(","));
        // let param_vec = keys.iter().map(|key|key.into());
        Ok(0)
    }

    // update $table_name set $column_name = ? where id = ?
    fn update_by_key(&self, e: &E) -> Result<u32,DatabaseError>{
        let sql = format!("update {} set {} where {} = ?", E::table_name(),"",E::key_name());
        // let param_vec = vec![e.key().into()];
        Ok(0)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    fn insert(&self, entity: &E) -> Result<E::K,DatabaseError>{
        let sql = format!("insert into {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![];
        Ok(E::new().key())
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    fn insert_batch(&self, entities: &Vec<E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![];
        Ok(0)
    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    fn select_page(&self, page: Page, query_wrapper: &QueryWrapper<E>) -> Result<PageRes<E>,DatabaseError>{

        // let param_vec = vec![key.into()];
        Ok(PageRes::new())
    }

    // select * from $table_name where $column = ? ...
    fn select(&self, query_wrapper: &QueryWrapper<E>) -> Result<Option<Vec<E>>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // select * from $table_name where $column = ? ... limit 1
    fn select_one(&self, query_wrapper: &QueryWrapper<E>) -> Result<Option<E>,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // update $table_name set $column_name = ? where $column = ? ...
    fn update(&self, entity: &E, query_wrapper: &QueryWrapper<E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        // 获取当前数据库类型
        let db_type = DbType::Mysql;
        // 获取查询语句
        // <SqliteSqlGenerator as SqlGenerator>::gen_query_sql(&query_wrapper);
        Ok(0)
    }

    // delete from $table_name where $column = ? ...
    fn delete(&self, query_wrapper: &QueryWrapper<E>) -> Result<u32,DatabaseError>{
        let sql = format!("select * from {} where {} = ?", E::table_name(),E::key_name());
        // let param_vec = vec![key.into()];
        Ok(0)
    }
}
