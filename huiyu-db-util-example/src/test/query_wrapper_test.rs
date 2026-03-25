use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::common::db::init_dbs;
use crate::entity::entities::RoleEntity;
use crate::mapper::mappers::RoleMapper;

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
    let mut query_wrapper = QueryWrapper::<RoleEntity>::new().eq("id", 1);
    query_wrapper = query_wrapper.like(RoleEntity::ROLE_NAME, "role_");
    query_wrapper = query_wrapper.like_left(RoleEntity::ROLE_NAME, "role_");
    query_wrapper = query_wrapper.like_right(RoleEntity::ROLE_NAME, "role_");

    let result = RoleMapper::select(&query_wrapper).await?;
    println!("{:?}", result);

    query_wrapper = query_wrapper.clear();
    query_wrapper = query_wrapper.eq("id", 1);
    query_wrapper = query_wrapper.ne(RoleEntity::ROLE_NAME, "role_001");
    query_wrapper = query_wrapper.gt(RoleEntity::STATUS, 0);
    query_wrapper = query_wrapper.lt(RoleEntity::STATUS, 1);
    query_wrapper = query_wrapper.order_by(RoleEntity::STATUS, false);

    query_wrapper = query_wrapper.or_wrapper(|mut query_wrapper1| {
        query_wrapper1 = query_wrapper1.eq(RoleEntity::ID, 1);
        query_wrapper1 = query_wrapper1.eq(RoleEntity::ROLE_NAME, 1);
        query_wrapper1
    });

    let result = RoleMapper::select(&query_wrapper).await?;
    println!("{:?}", result);

    Ok(())
}