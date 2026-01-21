use chrono::{DateTime, Local};
use crate::base::param::ParamValue;

pub trait Entity : Send + Sync + 'static {
    type K: Into<ParamValue>;

    fn key(&self) -> Self::K;

    fn key_name() -> &'static str;

    fn column_names() -> Vec<&'static str>;

    fn field_names() -> Vec<&'static str>;

    fn table_name() -> &'static str;

    fn new()-> Self;

    fn get_value_by_field_name(&self,field_name: &str)->ParamValue;

    fn get_value_by_column_name(&self,column_name: &str)->ParamValue;

    fn set_value_by_field_name(&mut self,field_name: &str, value : ParamValue);

    fn set_value_by_column_name(&mut self,column_name: &str, value : ParamValue);

    fn get_column_infos()->Vec<ColumnInfo>;
    
}



pub struct ColumnInfo{

    pub field_name: &'static str,

    pub column_name: &'static str,

    pub column_type: ColumnType,
    // 当更新时候放入最新值，只有时间类型生效
    pub fill_on_update: bool,
    // 当插入时候放入最新值，只有时间类型生效
    pub fill_on_insert: bool,

}

impl ColumnInfo{
    pub fn new(field_name: &'static str, column_name: &'static str, column_type: ColumnType, fill_on_update: bool, fill_on_insert: bool) -> Self {
        Self{field_name, column_name,column_type, fill_on_update, fill_on_insert}
    }
}

#[derive(Debug,Eq,PartialEq)]
pub enum ColumnType{
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    Bool,
    String,
    DateTime,
}

pub trait TypeHandler{
    type K: Into<ParamValue>;
}