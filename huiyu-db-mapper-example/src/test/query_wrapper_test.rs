use chrono::Local;
use crate::common::db::init_dbs;
use crate::mapper::mappers::{PermissionMapper, RoleMapper};
use huiyu_db_mapper::huiyu_db_mapper_macros::datasource;
use huiyu_db_mapper::huiyu_db_mapper_impl::query::base_mapper::BaseMapper;
use huiyu_db_mapper::huiyu_db_mapper_impl::query::query_wrapper_occupy::OccupyQueryMapper;
use huiyu_db_mapper::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper::huiyu_db_mapper_core::base::page::Page;

#[tokio::test]
async fn test(){
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    init_dbs();

    let res = queries().await;
    if res.is_err(){
        println!("{}", res.err().unwrap());
    }
}
#[datasource("sqlite")]
async fn queries()->Result<(),DatabaseError>{
    let permission = PermissionMapper::select_by_key(&"perm_001".to_string()).await?;
    println!("{:?}", permission);

    let role = RoleMapper::query_role_dtos("role_001".to_string()).await?;
    println!("{:?}", role);

    let page = RoleMapper::query_role_page(Page::new(1,2), "role".to_string()).await;
    println!("{:?}", page);

    let role = RoleMapper::query_role_first("admin".to_string(),1).await?;
    println!("{:?}", role);

    let role_name = RoleMapper::query_role_name("admin".to_string(), 1).await?;
    println!("{:?}", role_name);

    let query1 = OccupyQueryMapper::new().eq("status",1);
    let res = RoleMapper::query_role_dtos_by_query_wrapper("admin".to_string(),&query1).await?;
    println!("{:?}", res);

    let res = RoleMapper::query_role_name_query_wrapper("管理员".to_string(), &query1).await?;
    println!("{:?}", res);

    let query2 = OccupyQueryMapper::new().le("create_time",Local::now());
    let res = RoleMapper::query_role_by_multiple_wrappers("admin".to_string(), &query1, &query2).await?;
    println!("{:?}", res);
    
    let res = RoleMapper::update_role(Local::now(), &query1, &query2).await?;  
    println!("{:?}", res);
    Ok(())
}