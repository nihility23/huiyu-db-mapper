use crate::base::db_type::DbType;
use crate::base::entity::{Entity, KeyGenerateType};
use crate::base::error::DatabaseError;
use crate::base::page::{Page, PageRes};
use crate::base::param::ParamValue;
use crate::pool::datasource::get_datasource_type;
use crate::query::query_wrapper::QueryWrapper;
use crate::sql::executor::Executor;
use crate::sql::sql_generator::{BaseSqlGenerator, QueryWrapperSqlGenerator};
use rustlog::info;

async fn exec<E,F,P,BF,Fut,T>(f: F, bf: BF) -> Result<T, DatabaseError>
where
    F: FnOnce(DbType) -> P,
    BF: FnOnce(P) -> Fut + Send,  // BF 返回 Future
    Fut: Future<Output = Result<T, DatabaseError>> + Send,
    T: Send + 'static,
    P: Send + 'static,
{
    let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
        "datasource type is null".to_string(),
    ))?;
    let p = f(db_type);
    bf(p).await  // 直接 await 异步函数
}

#[allow(async_fn_in_trait)]
pub trait BaseMapper<E>
where
    E: Entity,
{
    // select * from $table_name where $id = ?
    async fn select_by_key(key: &E::K) -> Result<Option<E>, DatabaseError> {
        let k = key.clone();
        exec::<E, _,_,_,_, Option<E>>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_select_by_key_sql::<E>(k);
            (db_type,sql,param_vec)
        },async move|(db_type,sql,param_vec)| {
            db_type.query_one(sql.as_str(),&vec![param_vec.clone()]).await
        }).await
    }

    // select * from $table_name where $id in (?,...)
    async fn select_by_keys(keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError> {
        let ks = keys.clone();
        exec::<E, _,_,_,_, Vec<E>>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_select_by_keys_sql::<E>(ks);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.query_some(sql.as_str(),&param_vec).await
        }).await
    }

    // delete from $table_name where $id = ?
    async fn delete_by_key(key: &E::K) -> Result<u64, DatabaseError> {
        let k = key.clone();
        exec::<E, _,_,_,_, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_delete_by_key_sql::<E>(&k);
            (db_type,sql,param_vec)
        }, async |(db_type,sql,param_vec)|{
            db_type.delete(sql.as_str(),&vec![param_vec.clone()]).await    
        }).await
    }

    // delete from $table_name where $id in (?,...)
    async fn delete_by_keys(keys: &Vec<E::K>) -> Result<u64, DatabaseError> {
        let ks = keys.clone();
        exec::<E, _,_,_,_, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_delete_by_keys_sql::<E>(&ks);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.delete(sql.as_str(),&param_vec).await
        }).await
    }

    // update $table_name set $column_name = ? where id = ?
    async fn update_by_key(e: &E) -> Result<u64, DatabaseError> {
        let e = e.clone();
        exec::<E, _,_,_,_, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_update_by_key_sql::<E>(&e,false);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.update(sql.as_str(),&param_vec).await
        }).await
    }

    // insert $table_name into ($id,$column,...) values (?,?,...)
    async fn insert(e: &mut E) -> Result<Option<E::K>, DatabaseError> {

        let key_info = E::key_info();
        if key_info.is_none() {
            exec::<E, _,_,_, _, Option<E::K>>(|db_type: DbType|{
                let (sql, param_vec) = db_type.gen_insert_one_sql::<E>(&e);
                (db_type,sql,param_vec)
            },async |(db_type,sql,param_vec)|{
                db_type.insert::<E>(sql.as_str(),&param_vec).await
            }).await;
        }
        let key_info = key_info.unwrap();
        let mut key = None;
        let key_generate_type = key_info.key_generate_type;

        // 有自增
        if key_info.is_auto_increment{
            return exec::<E, _,_,_, _, Option<E::K>>(|db_type: DbType|{
                let (sql, param_vec) = db_type.gen_insert_and_get_id_sql::<E>(&e);
                (db_type,sql,param_vec)
            },async |(db_type,sql,param_vec)|{
                db_type.insert::<E>(sql.as_str(),&param_vec).await
            }).await;
        }

        // 无自增:uuid
        if key_generate_type == KeyGenerateType::UUID {
            let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
            e.set_value_by_column_name(key_info.column_name, uuid.clone().into());

            exec::<E, _,_,_, _, Option<E::K>>(|db_type: DbType|{
                let (sql, param_vec) = db_type.gen_insert_and_get_id_sql::<E>(&e);
                (db_type,sql,param_vec)
            },async |(db_type,sql,param_vec)|{
                db_type.insert::<E>(sql.as_str(),&param_vec).await
            }).await;
            key = Some(ParamValue::String(uuid).into());
        }
        Ok(key)
    }

    // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
    async fn insert_batch(entities: Vec<E>) -> Result<u64, DatabaseError> {
        // let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
        //     "datasource type is null".to_string(),
        // ))?;
        // let (sql, param_vec) = db_type.gen_insert_batch_sql(entities);
        // exec_tx!(db_type, sql.as_str(), &param_vec, E, insert_batch)

        // exec::<E, _, u64>(|db_type: DbType|{
        //     let (sql, param_vec) = db_type.gen_insert_batch_sql::<E>(entities);
        //     db_type.insert_batch::<E>(sql.as_str(),&param_vec)
        // }).await
        exec::<E, _,_,_, _, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_insert_batch_sql::<E>(&entities);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.insert_batch::<E>(sql.as_str(),&param_vec).await
        }).await

    }

    // select count(*) from (select * from $table_name where $column = ? ...)
    // select * from $table_name where $column = ? ... limit ?,?
    async fn select_page<'a>(
        page: Page,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<PageRes<E>, DatabaseError> {
        exec::<E, _,_,_, _, PageRes<E>>(|db_type: DbType|{
            let (query_sql, total_sql, param_vec) = db_type.gen_page_sql::<E>(&page, query_wrapper);
            (db_type,query_sql,total_sql,param_vec,page.page_size)
        },async |(db_type,query_sql,total_sql,param_vec,page_size)|{
            let total = db_type.query_count(total_sql.as_str(), &param_vec).await?;
            let list = db_type.query_some(query_sql.as_str(), &param_vec).await?;
            Ok(PageRes::new_from_records(total, page_size, list))
        }).await
    }

    // select * from $table_name where $column = ? ...
    async fn select<'a>(
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<Vec<E>, DatabaseError> {
        exec::<E, _,_,_, _, Vec<E>>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_query_sql::<E>(query_wrapper);
            info!("sql: {}, param_vec: {:?}", sql, param_vec);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.query_some(sql.as_str(),&param_vec).await
        }).await
    }

    // select * from $table_name where $column = ? ... limit 1
    async fn select_one<'a>(
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<Option<E>, DatabaseError> {
        exec::<E, _,_,_, _, Option<E>>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_query_sql::<E>(query_wrapper);
            info!("sql: {}, param_vec: {:?}", sql, param_vec);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.query_one(sql.as_str(),&param_vec).await
        }).await

    }

    // update $table_name set $column_name = ? where $column = ? ...
    async fn update<'a>(
        e: &E,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<u64, DatabaseError> {

        exec::<E, _,_,_, _, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_update_sql::<E>(&e, query_wrapper, false);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.update(sql.as_str(),&param_vec).await
        }).await
    }

    async fn update_with_null<'a>(
        &self,
        e: &E,
        query_wrapper: &QueryWrapper<'a, E>,
    ) -> Result<u64, DatabaseError> {
        exec::<E, _,_,_, _, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_update_sql::<E>(e, query_wrapper, true);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.update(sql.as_str(),&param_vec).await
        }).await
    }

    // delete from $table_name where $column = ? ...
    async fn delete<'a>(query_wrapper: &QueryWrapper<'a, E>) -> Result<u64, DatabaseError> {
        exec::<E, _,_,_, _, u64>(|db_type: DbType|{
            let (sql, param_vec) = db_type.gen_delete_sql::<E>(query_wrapper);
            (db_type,sql,param_vec)
        },async |(db_type,sql,param_vec)|{
            db_type.delete(sql.as_str(),&param_vec).await
        }).await
    }
}
