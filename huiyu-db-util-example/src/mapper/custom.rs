#[macro_export]
macro_rules! select_impl {
    // ===== 主入口：匹配带 value 属性的方法 =====
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

    // ===== 主入口：匹配 PageRes<T> 方法（Page 必须是第一个参数） =====
    (
        #[select($sql:literal)]
        async fn $method_name:ident($page_param:ident: Page, $($param_name:ident: $param_type:ty),* $(,)?) -> Result<PageRes<$inner:ty>, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($page_param: Page, $($param_name: $param_type),*) -> Result<PageRes<$inner>, DatabaseError> {
            Self::exec::<_, _, _, _, PageRes<$inner>>(
                |db_type: DbType| {
                    let sql = $sql;
                    let mut p1: u64 = 0;
                    let mut p2: u64 = 0;
                    let mut page_sql = String::new();

                    let mut param_vec = vec![
                        $($param_name.into(),)*
                    ];

                    let total_page_sql = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_total_sql(&sql);

                    (page_sql, p1, p2) = <DbType as Into<DbTypeWrapper>>::into(db_type)
                        .gen_page_query_sql(&sql, $page_param.current_page, $page_param.page_size);

                    param_vec.push(ParamValue::U64(p1));
                    param_vec.push(ParamValue::U64(p2));

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

    // ===== 主入口：匹配 Vec<T> 方法 =====
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

    // ===== 主入口：匹配 Option<T> 实体类型方法 =====
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

    // ===== 主入口：匹配单个 T（非 Option）方法 =====
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