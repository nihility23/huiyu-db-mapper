use huiyu_db_mapper_core::base::entity::{ColumnInfo, Entity};
use huiyu_db_mapper_core::base::mapping::Mapping;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;

pub struct EntityOccupy;

impl Entity for EntityOccupy {
    type K = String;

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

    fn get_value_by_field_name(&self, field_name: &str) -> ParamValue {
        ParamValue::Null

    }

    fn get_value_by_column_name(&self, column_name: &str) -> ParamValue {
        ParamValue::Null

    }

    fn set_value_by_field_name(&mut self, field_name: &str, value: ParamValue) {

    }

    fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue) {

    }
}

pub type OccupyQueryMapper<'a> = QueryWrapper<'a,EntityOccupy>;

