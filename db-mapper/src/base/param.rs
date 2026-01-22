use chrono::{DateTime, Local};
use crate::base::error::DatabaseError;
use crate::base::error::DatabaseError::ConvertError;

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
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    DateTime(DateTime<Local>),
    Blob(Vec<u8>),
    Clob(Vec<u8>),
    Null
}

// 支持所有类型的宏版本
macro_rules! impl_param_value {
    // 基本数值类型
    ($($type:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$type> for ParamValue {
                fn from(value: $type) -> Self {
                    ParamValue::$variant(value)
                }
            }

            impl From<&$type> for ParamValue {
                fn from(value: &$type) -> Self {
                    ParamValue::$variant(*value)
                }
            }
        )*
    };

    // 需要 Clone 的类型
    (clone $($type:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$type> for ParamValue {
                fn from(value: $type) -> Self {
                    ParamValue::$variant(value)
                }
            }

            impl From<&$type> for ParamValue {
                fn from(value: &$type) -> Self {
                    ParamValue::$variant(value.clone())
                }
            }
        )*
    };

    // 需要特殊处理的类型
    (special $type:ty => $variant:ident, $convert:expr) => {
        impl From<$type> for ParamValue {
            fn from(value: $type) -> Self {
                ParamValue::$variant($convert(value))
            }
        }
    };
}

// 使用
impl_param_value! {
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    usize => USize,
    f32 => F32,
    f64 => F64,
    bool => Bool,
}

impl_param_value!(clone
    String => String,
    DateTime<Local> => DateTime,
    Vec<u8> => Blob,
);

// &str 特殊处理
impl From<&str> for ParamValue {
    fn from(s: &str) -> Self {
        ParamValue::String(s.to_string())
    }
}

// 为 Option 提供便捷方法
pub fn get_param_value<T>(value: Option<T>) -> ParamValue
where
    T: Into<ParamValue>,
{
    value.map(Into::into).unwrap_or(ParamValue::Null)
}

// 为 &Option 也提供
pub fn get_param_value_ref<T>(value: &Option<T>) -> ParamValue
where
    T: Clone + Into<ParamValue>,
{
    value.as_ref().map(|v| v.clone().into()).unwrap_or(ParamValue::Null)
}


// 基础数值转换宏（只支持值类型，支持 Option 和非 Option）
macro_rules! impl_numeric_conversions {
    // 支持多个源类型的转换
    ($target:ty: $($src:ident),+ $(,)?) => {
        // 1. 值类型 -> Option<T>
        impl Into<Option<$target>> for ParamValue {
            fn into(self) -> Option<$target> {
                match self {
                    $(
                        ParamValue::$src(v) => Some(v as $target),
                    )+
                    _ => None,
                }
            }
        }

        // 2. 值类型 -> T (Result, 非 Option)
        impl TryInto<$target> for ParamValue {
            type Error = crate::base::error::DatabaseError;
            
            fn try_into(self) -> Result<$target, Self::Error> {
                match self {
                    $(
                        ParamValue::$src(v) => Ok(v as $target),
                    )+
                    _ => Err(ConvertError(format!("{} can't transfer to {}", self.to_string(), std::any::type_name::<$target>()))),
                }
            }
        }
    };

    // 单个源类型（不需要转换）
    ($target:ty: $src:ident) => {
        // 1. 值类型 -> Option<T>
        impl Into<Option<$target>> for ParamValue {
            fn into(self) -> Option<$target> {
                match self {
                    ParamValue::$src(v) => Some(v),
                    _ => None,
                }
            }
        }

        // 2. 值类型 -> T (Result, 非 Option)
        impl TryInto<$target> for ParamValue {
            type Error = ConvertError;

            fn try_into(self) -> Result<$target, Self::Error> {
                match self {
                    ParamValue::$src(v) => Ok(v),
                    _ => Err(ConvertError(format!("{} can't transfer to {}", self.to_string(), std::any::type_name::<$target>()))),
                }
            }
        }
    };
}

// 浮点数转换宏
macro_rules! impl_float_conversions {
    ($target:ty: $($src:ident),+ $(,)?) => {
        // 1. 值类型 -> Option<T>
        impl Into<Option<$target>> for ParamValue {
            fn into(self) -> Option<$target> {
                match self {
                    $(
                        ParamValue::$src(v) => Some(v as $target),
                    )+
                    _ => None,
                }
            }
        }

        // 2. 值类型 -> T (Result, 非 Option)
        impl TryInto<$target> for ParamValue {
            type Error = DatabaseError;

            fn try_into(self) -> Result<$target, Self::Error> {
                match self {
                    $(
                        ParamValue::$src(v) => Ok(v as $target),
                    )+
                    _ => Err(ConvertError(format!("{} can't transfer to {}", self.to_string(), std::any::type_name::<$target>()))),
                }
            }
        }
    };
}

// 使用宏
impl_numeric_conversions!(i8: I8, U8);
impl_numeric_conversions!(i16: I8, I16, U8, U16);
impl_numeric_conversions!(i32: I8, I16, I32, U8, U16, U32);
impl_numeric_conversions!(i64: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(usize: I8, I16, I32, U8, U16, U32, USize);
impl_numeric_conversions!(u8: U8);
impl_numeric_conversions!(u16: U8, U16);
impl_numeric_conversions!(u32: U8, U16, U32);
impl_numeric_conversions!(u64: U8, U16, U32, U64);

// 浮点数
impl_float_conversions!(f32: F32, I8, I16, I32, I64, U8, U16, U32, U64);
impl_float_conversions!(f64: F64, F32, I8, I16, I32, I64, U8, U16, U32, U64);

// 布尔值
impl_numeric_conversions!(bool: Bool);

// 字符串（特殊处理）
// 1. 值类型 -> Option<String>
impl Into<Option<String>> for ParamValue {
    fn into(self) -> Option<String> {
        Some(self.to_string())
    }
}

// 2. 值类型 -> String (Result, 非 Option)
impl TryInto<String> for ParamValue {
    type Error = DatabaseError;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.to_string())
    }
}

// 日期时间
impl_numeric_conversions!(DateTime<Local>: DateTime);

// Blob (Vec<u8>)
// 1. 值类型 -> Option<Vec<u8>>
impl Into<Option<Vec<u8>>> for ParamValue {
    fn into(self) -> Option<Vec<u8>> {
        match self {
            ParamValue::Blob(v) | ParamValue::Clob(v) => Some(v),
            _ => None,
        }
    }
}

// 2. 值类型 -> Vec<u8> (Result, 非 Option)
impl TryInto<Vec<u8>> for ParamValue {
    type Error = DatabaseError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            ParamValue::Blob(v) | ParamValue::Clob(v) => Ok(v),
            _ => Err(ConvertError(format!("{} can't transfer to Vec<u8>", self.to_string()))),
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
            ParamValue::F32(x)=>x.to_string(),
            ParamValue::F64(x)=>x.to_string()
        }
    }

    // 转换为 Option<T>
    pub fn as_option<T>(self) -> Option<T>
    where
        Self: Into<Option<T>>,
    {
        self.into()
    }

    // 尝试转换为 T
    pub fn try_as<T>(self) -> Result<T, DatabaseError>
    where
        Self: TryInto<T, Error = DatabaseError>,
    {
        self.try_into()
    }

    // 安全的数值转换，提供默认值
    pub fn as_number_or<T>(self, default: T) -> T
    where
        T: Default,
        Self: Into<Option<T>>,
    {
        self.into().unwrap_or(default)
    }

    // 检查是否可以转换为指定类型
    pub fn can_convert_to<T>(&self) -> bool
    where
        Self: Clone + Into<Option<T>>,
    {
        self.clone().into().is_some()
    }
}