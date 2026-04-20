use crate::base::mapping::Mapping;
use crate::base::param::ParamValue;
use tracing::error;

pub trait Entity: Mapping + Send + Sync + 'static {
    type K: Into<ParamValue> + From<ParamValue> + Default + Clone + Send + Sync + 'static;

    fn is_case_sensitive() -> bool;

    fn key(&self) -> Self::K;

    fn key_name() -> &'static str;

    fn key_info() -> Option<ColumnInfo>;

    fn table_name() -> &'static str;

    fn get_column_infos() -> Vec<ColumnInfo>;
}

/**
 * 键生成类型：
    数据库	主键类型	推荐方法	order	关键SQL
    MySQL	自增ID	useGeneratedKeys	-	无需额外SQL
    MySQL	自增ID	<selectKey>	AFTER	LAST_INSERT_ID()
    MySQL	UUID	<selectKey>	BEFORE	UUID()
    Oracle	序列	<selectKey>	BEFORE	序列名.NEXTVAL
    Oracle	UUID	<selectKey>	BEFORE	SYS_GUID()
    PostgreSQL	自增	useGeneratedKeys	-	可能需要keyColumn
    SQL Server	自增	useGeneratedKeys	-	或SCOPE_IDENTITY()
 */

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyGenerateType {
    None,
    // mysql专用
    AutoIncrement,
    // mysql,oracle
    UUID,
    // mysql,oracle,postgresql,sqlserver
    UseGeneratedKeys,
    // oracle,
    Sequence
}

impl From<String> for KeyGenerateType {
    fn from(value: String) -> Self {
        KeyGenerateType::from(value.as_str())
    }

}

impl From<Option<String>> for KeyGenerateType {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => KeyGenerateType::from(value),
            None => KeyGenerateType::None,
        }
    }
}

impl From<&str> for KeyGenerateType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "none" => KeyGenerateType::None,
            "auto_increment" => KeyGenerateType::AutoIncrement,
            "uuid" => KeyGenerateType::UUID,
            "use_generated_keys" => KeyGenerateType::UseGeneratedKeys,
            "sequence" => KeyGenerateType::Sequence,
            _ => {
                error!("Unknown key generate type: {}", value);
                KeyGenerateType::None
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnInfo {
    pub field_name: &'static str,

    pub field_type: FieldType,

    pub column_name: &'static str,

    pub column_type: ColumnType,
    // 当更新时候放入最新值，只有时间类型生效
    pub fill_on_update: bool,
    // 当插入时候放入最新值，只有时间类型生效
    pub fill_on_insert: bool,
    
    pub is_primary_key: bool,

    pub update_null: bool,

    pub is_auto_increment: bool,

    pub key_generate_type: KeyGenerateType,
}

impl ColumnInfo {
    pub fn new(
        field_name: &'static str,
        field_type: FieldType,
        column_name: &'static str,
        column_type: ColumnType,
        fill_on_update: bool,
        fill_on_insert: bool,
        is_primary_key: bool,
        update_null: bool,
        is_auto_increment: bool,
        key_generate_type: KeyGenerateType,
    ) -> Self {
        Self {
            field_name,
            field_type,
            column_name,
            column_type,
            fill_on_update,
            fill_on_insert,
            is_primary_key,
            update_null,
            is_auto_increment,
            key_generate_type,
        }
    }
}

// 数据库支持的类型
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ColumnType {
    TinyInt,
    SmallInt,
    Integer,
    BigInt,
    Float,
    Double,
    Decimal,
    Real,

    Bool,
    DateTime,
    Timestamp,
    TimestampTz,
    Date,
    Time,

    Char,
    NChar,
    Varchar,
    NVarchar,
    Text,
    Blob,
    Clob,
    Numeric,
    Null
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum FieldType{
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    DateTime,
    Decimal,
    Null,
}

pub trait TypeHandler {
    type K: Into<ParamValue>;
}
