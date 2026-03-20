use crate::base::param::ParamValue;

pub trait Mapping: Send + Sync + 'static {

    fn column_names() -> Vec<&'static str>;

    fn field_names() -> Vec<&'static str>;

    fn new() -> Self;

    fn get_value_by_field_name(&self, field_name: &str) -> ParamValue;

    fn get_value_by_column_name(&self, column_name: &str) -> ParamValue;

    fn set_value_by_field_name(&mut self, field_name: &str, value: ParamValue);

    fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue);

}