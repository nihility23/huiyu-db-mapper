use huiyu_db_util::huiyu_db_macros::datasource;
use crate::entities::UserEntity;
use crate::mappers::UserMapper;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test(){
    common::init_dbs();

    for i in 0..10{
        tokio::spawn(async {
            let users = query_sqlite_users().await;
            println!("users: {:?}", users)
        });
    }

}

#[datasource("sqlite")]
async fn query_sqlite_users()->Vec<UserEntity>{
    let start = std::time::Instant::now();
    let query_wrapper = QueryWrapper::<UserEntity>::new().eq("id", "1".into());
    let users = UserMapper::select(&query_wrapper).await;
    let end = start.elapsed();
    println!("query_sqlite_users cost: {:?}", end);
    println!("query_sqlite_users result: {:?}", users);
    users.unwrap()
}