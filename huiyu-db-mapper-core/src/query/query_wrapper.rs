use crate::base::entity::Entity;
use crate::base::param::{IntoParamValue, ParamValue};
use std::marker::PhantomData;
use crate::query::query::{Query, QueryItem, QueryItemNode};
use crate::query::query::QueryRelation::{And, Or};

pub struct QueryWrapper<'a, E> where E: Entity{
    pub query: Query<'a>,
    e:PhantomData<E>
}


impl <'a,E>QueryWrapper<'a, E>where E: Entity{
    pub fn new()->QueryWrapper<'a, E>{
        Self{
            query: Query::new(),
            e:PhantomData,
        }
    }

    pub fn or_wrapper_when<F>(self, condition: bool, f: F)->Self where Self: Sized, F: Fn(&mut QueryWrapper<'a,E>)->QueryWrapper<'a,E>{
        if !condition {return self;}
        self.or_wrapper(f)
    }
    pub fn and_wrapper_when<F>(self, condition: bool, f: F)->Self where Self: Sized,F: Fn(&mut QueryWrapper<'a,E>)->QueryWrapper<'a,E>{
        if !condition {return self;}
        self.and_wrapper(f)
    }

    pub fn eq_when(self, condition: bool,column: &'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.eq(column,value)
    }

    pub fn lt_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.lt(column,value)
    }

    pub fn le_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.le(column,value)
    }

    pub fn gt_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.gt(column,value)
    }

    pub fn ge_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.ge(column,value)
    }

    pub fn ne_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.ne(column,value)
    }

    pub fn between_when(self,condition: bool,column:&'a str, start_value: impl IntoParamValue, end_value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.between(column,start_value,end_value)
    }

    pub fn order_by_desc_when(self,condition: bool,column: &'a str)->Self{
        if !condition {return self;}
        self.order_by_desc(column)
    }

    pub fn order_by_asc_when(self,condition: bool,column: &'a str)->Self{
        if !condition {return self;}
        self.order_by_asc(column)
    }

    pub fn order_by_when(self,condition: bool,column: &'a str, is_asc: bool)->Self{
        if !condition {return self;}
        self.order_by(column,is_asc)
    }

    pub fn like_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.like(column,value)
    }

    pub fn like_left_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.like_left(column,value)
    }

    pub fn like_right_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.like_right(column,value)
    }

    pub fn not_like_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.not_like(column,value)
    }

    pub fn not_like_left_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.not_like_left(column,value)
    }

    pub fn not_like_right_when(self,condition: bool,column:&'a str, value: impl IntoParamValue)->Self{
        if !condition {return self;}
        self.not_like_right(column,value)
    }

    pub fn in_values_when(self,condition: bool,column:&'a str, values: Vec<impl IntoParamValue>)->Self{
        if !condition {return self;}
        self.in_values(column,values)
    }

    pub fn in_sql_when(self,condition: bool,column:&'a str, sql:&str)->Self{
        if !condition {return self;}
        self.in_sql(column,sql)
    }

    pub fn not_in_values_when(self,condition: bool,column:&'a str, sql:&str)->Self{
        if !condition {return self;}
        self.not_in_values(column,sql)
    }

    pub fn not_in_sql_when(self,condition: bool,column:&'a str, values: Vec<impl IntoParamValue>)->Self{
        if !condition {return self;}
        self.not_in_sql(column,values)
    }

    pub fn null_when(self,condition: bool,column:&'a str)->Self{
        if !condition {return self;}
        self.null(column)
    }

    pub fn not_null_when(self,condition: bool,column:&'a str)->Self{
        if !condition {return self;}
        self.not_null(column)
    }

    pub fn apply_sql_when(self, condition: bool,sql:&'a str, params: Vec<impl IntoParamValue>)->Self{

        if !condition {return self;}
        self.apply_sql(sql,params)
    }

    pub fn clear_when(mut self, condition: bool)->Self{
        if condition {
            self.query.clear();
        }
        self
    }

    pub fn or_wrapper<F>(mut self, f: F)->Self where Self: Sized, F: Fn(&mut QueryWrapper<'a,E>)->QueryWrapper<'a,E>{
        let mut sub_query_wrapper = QueryWrapper::new();
        f(&mut sub_query_wrapper);
        sub_query_wrapper.query.query_group.relation_type = Or;
        self.query.query_group.query_item_nodes.push(QueryItemNode::ItemGroup(sub_query_wrapper.query.query_group));
        self
    }
    pub fn and_wrapper<F>(mut self, f: F)->Self where Self: Sized,F: Fn(&mut QueryWrapper<'a,E>)->QueryWrapper<'a,E>{
        let mut sub_query_wrapper = QueryWrapper::new();
        f(&mut sub_query_wrapper);
        sub_query_wrapper.query.query_group.relation_type = And;
        self.query.query_group.query_item_nodes.push(QueryItemNode::ItemGroup(sub_query_wrapper.query.query_group));
        self
    }

    pub fn eq(self,column: &'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Eq(column,value.into_param_value()))
    }

    pub fn lt(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Lt(column,value.into_param_value()))
    }

    pub fn le(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Le(column,value.into_param_value()))
    }

    pub fn gt(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Gt(column,value.into_param_value()))
    }

    pub fn ge(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Ge(column,value.into_param_value()))
    }

    pub fn ne(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Ne(column,value.into_param_value()))
    }

    pub fn between(self,column:&'a str, start_value: impl IntoParamValue, end_value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Between(column,start_value.into_param_value(),end_value.into_param_value()))
    }

    pub fn order_by_desc(mut self,column: &'a str)->Self{
        self.query.order_by_types.push((column,false));
        self
    }

    pub fn order_by_asc(mut self,column: &'a str)->Self{
        self.query.order_by_types.push((column,true));
        self
    }

    pub fn order_by(mut self,column: &'a str, is_asc: bool)->Self{
        self.query.order_by_types.push((column,is_asc));
        self
    }

    pub fn like(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::Like(column,value.into_param_value()))
    }

    pub fn like_left(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::LikeLeft(column,value.into_param_value()))
    }

    pub fn like_right(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::LikeRight(column,value.into_param_value()))
    }

    pub fn not_like(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::NotLike(column,value.into_param_value()))
    }

    pub fn not_like_left(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::NotLikeLeft(column,value.into_param_value()))
    }

    pub fn not_like_right(self,column:&'a str, value: impl IntoParamValue)->Self{
        self.add_condition(QueryItem::NotLikeRight(column,value.into_param_value()))
    }

    pub fn in_values(self,column:&'a str, values: Vec<impl IntoParamValue>)->Self{
        self.add_condition(QueryItem::In(column,values.into_iter().map(|value| value.into_param_value()).collect()))
    }

    pub fn in_sql(self,column:&'a str, sql:&str)->Self{
        self.add_condition(QueryItem::InSql(column,sql.to_string()))
    }

    pub fn not_in_values(self,column:&'a str, sql:&str)->Self{
        self.add_condition(QueryItem::NotInSql(column,sql.to_string()))
    }

    pub fn not_in_sql(self,column:&'a str, values: Vec<impl IntoParamValue>)->Self{
        self.add_condition(QueryItem::NotIn(column,values.into_iter().map(|v|v.into_param_value()).collect()))
    }

    pub fn null(self,column:&'a str)->Self{
        self.add_condition(QueryItem::IsNull(column))
    }

    pub fn not_null(self,column:&'a str)->Self{
        self.add_condition(QueryItem::IsNotNull(column))
    }

    pub fn group_by(mut self,columns:Vec<&'a str>)->Self{
        self.query.group_by_columns.extend(columns);
        self
    }

    pub fn limit(mut self,size:u64)->Self{
        self.query.limit_size = Some(size);
        self
    }

    pub fn select(mut self,columns: Vec<&'a str>)->Self{
        self.query.select_include_columns.extend(columns);
        self
    }
    pub fn select_excludes(mut self,columns: Vec<&'a str>)->Self{
        self.query.select_exclude_columns.extend(columns);
        self
    }

    pub fn apply_sql(self, sql:&'a str, params: Vec<impl IntoParamValue>)->Self{
        self.add_condition(QueryItem::ApplySql(sql,params.into_iter().map(|v|v.into_param_value()).collect()))
    }

    pub fn exists(self, sql:&'a str, params: Vec<impl IntoParamValue>)->Self{
        self.add_condition(QueryItem::ExistsSql(sql,params.into_iter().map(|v|v.into_param_value()).collect()))
    }

    pub fn not_exists(self, sql:&'a str, params: Vec<impl IntoParamValue>)->Self{
        self.add_condition(QueryItem::NotExistsSql(sql,params.into_iter().map(|v|v.into_param_value()).collect()))
    }

    pub fn clear(mut self)->Self{
        self.query.clear();
        self
    }

    pub fn add_condition(mut self, query_item: QueryItem<'a>)->Self{
        self.query.query_group.query_item_nodes.push(QueryItemNode::Item(query_item));
        self
    }

}


