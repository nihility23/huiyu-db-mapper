use crate::base::entity::{ColumnType, Entity};
use crate::base::page::Page;
use crate::base::param::ParamValue;
use chrono::Local;
use crate::query::query::{QueryItem, QueryItemGroup};
use crate::query::query::QueryItemNode::{Item, ItemGroup};
use crate::query::query_wrapper::QueryWrapper;

pub trait WhereSqlGenerator{
    fn eq(&self, column:&str)->String{
        format!("{} = ?", column)
    }
    fn ne(&self, column:&str)->String{
        format!("{} != ?", column)
    }
    fn lt(&self, column:&str)->String{
        format!("{} < ?", column)
    }
    fn le(&self, column:&str)->String{
        format!("{} <= ?", column)
    }
    fn gt(&self, column:&str)->String{
        format!("{} > ?", column)
    }
    fn ge(&self, column:&str)->String{
        format!("{} >= ?", column)
    }
    fn between(&self, column:&str)->String{
        format!("{} between ? and ?", column)
    }
    fn like(&self, column:&str)->String{
        format!("{} like ?", column)
    }
    fn not_like(&self, column:&str)->String{
        format!("{} != ?", column)
    }
    fn is_null(&self, column:&str)->String{
        format!("{} is null", column)
    }
    fn is_not_null(&self, column:&str)->String{
        format!("{} is not null", column)
    }
    fn in_vec(&self, column:&str, vec_size: usize)->String{
        format!("{} in ({})", column,vec!["?";vec_size].join(","))
    }
    fn in_sql(&self, column:&str, sql: &str)->String{
        format!("{} in ({})", column,sql)
    }
    fn not_in_vec(&self, column:&str, vec_size: usize)->String{
        format!("{} not in ({})", column,vec!["?";vec_size].join(","))
    }
    fn not_in_sql(&self, column:&str, sql: &str)->String{
        format!("{} not in ({})", column,sql)
    }

    fn apply_sql(&self, sql: &str, vec_size: usize)->String{
        sql.to_string()
    }
    
    fn exist(&self, sql:&str)->String{
        format!("exists ({})", sql)
    }
    
    fn not_exist(&self, sql:&str)->String{
        format!("not exists ({})", sql)
    }
}

pub trait PageSqlGenerator{
    fn gen_page_query_sql(&self, query_sql:&str, current_page: u64, page_size: u64)->(String,u64,u64);
    fn gen_page_total_sql(&self, query_sql:&str)->String{
        format!("select count(*) from ({}) t", query_sql)
    }
}

pub trait BaseSqlGenerator{
    fn gen_insert_one_sql<E>(&self, e: &E) ->(String, Vec<ParamValue>) where E:Entity{
        let mut params = Vec::new();
        let mut column_names = Vec::new();
        for column_info in E::get_column_infos(){
            let value = e.get_value_by_column_name(column_info.column_name);
            if value.is_not_null(){
                column_names.push(column_info.column_name);
                params.push(value);
            }else if column_info.column_type==ColumnType::DateTime && (column_info.fill_on_insert || column_info.fill_on_update){
                    column_names.push(column_info.column_name);
                    params.push(ParamValue::DateTime(Local::now()));

            }
        }
        (format!("insert into {}({}) values ({})", E::table_name(),column_names.join(","),vec!["?";column_names.len()].join(",")),params)
    }

    fn gen_insert_batch_sql<E>(&self, e_vec: &Vec<E>)->(String,Vec<ParamValue>) where E:Entity{
        let mut params = Vec::new();
        for e in e_vec{
            for column_info in E::get_column_infos(){
                let value = e.get_value_by_column_name(column_info.column_name);
                if value.is_not_null(){
                    params.push(value);
                }else if column_info.column_type==ColumnType::DateTime && (column_info.fill_on_insert || column_info.fill_on_update){
                    params.push(ParamValue::DateTime(Local::now()));

                }
            }
        }
        (format!("insert into {}({}) values {}", E::table_name(),E::column_names().join(","),vec![format!("({})", vec!["?";E::column_names().len()].join(",")).as_str();e_vec.len()].join(" ")),params)
    }

    fn gen_select_by_key_sql<E>(&self,k : E::K) ->(String,ParamValue) where E:Entity{
        (format!("select * from {} where {} = ?", E::table_name(),E::key_name()),k.into())
    }

    fn gen_select_by_keys_sql<E>(&self,ks : Vec<E::K>) ->(String,Vec<ParamValue>) where E:Entity{
        (format!("select * from {} where {} = ?", E::table_name(),E::key_name()),ks.into_iter().map(|k| k.into()).collect::<Vec<ParamValue>>())
    }

    fn gen_delete_by_key_sql<E>(&self,k : &E::K) ->(String,ParamValue) where E:Entity{
        (format!("delete from {} where {} = ?", E::table_name(),E::key_name()),k.clone().into())
    }

    fn gen_delete_by_keys_sql<E>(&self,ks : &Vec<E::K>) ->(String,Vec<ParamValue>) where E:Entity{
        (format!("delete from {} where {} in ({})", E::table_name(),E::key_name(),vec!["?";ks.len()].join(",")),ks.into_iter().map(|k| k.clone().into()).collect::<Vec<ParamValue>>())
    }

    fn gen_update_by_key_sql<E>(&self,e: &E, is_update_null: bool) ->(String,Vec<ParamValue>) where E:Entity{
        let mut params = Vec::new();
        let mut update_sql_parts = Vec::new();
        for column_info in E::get_column_infos(){
            if column_info.is_primary_key {
                continue;
            }
            let value = e.get_value_by_column_name(column_info.column_name);
            if is_update_null || value.is_not_null(){
                update_sql_parts.push(format!("{} = ?", column_info.column_name));
                params.push(value);
            }else if column_info.fill_on_update && column_info.column_type==ColumnType::DateTime{
                update_sql_parts.push(format!("{} = ?", column_info.column_name));
                params.push(ParamValue::DateTime(Local::now()));
            }
        }
        params.push(e.key().into());
        (format!("update {} set {} where {} = ?", E::table_name(),update_sql_parts.join(","),E::key_name()),params)
    }

    fn gen_insert_and_get_id_sql<E>(&self, e: &E) ->(String,Vec<ParamValue>) where E:Entity;
}

pub trait QueryWrapperSqlGenerator : BaseSqlGenerator + PageSqlGenerator + WhereSqlGenerator {
    fn gen_update_sql<E>(&self,e: &E,query_wrapper: &QueryWrapper<E>, is_update_null: bool) ->(String,Vec<ParamValue>) where E:Entity{
        let mut params = Vec::new();
        let mut update_sql_parts = Vec::new();
        for column_info in E::get_column_infos(){
            let value = e.get_value_by_column_name(column_info.column_name);
            if is_update_null || value.is_not_null(){
                update_sql_parts.push(format!("{} = ?", column_info.column_name));
                params.push(value);
            }else if column_info.fill_on_update && column_info.column_type==ColumnType::DateTime{
                update_sql_parts.push(format!("{} = ?", column_info.column_name));
                params.push(ParamValue::DateTime(Local::now()));
            }
        }
        let (where_sql,param_types) = self.gen_where_sql(query_wrapper).unwrap();
        params.extend(param_types);
        (format!("update {} set {} where {}", E::table_name(),update_sql_parts.join(","),where_sql),params)
    }

    fn gen_delete_sql<E>(&self,query_wrapper: &QueryWrapper<E>) ->(String,Vec<ParamValue>) where E:Entity{
        let (where_sql,params) = self.gen_where_sql(query_wrapper).unwrap();
        (format!("delete from {} where {}", E::table_name(),where_sql),params)
    }
    fn gen_query_column_sql<E>(&self, query_wrapper: &QueryWrapper<E>)-> String where E:Entity {
        if query_wrapper.query.select_include_columns.len() > 0{
            return query_wrapper.query.select_include_columns.join(",");
        }
        let mut column_names: Vec<_> = E::column_names();
        if query_wrapper.query.select_exclude_columns.len() > 0 {
            column_names.retain(|c|!query_wrapper.query.select_exclude_columns.contains(c));
        }
        column_names.join(",")
    }
    fn gen_where_sql<E>(&self, query_wrapper: &QueryWrapper<E>)-> Option<(String,Vec<ParamValue>)> where E:Entity {
        if query_wrapper.query.is_empty(){
            return None
        }
        let (where_sql,param_type_vec) = self.query_group_to_sql::<>(&query_wrapper.query.query_group);
        Some((where_sql,param_type_vec))
    }

    fn gen_query_sql<E>(&self, query_wrapper: &QueryWrapper<E>)->(String,Vec<ParamValue>) where E:Entity  {
        let mut params = Vec::new();
        let query_column_sql = self.gen_query_column_sql(query_wrapper);
        let where_sql_opt = self.gen_where_sql(query_wrapper);
        let mut sql;
        if where_sql_opt.is_some(){
            let (where_sql,param_types) = where_sql_opt.unwrap();
            sql = format!("select {} from {} where {}",query_column_sql,E::table_name(),where_sql);
            params.extend(param_types);
        }else {
            sql = format!("select {} from {}", query_column_sql,E::table_name());
        }
        if !query_wrapper.query.group_by_columns.is_empty(){
            sql.push_str(format!(" group by {}",query_wrapper.query.group_by_columns.iter().map(|c|c.to_string()).collect::<Vec<_>>().join(",")).as_str());
        }
        if !query_wrapper.query.order_by_types.is_empty(){
            let mut order_sql = Vec::new();
            for (c,order_by) in query_wrapper.query.order_by_types.iter(){
                order_sql.push(format!("{} {}",c,match order_by { true => "ASC", false => "DESC" }));
            }
            sql.push_str(format!(" order by {}",order_sql.join(",")).as_str());
        }
        if query_wrapper.query.limit_size.is_some(){
            let p1;
            let p2;
            (sql, p1, p2) = self.gen_page_query_sql(&sql,1u64,query_wrapper.query.limit_size.unwrap());
            params.push(ParamValue::U64(p1));
            params.push(ParamValue::U64(p2));
        }
        (sql, params)
    }
    fn gen_page_sql<E>(&self, page: &Page,query_wrapper: &QueryWrapper<E>)-> (String,String,Vec<ParamValue>) where E:Entity,  {
        let (mut sql,mut params) = self.gen_query_sql(query_wrapper);
        let p1;
        let p2;
        let total_page_sql = self.gen_page_total_sql(&sql.as_str());
        (sql, p1, p2) = self.gen_page_query_sql(&sql.as_str(),page.current_page,page.page_size);
        params.push(ParamValue::U64(p1));
        params.push(ParamValue::U64(p2));
        (sql,total_page_sql,params)
    }

    fn query_group_to_sql(&self, query_item_group: &QueryItemGroup)-> (String,Vec<ParamValue>){
        let mut query_value_vec = Vec::new();
        let mut where_sql_vec = Vec::new();
        for query_item_node  in &query_item_group.query_item_nodes{
            match query_item_node  {
                Item(query_item) => {
                    match query_item {
                        QueryItem::Eq(column,param_value)=>{
                            where_sql_vec.push(self.eq(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Ne(column,param_value)=>{
                            where_sql_vec.push(self.ne(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Lt(column,param_value)=>{
                            where_sql_vec.push(self.lt(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Le(column,param_value)=>{
                            where_sql_vec.push(self.le(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Gt(column,param_value)=>{
                            where_sql_vec.push(self.gt(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Ge(column,param_value)=>{
                            where_sql_vec.push(self.ge(column));
                            query_value_vec.push(param_value.clone());
                        }
                        QueryItem::Between(column,param_value_start,param_value_end)=>{
                            where_sql_vec.push(self.between(column));
                            query_value_vec.push(param_value_start.clone());
                            query_value_vec.push(param_value_end.clone());
                        }
                        QueryItem::Like(column,param_value)=>{
                            where_sql_vec.push(self.like(column));
                            query_value_vec.push(ParamValue::String(format!("%{}%",param_value.to_string())));
                        }
                        QueryItem::LikeLeft(column,param_value)=>{
                            where_sql_vec.push(self.like(column));
                            query_value_vec.push(ParamValue::String(format!("%{}",param_value.to_string())));
                        }
                        QueryItem::LikeRight(column,param_value)=>{
                            where_sql_vec.push(self.like(column));
                            query_value_vec.push(ParamValue::String(format!("{}%",param_value.to_string())));
                        }
                        QueryItem::NotLike(column,param_value)=>{
                            where_sql_vec.push(self.not_like(column));
                            query_value_vec.push(ParamValue::String(format!("%{}%",param_value.to_string())));
                        }
                        QueryItem::NotLikeLeft(column,param_value)=>{
                            where_sql_vec.push(self.not_like(column));
                            query_value_vec.push(ParamValue::String(format!("%{}",param_value.to_string())));
                        }
                        QueryItem::NotLikeRight(column,param_value)=>{
                            where_sql_vec.push(self.not_like(column));
                            query_value_vec.push(ParamValue::String(format!("{}%",param_value.to_string())));
                        }
                        QueryItem::IsNotNull(column)=>{
                            where_sql_vec.push(self.is_not_null(column));
                        }
                        QueryItem::IsNull(column)=>{
                            where_sql_vec.push(self.is_null(column));
                        }
                        QueryItem::In(column,param_values)=>{
                            where_sql_vec.push(self.in_vec(column,param_values.len()));
                            query_value_vec.extend(param_values.to_vec());
                        }
                        QueryItem::NotIn(column,param_values)=>{
                            where_sql_vec.push(self.not_in_vec(column,param_values.len()));
                            query_value_vec.extend(param_values.to_vec());
                        }
                        QueryItem::InSql(column,sql)=>{
                            where_sql_vec.push(self.in_sql(column,sql));
                        }
                        QueryItem::NotInSql(column,sql)=>{
                            where_sql_vec.push(self.not_in_sql(column,sql));
                        }
                        QueryItem::ApplySql(sql,param_values)=>{
                            where_sql_vec.push(self.apply_sql(sql,param_values.len()));
                                query_value_vec.extend(param_values.to_vec());
                        }
                        QueryItem::ExistsSql(sql,param_values)=>{
                            where_sql_vec.push(self.exist(sql));
                            query_value_vec.extend(param_values.to_vec());
                        }
                        QueryItem::NotExistsSql(sql,param_values)=>{
                            where_sql_vec.push(self.not_exist(sql));
                            query_value_vec.extend(param_values.to_vec());
                        }
                    }
                }
                ItemGroup(query_item_group) => {
                    let (sub_sql,sub_param_vec) = self.query_group_to_sql(query_item_group);
                    where_sql_vec.push(format!("({})",sub_sql));
                    query_value_vec.extend(sub_param_vec);
                }
            }
        }
        (where_sql_vec.join(format!(" {} ",query_item_group.relation_type.to_sql()).as_str()),query_value_vec)
    }

}

