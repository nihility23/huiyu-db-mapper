use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::pool::datasource::get_datasource_type;
use crate::pool::db_manager::DbManager;
use crate::pool::transactional::get_transaction_id;
use crate::query::query_wrapper::QueryWrapper;
use r2d2::PooledConnection;
use r2d2_mysql::MySqlConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;

pub trait BaseMapperTx<E, Tx>
where
    E: Entity,
{
    type Tx;
    // select * from $table_name where $id = ?
    fn select_by_key(&self, tx: Self::Tx, key: &E::K) -> Result<Option<E>, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        let k = key.clone();
        // let param_vec:Vec<ParamValue> = vec![(*k).into()];
        // 判断数据库类型，获取数据库执行器
        // 获取数据库姓名

        let db_type_opt = get_datasource_type();
        let db_type = db_type_opt.ok_or(DatabaseError::NotFoundError(
            "DataSource Not config !!!".to_string(),
        ))?;

        let tx_id_opt = get_transaction_id();
        // 判断是否在事务里面，如果在事务里面则使用事务里面的数据库连接
        match db_type {
            // 获取当前数据库连接
            // 执行查询
            // 返回结果
            DbType::Mysql => {
                // 获取mysql数据库连接
                let conn: PooledConnection<MySqlConnectionManager> =
                    DbManager::get_instance().unwrap().get_conn()?;
            }
            DbType::Sqlite => {
                // 获取sqlite数据库连接
                let conn: PooledConnection<SqliteConnectionManager> =
                    DbManager::get_instance().unwrap().get_conn()?;
            }
            DbType::Oracle => {
                // 获取oracle数据库连接
            }
            DbType::Postgres => {
                // 获取postgres数据库连接
            }
            DbType::SqlServer => {
                // 获取sqlserver数据库连接
            }
        }

        Ok(None)
    }

    // select * from $table_name where $id in (?,...)
    fn select_by_keys(&self, tx: Self::Tx, keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError> {
        let sql = format!(
            "select * from {} where {} in ({})",
            E::table_name(),
            E::key_name(),
            vec!["?"; keys.len()].join(",")
        );
        // let param_vec = keys.iter().map(|key|<E as Entity>::K as Into<ParamValue>>::into(key));
        Ok(Vec::new())
    }

    // delete from $table_name where $id = ?
    fn delete_by_key(&self, tx: Self::Tx, key: &E::K) -> Result<u32, DatabaseError> {
        let sql = format!(
            "delete from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![key.into()];
        Ok(0)
    }

    // delete from $table_name where $id in (?,...)
    fn delete_by_keys(&self, tx: Self::Tx, keys: &Vec<E::K>) -> Result<u32, DatabaseError> {
        let sql = format!(
            "delete from {} where {} in ({})",
            E::table_name(),
            E::key_name(),
            vec!["?"; keys.len()].join(",")
        );
        // let param_vec = keys.iter().map(|key|key.into());
        Ok(0)
    }

    // update $table_name set $column_name = ? where id = ?
    fn update_by_key(&self, tx: Self::Tx, e: &E) -> Result<u32, DatabaseError> {
        let sql = format!(
            "update {} set {} where {} = ?",
            E::table_name(),
            "",
            E::key_name()
        );
        // let param_vec = vec![e.key().into()];
        Ok(0)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    fn insert(&self, tx: Self::Tx, entity: &E) -> Result<E::K, DatabaseError> {
        let sql = format!(
            "insert into {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![];
        Ok(E::new().key())
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    fn insert_batch(&self, tx: Self::Tx, entities: &Vec<E>) -> Result<u32, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![];
        Ok(0)
    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    fn select_page(
        &self,
        tx: Self::Tx,
        page: Page,
        query_wrapper: &QueryWrapper<E>,
    ) -> Result<PageRes<E>, DatabaseError> {
        // let param_vec = vec![key.into()];
        Ok(PageRes::new())
    }

    // select * from $table_name where $column = ? ...
    fn select(
        &self,
        tx: Self::Tx,
        query_wrapper: &QueryWrapper<E>,
    ) -> Result<Option<Vec<E>>, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // select * from $table_name where $column = ? ... limit 1
    fn select_one(
        &self,
        tx: Self::Tx,
        query_wrapper: &QueryWrapper<E>,
    ) -> Result<Option<E>, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![key.into()];
        Ok(None)
    }

    // update $table_name set $column_name = ? where $column = ? ...
    fn update(
        &self,
        tx: Self::Tx,
        entity: &E,
        query_wrapper: &QueryWrapper<E>,
    ) -> Result<u32, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![key.into()];
        // 获取当前数据库类型
        let db_type = DbType::Mysql;
        // 获取查询语句
        // <SqliteSqlGenerator as SqlGenerator>::gen_query_sql(&query_wrapper);
        Ok(0)
    }

    // delete from $table_name where $column = ? ...
    fn delete(&self, tx: Self::Tx, query_wrapper: &QueryWrapper<E>) -> Result<u32, DatabaseError> {
        let sql = format!(
            "select * from {} where {} = ?",
            E::table_name(),
            E::key_name()
        );
        // let param_vec = vec![key.into()];
        Ok(0)
    }
}
