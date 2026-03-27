use huiyu_db_mapper_core::base::param::ParamValue;

#[macro_export]
macro_rules! select_impl {
    // ===== 辅助宏：处理参数 =====
    // 处理空参数列表
    (@process_args (), $sql:expr, $db_type:expr, $params:expr) => {
        // 空参数列表，不需要处理
    };

    // 处理包含 OccupyQueryMapper 的参数（带生命周期）
    (@process_args ($wrapper:ident: &OccupyQueryMapper<$lt:lifetime>, $($rest:tt)*), $sql:expr, $db_type:expr, $params:expr) => {
        if let Some((where_sql, mut wrapper_params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
            .gen_where_sql($wrapper) {
            $sql = $sql.replacen("#{query_wrapper}", &where_sql, 1);
            $params.append(&mut wrapper_params);
        }
        select_impl!(@process_args ($($rest)*), $sql, $db_type, $params);
    };

    // 处理单个包含 OccupyQueryMapper 的参数（带生命周期）
    (@process_args ($wrapper:ident: &OccupyQueryMapper<$lt:lifetime>), $sql:expr, $db_type:expr, $params:expr) => {
        if let Some((where_sql, mut wrapper_params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
            .gen_where_sql($wrapper) {
            $sql = $sql.replacen("#{query_wrapper}", &where_sql, 1);
            $params.append(&mut wrapper_params);
        }
    };

    // 处理普通参数（带逗号）
    (@process_args ($param:ident: $param_type:ty, $($rest:tt)*), $sql:expr, $db_type:expr, $params:expr) => {
        $params.push($param.into());
        select_impl!(@process_args ($($rest)*), $sql, $db_type, $params);
    };

    // 处理单个普通参数
    (@process_args ($param:ident: $param_type:ty), $sql:expr, $db_type:expr, $params:expr) => {
        $params.push($param.into());
    };

    // ===== 带 value 属性的方法（带生命周期）=====
    (
        #[select($sql:literal)]
        #[value]
        async fn $method_name:ident<'a>($($args:tt)*) -> Result<Option<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($args)*) -> Result<Option<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one_value(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== 带 value 属性的方法（无生命周期）=====
    (
        #[select($sql:literal)]
        #[value]
        async fn $method_name:ident($($args:tt)*) -> Result<Option<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($args)*) -> Result<Option<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }

                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one_value(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== PageRes<T> 方法（带生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($page_param:ident: Page, $($args:tt)*) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($page_param: Page, $($args)*) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }

                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    let total_sql = <DbType as Into<DbTypeWrapper>>::into(db_type).gen_page_total_sql(&sql);
                    let (page_sql, offset, limit) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);
                    params.push(ParamValue::U64(offset));
                    params.push(ParamValue::U64(limit));
                    (page_sql, total_sql, params, $page_param, db_type)
                },
                async |(page_sql, total_sql, mut params, page, db_type)| {
                    let total_params = params[0..params.len()-2].to_vec();
                    let total = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_count(&total_sql.as_str(), &total_params).await?;
                    let list = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(&page_sql.as_str(), &params).await?;
                    Ok(PageRes::new_from_records(total, page.page_size, list))
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== PageRes<T> 方法（无生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($page_param:ident: Page, $($args:tt)*) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($page_param: Page, $($args)*) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    let total_sql = <DbType as Into<DbTypeWrapper>>::into(db_type).gen_page_total_sql(&sql);
                    let (page_sql, offset, limit) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);
                    params.push(ParamValue::U64(offset));
                    params.push(ParamValue::U64(limit));
                    (page_sql, total_sql, params, $page_param, db_type)
                },
                async |(page_sql, total_sql, mut params, page, db_type)| {
                    let total_params = params[0..params.len()-2].to_vec();
                    let total = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_count(&total_sql.as_str(), &total_params).await?;
                    let list = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(&page_sql.as_str(), &params).await?;
                    Ok(PageRes::new_from_records(total, page.page_size, list))
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Vec<T> 方法（带生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($args:tt)*) -> Result<Vec<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($args)*) -> Result<Vec<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Vec<T> 方法（无生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($args:tt)*) -> Result<Vec<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($args)*) -> Result<Vec<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Option<T> 方法（带生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($args:tt)*) -> Result<Option<$entity:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($args)*) -> Result<Option<$entity>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Option<T> 方法（无生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($args:tt)*) -> Result<Option<$entity:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($args)*) -> Result<Option<$entity>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql.as_str(), &params)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== T 方法（带生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($args:tt)*) -> Result<$entity:ty, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($args)*) -> Result<$entity, DatabaseError> {
            let result: Option<$entity> = Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql.as_str(), &params)
                        .await
                }
            ).await?;
            result.ok_or(DatabaseError::CommonError("Not found".to_string()))
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== T 方法（无生命周期）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($args:tt)*) -> Result<$entity:ty, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($args)*) -> Result<$entity, DatabaseError> {
            let result: Option<$entity> = Self::exec(
                |db_type: DbType| {
                    let mut sql = $sql.to_string();
                    let mut params: Vec<ParamValue> = vec![];
                    
                    // 处理参数
                    select_impl!(@process_args ($($args)*), sql, db_type, params);
                    
                    // 处理 ?# 占位符
                    let mut param_vec = params.clone();
                    while sql.contains("?#") {
                        let idx = sql.find("?#").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?#", &param_vec[idx].to_string(), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?$") {
                        let idx = sql.find("?$").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?$", &format!("'{}'",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    while sql.contains("?@") {
                        let idx = sql.find("?@").map(|pos| sql[..pos].matches('?').count()).unwrap();
                        sql = sql.replacen("?@", &format!("\"{}\"",&param_vec[idx].to_string()), 1);
                        param_vec.remove(idx);
                    }
                    
                    (sql, params, db_type)
                },
                async |(sql, params, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql.as_str(), &params)
                        .await
                }
            ).await?;
            result.ok_or(DatabaseError::CommonError("Not found".to_string()))
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== 终止递归 =====
    () => { };
}