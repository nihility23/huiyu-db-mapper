use chrono::{DateTime, Local};

#[derive(Clone,Debug)]
pub enum ParamValue{
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    Bool(bool),
    String(String),
    DateTime(DateTime<Local>),
    Blob(Vec<u8>),
    Clob(Vec<u8>),
    Null
}

pub fn get_param_value<T>(value: Option<T>)-> ParamValue where T: Into<ParamValue> {
    if value.is_none() {
        return ParamValue::Null;
    }
    value.unwrap().into()
}

impl From<String> for ParamValue {
    fn from(s: String) -> Self {
        ParamValue::String(s)
    }
}

impl From<u64> for ParamValue {
    fn from(s: u64) -> Self {
        ParamValue::U64(s)
    }
}

impl From<u32> for ParamValue {
    fn from(s: u32) -> Self {
        ParamValue::U32(s)
    }
}

impl From<u16> for ParamValue {
    fn from(s: u16) -> Self {
        ParamValue::U16(s)
    }
}

impl From<u8> for ParamValue {
    fn from(s: u8) -> Self {
        ParamValue::U8(s)
    }
}

impl From<i64> for ParamValue {
    fn from(s: i64) -> Self {
        ParamValue::I64(s)
    }
}

impl From<i32> for ParamValue {
    fn from(s: i32) -> Self {
        ParamValue::I32(s)
    }
}

impl From<i16> for ParamValue {
    fn from(s: i16) -> Self {
        ParamValue::I16(s)
    }
}

impl From<usize> for ParamValue {
    fn from(value: usize) -> Self {
        ParamValue::USize(value)
    }
}

impl From<i8> for ParamValue {
    fn from(value: i8) -> Self {
        ParamValue::I8(value)
    }
}

impl From<bool> for ParamValue {
    fn from(value: bool) -> Self {
        ParamValue::Bool(value)
    }
}

impl From<DateTime<Local>> for ParamValue {
    fn from(value: DateTime<Local>) -> Self {
        ParamValue::DateTime(value)
    }
}

impl Into<Option<i8>> for ParamValue {
    fn into(self) -> Option<i8> {
        match self {
            ParamValue::I8(v) => Some(v),
            _=>None
        }
    }
}
impl Into<Option<i16>> for ParamValue {
    fn into(self) -> Option<i16> {
        match self {
            ParamValue::I16(v) => Some(v),
            _=>None
        }
    }
}
impl Into<Option<i32>> for ParamValue {
    fn into(self) -> Option<i32> {
        match self {
            ParamValue::I32(v) => Some(v),
            _=>None
        }
    }
}
impl Into<Option<i64>> for ParamValue {
    fn into(self) -> Option<i64> {
        match self {
            ParamValue::I64(v) => Some(v),
            _=>None
        }
    }
}


impl Into<Option<usize>> for ParamValue {
    fn into(self) -> Option<usize> {
        match self {
            ParamValue::USize(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<u8>> for ParamValue {
    fn into(self) -> Option<u8> {
        match self {
            ParamValue::U8(v) => Some(v),
            _=>None
        }
    }
}



impl Into<Option<u16>> for ParamValue {
    fn into(self) -> Option<u16> {
        match self {
            ParamValue::U16(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<u32>> for ParamValue {
    fn into(self) -> Option<u32> {
        match self {
            ParamValue::U32(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<u64>> for ParamValue {
    fn into(self) -> Option<u64> {
        match self {
            ParamValue::U64(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<bool>> for ParamValue {
    fn into(self) -> Option<bool> {
        match self {
            ParamValue::Bool(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<String>> for ParamValue {
    fn into(self) -> Option<String> {
        match self {
            ParamValue::String(v) => Some(v),
            _=>None
        }
    }
}

impl Into<Option<DateTime<Local>>> for ParamValue {
    fn into(self) -> Option<DateTime<Local>> {
        match self {
            ParamValue::DateTime(v) => Some(v),
            _=>None
        }
    }
}

impl ParamValue{
    
    pub fn is_null(&self) -> bool {
        match self { 
            ParamValue::Null => true,
            _ => false,
        }
    }

    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }
    
    pub fn to_string(&self) -> String{
        match self {
            ParamValue::U64(x) => x.to_string(),
            ParamValue::U32(x) => x.to_string(),
            ParamValue::U16(x) => x.to_string(),
            ParamValue::U8(x) => x.to_string(),
            ParamValue::I64(x) => x.to_string(),
            ParamValue::I32(x) => x.to_string(),
            ParamValue::I16(x) => x.to_string(),
            ParamValue::I8(x) => x.to_string(),
            ParamValue::USize(x) => x.to_string(),
            ParamValue::Bool(x) => x.to_string(),
            ParamValue::String(x) => x.to_string(),
            ParamValue::DateTime(x) => x.to_string(),
            ParamValue::Blob(_) => String::new(),
            ParamValue::Clob(x) => String::from_utf8(x.to_vec()).unwrap(),
            ParamValue::Null => "null".to_string(),
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            ParamValue::U64(x) => Some(*x),
            ParamValue::U32(x) => Some(*x as u64),
            ParamValue::U16(x) => Some(*x as u64),
            ParamValue::U8(x) => Some(*x as u64),
            ParamValue::I64(x) => Some(*x as u64),
            ParamValue::I32(x) => Some(*x as u64),
            ParamValue::I16(x) => Some(*x as u64),
            ParamValue::I8(x) => Some(*x as u64),
            ParamValue::USize(x) => Some(*x as u64),
            ParamValue::Bool(_) => None,
            ParamValue::String(_) => None,
            ParamValue::DateTime(_) => None,
            ParamValue::Blob(_) => None,
            ParamValue::Clob(_) => None,
            ParamValue::Null => None,
        }
    }
}
