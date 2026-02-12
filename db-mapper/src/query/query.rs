use crate::base::param::ParamValue;
use crate::query::query::QueryRelation::{And, Or};
use std::fmt::Debug;

// where
#[derive(Debug)]
pub enum QueryItem<'a> {
    Eq(&'a str, ParamValue),
    Ne(&'a str, ParamValue),
    Lt(&'a str, ParamValue),
    Le(&'a str, ParamValue),
    Gt(&'a str, ParamValue),
    Ge(&'a str, ParamValue),
    Between(&'a str, ParamValue, ParamValue),
    Like(&'a str, ParamValue),
    LikeLeft(&'a str, ParamValue),
    LikeRight(&'a str, ParamValue),
    NotLike(&'a str, ParamValue),
    NotLikeLeft(&'a str, ParamValue),
    NotLikeRight(&'a str, ParamValue),
    In(&'a str, Vec<ParamValue>),
    InSql(&'a str, String),
    NotIn(&'a str, Vec<ParamValue>),
    NotInSql(&'a str, String),
    IsNotNull(&'a str),
    IsNull(&'a str),
    ApplySql(&'a str, Vec<ParamValue>),
}

// select 列
pub enum SelectType<'a> {
    Select(Vec<&'a str>),
    SelectExcludes(Vec<&'a str>),
    Limit(usize),
    GroupBy(Vec<&'a str>),
}

// order
pub enum OrderByType<'a> {
    OrderByDesc(&'a str),
    OrderByAsc(&'a str),
    OrderBy(&'a str, bool),
}

// 查询关系
#[derive(Debug)]
pub(crate) enum QueryRelation {
    And,
    Or,
}

impl QueryRelation {
    pub(crate) fn to_sql(&self) -> &'static str {
        match self {
            And => " AND ",
            Or => " OR ",
        }
    }
}

pub(crate) struct QueryItemGroup<'a> {
    pub(crate) query_item_nodes: Vec<QueryItemNode<'a>>,
    pub(crate) relation_type: QueryRelation,
}

pub(crate) enum QueryItemNode<'a> {
    Item(QueryItem<'a>),
    ItemGroup(QueryItemGroup<'a>),
}

impl QueryItemGroup<'_> {
    pub(crate) fn and() -> Self {
        Self {
            query_item_nodes: Vec::new(),
            relation_type: And,
        }
    }

    pub(crate) fn or() -> Self {
        Self {
            query_item_nodes: Vec::new(),
            relation_type: Or,
        }
    }

    pub(crate) fn new() -> Self {
        Self::and()
    }

    pub(crate) fn clear(&mut self) {
        self.query_item_nodes.clear();
        self.relation_type = And
    }
}

pub(crate) struct Query<'a> {
    pub(crate) query_group: QueryItemGroup<'a>,
    pub(crate) select_include_columns: Vec<&'a str>,
    pub(crate) select_exclude_columns: Vec<&'a str>,
    pub(crate) order_by_types: Vec<(&'a str, bool)>,
    pub(crate) group_by_columns: Vec<&'a str>,
    pub(crate) limit_size: Option<u64>,
}

impl Query<'_> {
    pub(crate) fn is_empty(&self) -> bool {
        self.query_group.query_item_nodes.is_empty()
    }
    pub(crate) fn new() -> Self {
        Self {
            query_group: QueryItemGroup::new(),
            select_include_columns: Vec::new(),
            select_exclude_columns: Vec::new(),
            order_by_types: Vec::new(),
            group_by_columns: Vec::new(),
            limit_size: None,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.query_group.clear();
        self.select_include_columns.clear();
        self.select_exclude_columns.clear();
        self.order_by_types.clear();
        self.group_by_columns.clear();
        self.limit_size = None;
    }
}

