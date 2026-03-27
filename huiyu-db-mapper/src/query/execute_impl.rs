use huiyu_db_mapper_core::base::param::ParamValue;

#[macro_export]
macro_rules! execute_impl {
    // ===== 标准 u64 返回值 + 任意参数的方法 =====
    (
        #[sql($sql:literal)]
        async fn $method_name:ident($($args:tt)*) -> Result<u64, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($args)*) -> Result<u64, DatabaseError> {
            Self::exec::<_, _, _, _, u64>(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    execute_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?&") {
                        let idx = sql.find("?&").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?&", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }

                    while sql.contains("?#") { 
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }

                    (db_type, sql, params)
                },
                async |(db_type, sql, params)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &params).await
                }
            ).await
        }
        $crate::execute_impl! { $($rest)* }
    };

    // ===== 辅助宏：处理参数 =====
    // 空参数列表
    (@process_args (), $sql:expr, $db_type:expr, $params:expr) => {
        // 空参数列表，不需要处理
    };

    // query_wrapper + 逗号 + 剩余参数
    (@process_args ($wrapper:ident: &OccupyQueryMapper<'_>, $($rest:tt)*), $sql:expr, $db_type:expr, $params:expr) => {
        if let Some((where_sql, mut wrapper_params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
            .gen_where_sql($wrapper) {
            $sql = $sql.replacen("#{query_wrapper}", &where_sql, 1);
            $params.append(&mut wrapper_params);
        }
        execute_impl!(@process_args ($($rest)*), $sql, $db_type, $params);
    };

    // 普通参数 + 逗号 + 剩余参数
    (@process_args ($param:ident: $param_type:ty, $($rest:tt)*), $sql:expr, $db_type:expr, $params:expr) => {
        $params.push($param.into());
        execute_impl!(@process_args ($($rest)*), $sql, $db_type, $params);
    };

    // 单个 query_wrapper
    (@process_args ($wrapper:ident: &OccupyQueryMapper<'_>), $sql:expr, $db_type:expr, $params:expr) => {
        if let Some((where_sql, mut wrapper_params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
            .gen_where_sql($wrapper) {
            $sql = $sql.replacen("#{query_wrapper}", &where_sql, 1);
            $params.append(&mut wrapper_params);
        }
    };

    // 单个普通参数
    (@process_args ($param:ident: $param_type:ty), $sql:expr, $db_type:expr, $params:expr) => {
        $params.push($param.into());
    };

    // 终止递归
    () => { };
}