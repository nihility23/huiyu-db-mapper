use crate::base::entity::Entity;
use crate::query::query::{Query, QueryItem, QueryItemNode};
use std::marker::PhantomData;
use crate::base::param::ParamValue;
use crate::query::query::QueryRelation::{And, Or};

pub struct QueryWrapper<'a, E> where E: Entity{
    pub(crate) query: Query<'a>,
    e:PhantomData<E>
}


impl <'a,E>QueryWrapper<'a, E>where E: Entity{
    pub fn new()->QueryWrapper<'a, E>{
        Self{
            query: Query::new(),
            e:PhantomData,
        }
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

    pub fn eq(self,column: &'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Eq(column,value))
    }

    pub fn lt(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Lt(column,value))
    }

    pub fn le(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Le(column,value))
    }

    pub fn gt(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Gt(column,value))
    }

    pub fn ge(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Ge(column,value))
    }

    pub fn ne(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Ne(column,value))
    }

    pub fn between(self,column:&'a str, start_value: ParamValue, end_value: ParamValue)->Self{
        self.add_condition(QueryItem::Between(column,start_value,end_value))
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

    pub fn like(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::Like(column,value))
    }

    pub fn like_left(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::LikeLeft(column,value))
    }

    pub fn like_right(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::LikeRight(column,value))
    }

    pub fn not_like(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::NotLike(column,value))
    }

    pub fn not_like_left(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::NotLikeLeft(column,value))
    }

    pub fn not_like_right(self,column:&'a str, value: ParamValue)->Self{
        self.add_condition(QueryItem::NotLikeRight(column,value))
    }

    pub fn in_values(self,column:&'a str, values: Vec<ParamValue>)->Self{
        self.add_condition(QueryItem::In(column,values))
    }

    pub fn in_sql(self,column:&'a str, sql:&str)->Self{
        self.add_condition(QueryItem::InSql(column,sql.to_string()))
    }

    pub fn not_in_values(self,column:&'a str, sql:&str)->Self{
        self.add_condition(QueryItem::NotInSql(column,sql.to_string()))
    }

    pub fn not_in_sql(self,column:&'a str, values: Vec<ParamValue>)->Self{
        self.add_condition(QueryItem::NotIn(column,values))
    }

    pub fn is_null(self,column:&'a str)->Self{
        self.add_condition(QueryItem::IsNull(column))
    }

    pub fn is_not_null(self,column:&'a str)->Self{
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

    pub fn apply_sql(self, sql:&'a str, params: Vec<ParamValue>)->Self{
        self.add_condition(QueryItem::ApplySql(sql,params))
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

