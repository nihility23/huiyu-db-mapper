use crate::base::error::{DatabaseError, RowError};
use crate::base::param::ParamValue;
use r2d2::Pool;
use crate::base::entity::Entity;

pub(crate) struct SqlExecutor<T: r2d2::ManageConnection> {
    pub(crate) pool: Pool<T>
}

pub(crate) trait Executor{
    type R<'a>;
    fn get_sql_executor()->&'static Self;

    fn row_to_entity<'a, E>(row :&Self::R<'_>)->Result<E,RowError> where E:Entity;
    fn exec<E>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;


}

// impl DbManager<SqliteConnectionManager> {
    // fn get_db_manager() -> &'static Self {
    //     static INSTANCE: OnceLock<DbManager> = OnceLock::new();
    //     INSTANCE.get_or_init(|| {
    //         let manager = SqliteConnectionManager::file(format!("{}/{}",Config::get_config().basic_config.basic_dir,DATABASE_URL));
    //         let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
    //         DbManager { pool }
    //     })
    // }
    //
    // pub(crate) fn init_db() -> Result<(),CustomError> {
    //     Self::create_db()
    // }
    //
    // pub(crate) fn create_db()->Result<(), CustomError>{
    //     let conn_res = Self::get_db_manager().pool.get();
    //     if conn_res.is_err() {
    //         return Err(CustomError::DatabaseError("获取连接错误".to_string()))
    //     }
    //     let conn = conn_res.unwrap();
    //     let create_table_res = conn.execute(CREATE_TABLE_FILE_SQL, params![]);
    //     if create_table_res.is_err(){
    //         error!("{}",create_table_res.err().unwrap().to_string())
    //     }
    //     let create_table_res = conn.execute(CREATE_TABLE_APP_SQL, params![]);
    //     if create_table_res.is_err(){
    //         error!("{}",create_table_res.err().unwrap().to_string())
    //     }
    //     let create_table_res = conn.execute(CREATE_TABLE_USER_SQL, params![]);
    //     if create_table_res.is_err(){
    //         error!("{}",create_table_res.err().unwrap().to_string())
    //     }
    //     Ok(())
    // }
    //
    // pub(crate) fn query_by_id<T,F>(table_name:&str,id:u64,f:F)->Result<Option<T>,CustomError> where F:FnMut(&Row<'_>) -> rusqlite::Result<T>,T:Serialize{
    //     let mut param_vec = Vec::new();
    //     param_vec.push(ParamValue::U64(id));
    //     DbManager::query_one(format!("select * from {} where id = ?",&table_name).as_str(), &param_vec, f, |t:&mut Vec<T>|{
    //         Ok(Some(t.pop().unwrap()))
    //     })
    // }
    //
    // pub(crate) fn delete_by_id(table_name:&str,id:u64)->Result<(),CustomError>{
    //     let mut param_vec = Vec::new();
    //     param_vec.push(ParamValue::U64(id));
    //     DbManager::exec(format!("delete from {} where id = ?",&table_name).as_str(), &param_vec)
    // }
    //
    // pub(crate) fn query_page<P,QF,T,F>(base_query_sql: &str,current_page:u32, page_size:u32, p: &P, qf:QF,f:F)->Result<PageResult<T>,CustomError> where P:Serialize,QF:Fn(&P)->(Vec<String>,Vec<ParamValue>),T:Serialize + std::fmt::Debug,F: FnMut(&Row<'_>) -> rusqlite::Result<T>{
    //     let (mut query_param_name_vec, mut query_param_value_vec) = qf(&p);
    //
    //     let mut query_param_sql = String::new();//format!("{} {}"," where " ,query_param_name_vec.join(" and "));
    //     if query_param_value_vec.len()>0{
    //         query_param_sql.push_str(" where ");
    //         query_param_sql.push_str(&query_param_name_vec.join(" and "));
    //     }
    //
    //     let total_page_sql = format!("select count(*) from ({} {})",base_query_sql , query_param_sql.as_str() );
    //     info!("querye_page total count sql : {}",total_page_sql);
    //     // 查询总数
    //     let count_res = Self::query_count(&total_page_sql, &query_param_value_vec);
    //     if count_res.is_err(){
    //         return Err(CustomError::DatabaseError(count_res.unwrap_err().to_string()))
    //     }
    //
    //     let mut page_sql = base_query_sql.to_string() + query_param_sql.as_str() + " order by create_time desc";
    //     page_sql.push_str(" limit ? offset ?");
    //     query_param_value_vec.push(ParamValue::U32(page_size));
    //     query_param_value_vec.push(ParamValue::U32((current_page-1) * page_size));
    //
    //     info!("query_page page sql : {}",page_sql);
    //     let rows_res = Self::query_vec(&page_sql,&query_param_value_vec,f);
    //     if rows_res.is_err(){
    //         let rows_err = rows_res.unwrap_err();
    //         return Err(CustomError::DatabaseError(format!("Query file sql error: {}",rows_err)));
    //     }
    //
    //     let rows = rows_res.unwrap();
    //     let page_res = PageResult::new(current_page,page_size,count_res.unwrap(),rows);
    //     Ok(page_res)
    // }
    //
    //
    // pub(crate) fn query_count(sql:&str, params: &Vec<ParamValue>)->Result<u64,CustomError> {
    //     let res = Self::query_one(sql,params,|row|{
    //         Ok(row.get::<_,u64>(0)?)
    //     },|t|{
    //         Ok(Some(*t.get(0).unwrap() as u64))
    //     });
    //     if res.is_err(){
    //         return Err(res.err().unwrap());
    //     }
    //     return Ok(res.unwrap().unwrap());
    // }
    //
    // pub(crate) fn insert_one(sql:&str, params: &Vec<ParamValue>)->Result<u64,CustomError>{
    //     let res = Self::query_one(sql,params,|row|{
    //         Ok(row.get::<_,u64>(0)?)
    //     },|t|{
    //         Ok(Some(*t.get(0).unwrap() as u64))
    //     });
    //
    //     if res.is_err(){
    //         return Err(res.err().unwrap());
    //     }
    //     return Ok(res.unwrap().unwrap());
    // }
    //
    // pub(crate) fn query_vec<F,T>(sql:&str, params: &Vec<ParamValue>, f:F ) -> Result<Vec<T>,CustomError> where T:Serialize, F: FnMut(&Row<'_>) -> rusqlite::Result<T> {
    //     let conn_res = Self::get_db_manager().pool.get();
    //     if conn_res.is_err(){
    //         return Err(BusinessError("Get connection error".to_string()))
    //     }
    //     let conn = conn_res.unwrap();
    //     let mut stmt_res = conn.prepare(sql);
    //     if stmt_res.is_err(){
    //         return Err(CustomError::DatabaseError(stmt_res.err().unwrap().to_string()))
    //     }
    //     let param_vec:Vec<&dyn ToSql> = params.as_slice().iter().map(|x| x.to_sql()).collect::<Vec<_>>();
    //     let p_slien = param_vec.as_slice();
    //     let mut binding = stmt_res.unwrap();
    //     let t_iter_res = binding.query_map(p_slien, f);
    //     if t_iter_res.is_err(){
    //         return Err(CustomError::DatabaseError(t_iter_res.err().unwrap().to_string()))
    //     }
    //     let mut vec = Vec::new();
    //     for t in t_iter_res.unwrap() {
    //         vec.push(t.unwrap());
    //     }
    //     Ok(vec)
    // }
    //
    // pub(crate) fn query_one<F,T,K,Q>(sql:&str, params: &Vec<ParamValue>, f:F, k:K ) -> Result<Option<Q>,CustomError> where T:Serialize, F: FnMut(&Row<'_>) -> rusqlite::Result<T>, K: Fn(&mut Vec<T>) -> Result<Option<Q>,CustomError> {
    //     let vec_res = Self::query_vec(sql,params,f);
    //     if vec_res.is_err(){
    //         return Err(vec_res.err().unwrap());
    //     }
    //     let mut vec = vec_res.unwrap();
    //     if vec.is_empty(){
    //         return Ok(None);
    //     }
    //     k(&mut vec)
    // }
    //
    // pub(crate) fn exec(sql:&str, params: &Vec<ParamValue>) -> Result<(),CustomError> {
    //     let conn_res = Self::get_db_manager().pool.get();
    //     if conn_res.is_err(){
    //         return Err(BusinessError("Get connection error".to_string()))
    //     }
    //     let conn = conn_res.unwrap();
    //     let param_vec:Vec<&dyn ToSql> = params.as_slice().iter().map(|x| x.to_sql()).collect::<Vec<_>>();
    //     let p_slien = param_vec.as_slice();
    //     let res = conn.execute(sql,p_slien);
    //     if res.is_err(){
    //         return Err(BusinessError("Get connection error".to_string()))
    //     }
    //     Ok(())
    // }
// }

// insert,