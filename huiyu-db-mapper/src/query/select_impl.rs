#[macro_export]
macro_rules! select_impl {
    // ===== 辅助宏：处理多个 query_wrapper 的 SQL 替换 =====
    (@process_wrappers $sql:expr, [$($query_wrapper:expr),*], $db_type:expr) => {{
        let mut result_sql = $sql.to_string();
        let mut all_params = Vec::new();
        
        $(
            if let Some((where_sql, mut params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
                .gen_where_sql($query_wrapper) {
                result_sql = result_sql.replacen("#{query_wrapper}", &where_sql,1);
                all_params.append(&mut params);
            }
        )*
        
        (result_sql, all_params)
    }};

    // ===== 辅助宏：处理单个 query_wrapper 的 SQL 替换（向后兼容）=====
    (@process_wrapper $sql:expr, $query_wrapper:expr, $db_type:expr) => {{
        select_impl!(@process_wrappers $sql, [$query_wrapper], $db_type)
    }};

    // ===== 带 value 属性 + 多个 query_wrapper 的方法 =====
    (
        #[select($sql:literal)]
        #[value]
        async fn $method_name:ident<'a>($($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<Option<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<Option<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);
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

    // ===== 带 value 属性 + query_wrapper 的方法（向后兼容）=====
    (
        #[select($sql:literal)]
        #[value]
        async fn $method_name:ident<'a>($query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<Option<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrapper $sql, &$query_wrapper, db_type);
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

    // ===== PageRes<T> + 多个 query_wrapper 方法 =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($page_param:ident: Page, $($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($page_param: Page, $($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let (sql, mut params) = select_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);

                    let total_sql = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_total_sql(&sql);

                    let (page_sql, offset, limit) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);

                    params.push(ParamValue::U64(offset));
                    params.push(ParamValue::U64(limit));

                    (page_sql, total_sql, params, $page_param, db_type)
                },
                async |(page_sql, total_sql, mut params, page, db_type)| {
                    let total_params = params[0..params.len()-2].to_vec();
                    let total = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_count(&total_sql, &total_params)
                        .await?;

                    let list = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(&page_sql, &params)
                        .await?;

                    Ok(PageRes::new_from_records(total, page.page_size, list))
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== PageRes<T> + query_wrapper 方法（向后兼容）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($page_param:ident: Page, $query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($page_param: Page, $query_wrapper: &OccupyQueryMapper<'a>) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let (sql, mut params) = select_impl!(@process_wrapper $sql, &$query_wrapper, db_type);

                    let total_sql = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_total_sql(&sql);

                    let (page_sql, offset, limit) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);

                    params.push(ParamValue::U64(offset));
                    params.push(ParamValue::U64(limit));

                    (page_sql, total_sql, params, $page_param, db_type)
                },
                async |(page_sql, total_sql, mut params, page, db_type)| {
                    let total_params = params[0..params.len()-2].to_vec();
                    let total = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_count(&total_sql, &total_params)
                        .await?;

                    let list = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(&page_sql, &params)
                        .await?;

                    Ok(PageRes::new_from_records(total, page.page_size, list))
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Vec<T> + 多个 query_wrapper 方法 =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<Vec<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<Vec<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);
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

    // ===== Vec<T> + query_wrapper 方法（向后兼容）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<Vec<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($query_wrapper: &OccupyQueryMapper<'a>) -> Result<Vec<$inner>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrapper $sql, &$query_wrapper, db_type);
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

    // ===== Option<T> + 多个 query_wrapper 方法 =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<Option<$entity:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<Option<$entity>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);
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

    // ===== Option<T> + query_wrapper 方法（向后兼容）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<Option<$entity:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<$entity>, DatabaseError> {
            Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrapper $sql, &$query_wrapper, db_type);
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

    // ===== T + 多个 query_wrapper 方法 =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<$entity:ty, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<$entity, DatabaseError> {
            let result: Option<$entity> = Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);
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

    // ===== T + query_wrapper 方法（向后兼容）=====
    (
        #[select($sql:literal)]
        async fn $method_name:ident<'a>($query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<$entity:ty, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($query_wrapper: &OccupyQueryMapper<'a>) -> Result<$entity, DatabaseError> {
            let result: Option<$entity> = Self::exec(
                |db_type: DbType| {
                    let (sql, params) = select_impl!(@process_wrapper $sql, &$query_wrapper, db_type);
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

    // ===== 带 value 属性的方法（无 query_wrapper） =====
    (
        #[select($sql:literal)]
        #[value]
        async fn $method_name:ident($($param_name:ident: $param_type:ty),* $(,)?) -> Result<Option<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($param_name: $param_type),*) -> Result<Option<$inner>, DatabaseError> {
            let sql = $sql;
            let param_vec = vec![
                $($param_name.into(),)*
            ];

            Self::exec(
                |db_type: DbType| {
                    (sql, param_vec, db_type)
                },
                async |(sql, param_vec, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one_value(sql, &param_vec)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== PageRes<T> 方法（无 query_wrapper） =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($page_param:ident: Page, $($param_name:ident: $param_type:ty),* $(,)?) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($page_param: Page, $($param_name: $param_type),*) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let sql = $sql;
                    let mut param_vec = vec![
                        $($param_name.into(),)*
                    ];

                    let total_page_sql = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_total_sql(&sql);

                    let (page_sql, offset, limit) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);

                    param_vec.push(ParamValue::U64(offset));
                    param_vec.push(ParamValue::U64(limit));

                    (page_sql, total_page_sql, param_vec, $page_param, db_type)
                },
                async |(page_sql, total_page_sql, param_vec, page, db_type)| {
                    let total_param_vec = param_vec[0..param_vec.len()-2].to_vec();
                    let total = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_count(total_page_sql.as_str(), &total_param_vec)
                        .await?;

                    let list = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(page_sql.as_str(), &param_vec)
                        .await?;

                    Ok(PageRes::new_from_records(total, page.page_size, list))
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Vec<T> 方法（无 query_wrapper） =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($param_name:ident: $param_type:ty),* $(,)?) -> Result<Vec<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($param_name: $param_type),*) -> Result<Vec<$inner>, DatabaseError> {
            let sql = $sql;
            let param_vec = vec![
                $($param_name.into(),)*
            ];

            Self::exec(
                |db_type: DbType| {
                    (sql, param_vec, db_type)
                },
                async |(sql, param_vec, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_some(sql, &param_vec)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== Option<T> 方法（无 query_wrapper） =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($param_name:ident: $param_type:ty),* $(,)?) -> Result<Option<$entity:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($param_name: $param_type),*) -> Result<Option<$entity>, DatabaseError> {
            let sql = $sql;
            let param_vec = vec![
                $($param_name.into(),)*
            ];

            Self::exec(
                |db_type: DbType| {
                    (sql, param_vec, db_type)
                },
                async |(sql, param_vec, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql, &param_vec)
                        .await
                }
            ).await
        }
        $crate::select_impl! { $($rest)* }
    };

    // ===== 单个 T 方法（无 query_wrapper） =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($($param_name:ident: $param_type:ty),* $(,)?) -> Result<$entity:ty, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($param_name: $param_type),*) -> Result<$entity, DatabaseError> {
            let sql = $sql;
            let param_vec = vec![
                $($param_name.into(),)*
            ];

            let result: Option<$entity> = Self::exec(
                |db_type: DbType| {
                    (sql, param_vec, db_type)
                },
                async |(sql, param_vec, db_type)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .query_one(sql, &param_vec)
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