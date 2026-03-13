// use huiyu_db_mapper_core::base::db_type::DbType;
// use huiyu_db_mapper_core::sql::executor::Executor;
// use huiyu_db_mapper_sqlite::sqlite::sqlite_executor::SQLITE_SQL_EXECUTOR;
// use crate::query::db_type_wrapper::DbTypeWrapper;
//
// impl DbTypeWrapper{
//     pub async fn transactional<T>(db_type: &DbType)->T {
//         match db_type {
//             #[cfg(feature = "mysql")]
//             DbType::Mysql => {
//
//             },
//             #[cfg(feature = "postgres")]
//             DbType::Postgres => {
//
//             },
//             #[cfg(feature = "sqlite")]
//             DbType::Sqlite => {
//
//             },
//             #[cfg(feature = "postgres")]
//             DbType::Postgres => {
//
//             },
//
//             _=>{}
//         }
//         Ok(())
//     }
// }