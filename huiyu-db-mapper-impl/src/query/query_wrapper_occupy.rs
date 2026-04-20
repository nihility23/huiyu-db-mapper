use huiyu_db_mapper_core::base::entity::{ColumnInfo, Entity};
use huiyu_db_mapper_core::base::mapping::Mapping;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;

pub struct EntityOccupy;

impl Entity for EntityOccupy {
    type K = String;

    fn is_case_sensitive() -> bool {
        false
    }

    fn key(&self) -> Self::K {
        String::from("key")
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn key_info() -> Option<ColumnInfo> {
        None
    }

    fn table_name() -> &'static str {
        "occupy"
    }

    fn get_column_infos() -> Vec<ColumnInfo> {
        Vec::new()
    }
}

impl Mapping for EntityOccupy {
    fn column_names() -> Vec<&'static str> {
        Vec::new()
    }

    fn field_names() -> Vec<&'static str> {
        Vec::new()
    }

    fn new() -> Self {
        EntityOccupy {}
    }

    fn get_value_by_field_name(&self, _field_name: &str) -> ParamValue {
        ParamValue::Null

    }

    fn get_value_by_column_name(&self, _column_name: &str) -> ParamValue {
        ParamValue::Null

    }

    fn set_value_by_field_name(&mut self, _field_name: &str, _value: ParamValue) {

    }

    fn set_value_by_column_name(&mut self, _column_name: &str, _value: ParamValue) {

    }
}

pub type OccupyQueryWrapper<'a> = QueryWrapper<'a,EntityOccupy>; 

