use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::common::init_dbs;
use crate::entities::UserEntity;
use crate::mappers::UserMapper;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test(){
    init_dbs();

    for i in 0..10{
        tokio::spawn(async {
            let users = query_sqlite_users().await;
            // users
        });
    }

}

// #[datasource("sqlite")]
async fn query_sqlite_users()->Vec<UserEntity>{
    let start = std::time::Instant::now();
    let query_wrapper = QueryWrapper::<UserEntity>::new().eq("id", "1".into());
    let users = UserMapper::select(&query_wrapper).await;
    let end = start.elapsed();
    println!("query_sqlite_users cost: {:?}", end);
    println!("query_sqlite_users result: {:?}", users);
    users.unwrap()
}