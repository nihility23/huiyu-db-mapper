use huiyu_db_mapper_core::base::param::ParamValue;

#[macro_export]
macro_rules! execute_impl {
    // ===== 辅助宏：处理多个 query_wrapper 的 SQL 替换 =====
    (@process_wrappers $sql:expr, [$($query_wrapper:expr),*], $db_type:expr) => {{
        let mut result_sql = $sql.to_string();
        let mut all_params = Vec::new();
        let placeholder = "#{query_wrapper}";
        
        $(
            if let Some((where_sql, mut params)) = <DbType as Into<DbTypeWrapper>>::into($db_type)
                .gen_where_sql($query_wrapper) {
                if let Some(pos) = result_sql.find(placeholder) {
                    let before = &result_sql[..pos];
                    let after = &result_sql[pos + placeholder.len()..];
                    result_sql = format!("{}{}{}", before, where_sql, after);
                    all_params.append(&mut params);
                }
            }
        )*
        
        (result_sql, all_params)
    }};

    // ===== 辅助宏：处理单个 query_wrapper 的 SQL 替换（向后兼容）=====
    (@process_wrapper $sql:expr, $query_wrapper:expr, $db_type:expr) => {{
        execute_impl!(@process_wrappers $sql, [$query_wrapper], $db_type)
    }};

    // ===== 带 query_wrapper 参数的方法（单个）=====
    (
        #[sql($sql:literal)]
        async fn $method_name:ident<'a>($query_wrapper:ident: &OccupyQueryMapper<'a>) -> Result<u64, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($query_wrapper: &OccupyQueryMapper<'a>) -> Result<u64, DatabaseError> {
            Self::exec::<_, _, _, _, u64>(
                |db_type: DbType| {
                    let (sql, params) = execute_impl!(@process_wrapper $sql, &$query_wrapper, db_type);
                    (db_type, sql, params)
                },
                async |(db_type, sql, params)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &params).await
                }
            ).await
        }
        $crate::execute_impl! { $($rest)* }
    };

    // ===== 带多个 query_wrapper 参数的方法 =====
    (
        #[sql($sql:literal)]
        async fn $method_name:ident<'a>($($query_wrapper:ident: &OccupyQueryMapper<'a>),+) -> Result<u64, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name<'a>($($query_wrapper: &OccupyQueryMapper<'a>),+) -> Result<u64, DatabaseError> {
            Self::exec::<_, _, _, _, u64>(
                |db_type: DbType| {
                    let (sql, params) = execute_impl!(@process_wrappers $sql, [$(&$query_wrapper),+], db_type);
                    (db_type, sql, params)
                },
                async |(db_type, sql, params)| {
                    <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &params).await
                }
            ).await
        }
        $crate::execute_impl! { $($rest)* }
    };

    // ===== 普通参数方法（无 query_wrapper）=====
    (
        #[sql($sql:literal)]
        async fn $method_name:ident($($param_name:ident: $param_type:ty),* $(,)?) -> Result<u64, DatabaseError>;
        $($rest:tt)*
    ) => {
        pub async fn $method_name($($param_name: $param_type),*) -> Result<u64, DatabaseError> {
            let mut sql = $sql.to_string();
            let mut param_vec:Vec<ParamValue> = vec![
                $($param_name.into(),)*
            ];
            while sql.contains("?#") {
                sql = sql.replace("?#", &param_vec[0].to_string().as_str());
                param_vec.remove(0);
            }

            Self::exec::<  _,_,_, _, u64>(|db_type: DbType|{
                (db_type,sql,param_vec)
            },async |(db_type,sql,param_vec)|{
                <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(),&param_vec).await
            }).await
        }
        $crate::execute_impl! { $($rest)* }
    };

    // ===== 终止递归 =====
    () => { };
}