use crate::common::db::init_dbs;
use crate::mapper::mappers::{PermissionMapper, RoleMapper};
use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper::query::query_wrapper_occupy::OccupyQueryMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_util::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::entity::entities::{PermissionEntity, RoleEntity};

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
    // let permission = PermissionMapper::select_by_key(&"perm_001".to_string()).await?;
    // println!("{:?}", permission);

    let role = RoleMapper::query_role_dtos("role_001".to_string()).await?;
    println!("{:?}", role);
    // let mut query_wrapper = QueryWrapper::<RoleEntity>::new().eq("id", 1);
    // query_wrapper = query_wrapper.like(RoleEntity::ROLE_NAME, "role_");
    // query_wrapper = query_wrapper.like_left(RoleEntity::ROLE_NAME, "role_");
    // query_wrapper = query_wrapper.like_right(RoleEntity::ROLE_NAME, "role_");
    // 
    // let result = RoleMapper::select(&query_wrapper).await?;
    // println!("{:?}", result);
    // 
    // query_wrapper = query_wrapper.clear();
    // query_wrapper = query_wrapper.eq("id", 1);
    // query_wrapper = query_wrapper.ne(RoleEntity::ROLE_NAME, "role_001");
    // query_wrapper = query_wrapper.gt(RoleEntity::STATUS, 0);
    // query_wrapper = query_wrapper.lt(RoleEntity::STATUS, 1);
    // query_wrapper = query_wrapper.order_by(RoleEntity::STATUS, false);
    // 
    // query_wrapper = query_wrapper.or_wrapper(|mut query_wrapper1| {
    //     query_wrapper1 = query_wrapper1.eq(RoleEntity::ID, 1);
    //     query_wrapper1 = query_wrapper1.eq(RoleEntity::ROLE_NAME, 1);
    //     query_wrapper1
    // });
    // 
    // let result = RoleMapper::select(&query_wrapper).await?;
    // println!("{:?}", result);
    // 
    // // 自定义sql,使用通用构造器，不用if标签封装
    // let res = RoleMapper::query_role_name_query_wrapper("".to_string(), &OccupyQueryMapper::new()
    //     .eq("u.username","admin")
    //     .like("r.role_name","管理员")
    // ).await?;
    // println!("{:?}", res);
    // // 测试多个 OccupyQueryMapper 的 SQL 生成
    // let db_type_wrapper = DbTypeWrapper::from(DbType::Sqlite);
    // 
    // let wrapper1 = OccupyQueryMapper::new().eq("status", 1);
    // let wrapper2 = OccupyQueryMapper::new().like("role_name", "test");
    // 
    // let res = RoleMapper::query_role_first_query_wrapper("abc".to_string(), &wrapper1).await?;
    // println!("{:?}",res.unwrap());
    // let res = RoleMapper::update_role_code("a".to_string(),"b".to_string(),&wrapper1,&wrapper2).await?;
    // println!("{:?}", res);
    // 
    // let res = RoleMapper::query_role_by_multiple_wrappers("abc".to_string(), &wrapper1, &wrapper2).await.unwrap();
    // println!("{:?}", res);
    // // 模拟宏中的处理逻辑
    // let result = RoleMapper::query_role_by_multiple_wrappers("".to_string(), &wrapper1, &wrapper2).await;
    // println!("{:?}", result.err());
    // 
    // let res = RoleMapper::create_table_employees(1).await?;
    // println!("{:?}", res);
    
    
    
    Ok(())
}