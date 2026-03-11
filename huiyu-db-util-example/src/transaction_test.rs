
/***
    transaction!(
        userMapper.insert();
        appMapper.insert();
        studentMapper.update();
    )

    展开后大致
    
    // 提前导入所有依赖
        use tokio::task;
        use crate::sql::executor::Executor;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use rustlog::{info};

        info!("Executing sql [{}] params[{:?}]", $sql, $params);

        // 创建闭包执行数据库操作
        let db_operation = move || -> Result<_, DatabaseError> {
            match $db_type {
                DbType::Mysql => {
                    // // mysql
                    use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
                    use r2d2_mysql::MySqlConnectionManager;
                    use r2d2_mysql::mysql::TxOpts;
                     // 获取连接管理器
                     let manager = DbManager::get_instance()
                         .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;
                    
                     // 获取连接
                     let mut conn: PooledConnection<MySqlConnectionManager> = manager.get_conn()
                         .map_err(|e| DatabaseError::CommonError(e.to_string()))?;
                    
                     // 开始事务
                     let tx = conn.start_transaction(TxOpts::default())
                         .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;
                    let tx:&r2d2_mysql::mysql::Transaction = $tx as &r2d2_mysql::mysql::Transaction;
                    // 执行查询 - 根据是否有类型参数选择调用方式
                    // let res = MysqlSqlExecutor::get_sql_executor()
                    //     .$f $(::<$type_args>)? (tx, $sql, $params)
                    //     .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    // res
                    move ||{
                       userMapper.insert(tx,...)?;
                        appMapper.insert(tx,...)?;
                        studentMapper.update(tx,...)?;
                        
                        如果执行报错，就回滚，根据result
                    }

                },
                DbType::Sqlite => {
                    use crate::db::sqlite::sqlite_executor::SqliteSqlExecutor;
                    use r2d2_sqlite::SqliteConnectionManager;
                    // 获取连接管理器
                    let manager = DbManager::get_instance()
                        .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;
                    
                    // 获取连接
                    let mut conn: PooledConnection<SqliteConnectionManager> = manager.get_conn()
                        .map_err(|e| DatabaseError::CommonError(e.to_string()))?;
                    
                    // 开始事务
                    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;
                    let tx:&rusqlite::Transaction = $tx as &rusqlite::Transaction;
                    // 执行查询 - 根据是否有类型参数选择调用方式
                    // let res = SqliteSqlExecutor::get_sql_executor()
                    //     .$f $(::<$type_args>)? (tx, $sql, $params)
                    //     .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    
                    userMapper.insert(tx,...);
                    appMapper.insert(tx,...);
                    studentMapper.update(tx,...);
                    
                    res
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }
        };

        // 在阻塞线程中执行并处理结果
        match task::spawn_blocking(db_operation).await {
            Ok(query_result) => query_result,
            Err(join_error) => Err(DatabaseError::CommonError(format!("Task execution failed: {}", join_error))),
        }
    }};


 */