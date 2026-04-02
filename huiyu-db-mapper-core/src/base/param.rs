use rust_decimal::prelude::ToPrimitive;
use rust_decimal::prelude::FromPrimitive;
use std::str::FromStr;
use crate::base::error::DatabaseError;
use crate::util::time_util;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub enum ParamValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    DateTime(DateTime<Local>),
    Blob(Vec<u8>),
    Clob(Vec<u8>),
    Decimal(Decimal),
    Null,
}

/////
/////  其他类型->ParamValue
/////
/////
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
    f32 => F32,
    f64 => F64,
    bool => Bool,
}

impl_param_value!(clone
    String => String,
    DateTime<Local> => DateTime,
    Vec<u8> => Blob,
    Decimal => Decimal,
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
    value
        .as_ref()
        .map(|v| v.clone().into())
        .unwrap_or(ParamValue::Null)
}

/////
///// ParamValue->其他类型
/////
/////
// 基础数值转换宏（只支持值类型，支持 Option 和非 Option）
macro_rules! impl_numeric_conversions {
        // 单个源类型（不需要转换）
    ($target:ty: $src:ident) => {
        impl From<ParamValue> for Option<$target> {
            fn from(val: ParamValue) -> Self {
                match val {
                    ParamValue::$src(v) => Some(v),
                    _ => None,
                }
            }
        }

        // 2. 值类型 -> T (Result, 非 Option)
        impl From<ParamValue> for $target where $target: Default {
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<$target> = val.into();
                result.unwrap_or_default()
            }
        }

    };
    // 支持多个源类型的转换
    ($target:ty: $($src:ident),+ $(,)?) => {

        impl From<ParamValue> for $target where $target: Default{
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<$target> = val.into();
                result.unwrap_or_default()
            }
        }
        // 2. 从 ParamValue 到 Option<$target> 的转换（总是成功）
        impl From<ParamValue> for Option<$target> {
            fn from(val: ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => Some(v as $target),
                    )+
                    ParamValue::String(v) => Some(v.parse::<$target>().unwrap_or_default()),
                    ParamValue::Bool(v)=>{
                        if v {
                            Some(1 as $target)
                        } else {
                            Some(0 as $target)
                        }
                    }
                    _ => None,
                }
            }
        }

        // 3. 从引用 ParamValue 到 Option<$target> 的转换（方便使用）
        impl From<&ParamValue> for Option<$target> {
            fn from(val: &ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => Some(*v as $target),
                    )+
                    ParamValue::String(v) => Some(v.parse::<$target>().unwrap_or_default()),
                    ParamValue::Bool(v)=>{
                        if *v {
                            Some(1 as $target)
                        } else {
                            Some(0 as $target)
                        }
                    },
                    _ => None,
                }
            }
        }
    };


}

// 浮点数转换宏
macro_rules! impl_float_conversions {
    ($target:ty: $($src:ident),+ $(,)?) => {
        impl From<ParamValue> for Option<$target> {
            fn from(val: ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => Some(v as $target),
                    )+
                    ParamValue::Decimal(v)=>Some(v.to_f64().unwrap_or_default() as $target),
                    ParamValue::String(v)=>Some(v.parse::<$target>().unwrap_or_default()),
                    _ => None,
                }
            }
        }
        impl From<ParamValue> for $target where $target: Default {
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<$target> = val.into();
                result.unwrap_or_default()
            }
        }
    };
}

// decimal转换宏
macro_rules! impl_decimal_conversions {
    ($target:ty: $($src:ident),+ $(,)?) => {
        impl From<ParamValue> for Option<$target> {
            fn from(val: ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => Some(v.into()),
                    )+
                    ParamValue::F64(v)=>Decimal::from_f64(v),
                    ParamValue::F32(v)=>Decimal::from_f32(v),
                    ParamValue::Decimal(v)=>Some(v),
                    ParamValue::String(v)=>Some(Decimal::from_str(v.as_str()).unwrap_or_default()),
                    _ => None,
                }
            }
        }
        impl From<ParamValue> for $target where $target: Default {
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<$target> = val.into();
                result.unwrap_or_default()
            }
        }
    };
}

macro_rules! impl_bool_conversions {
    (bool: $($src:ident),+ $(,)?) => {
        impl From<ParamValue> for Option<bool> {
            fn from(val: ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => {
                            if v > 0{
                                Some(true)
                            }else {
                                Some(false)
                            }
                        },

                    )+
                    ParamValue::Bool(v)=>{
                        Some(v)
                    },
                    ParamValue::String(v)=>{
                        Some(v.parse::<bool>().unwrap_or_default())
                    },
                    _ => None,
                }
            }
        }
        impl From<ParamValue> for bool where bool: Default {
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<bool> = val.into();
                result.unwrap_or_default()
            }
        }
    };
}
macro_rules! impl_date_conversions {
    (date: $($src:ident),+ $(,)?) => {
        impl From<ParamValue> for Option<DateTime<Local>> {
            fn from(val: ParamValue) -> Self {
                match val {
                    $(
                        ParamValue::$src(v) => {
                            // 数字作为秒值转日期
                            Some(time_util::create_datetime_local_from_seconds(v as i64))
                        },

                    )+
                    ParamValue::String(v)=>{
                        // 字符串转Date
                        let res = time_util::format_date_time_local_from_str(v.as_str(), "%Y-%m-%d %H:%M:%S");
                        if res.is_ok() {
                            return Some(res.unwrap());
                        }
                        None
                    },
                    _ => None,
                }
            }
        }
        impl From<ParamValue> for DateTime<Local> where DateTime<Local>: Default {
            fn from(val: ParamValue) -> Self {
                // 不能在重复模式中直接使用 $target
                let result: Option<DateTime<Local>> = val.into();
                result.unwrap_or_default()
            }
        }
    };
}

// 使用宏
impl_numeric_conversions!(i8: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(i16: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(i32: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(i64: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(usize: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(u8: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(u16: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(u32: I8, I16, I32, I64, U8, U16, U32, U64);
impl_numeric_conversions!(u64: I8, I16, I32, I64, U8, U16, U32, U64);

// 浮点数
impl_float_conversions!(f32: F64, F32, I8, I16, I32, I64, U8, U16, U32, U64);
impl_float_conversions!(f64: F64, F32, I8, I16, I32, I64, U8, U16, U32, U64);
impl_decimal_conversions!(Decimal: I8, I16, I32, I64, U8, U16, U32, U64);

// 布尔值
impl_bool_conversions!(bool: I8, I16, I32, I64, U8, U16, U32, U64);

// 字符串（特殊处理）
// 1. 值类型 -> Option<String>
impl From<ParamValue> for Option<String> {
    fn from(val: ParamValue) -> Self {
        if val.is_null() {
            None
        } else {
            Some(val.to_string())
        }
    }
}

// 2. 值类型 -> String (Result, 非 Option)
impl From<ParamValue> for String {
    fn from(val: ParamValue) -> Self {
        if val.is_null() {
            "".to_string()
        } else {
            val.to_string()
        }
    }
}


// 日期时间
impl_date_conversions!(date: I8, I16, I32, I64, U8, U16, U32, U64);

// Blob (Vec<u8>)
// 1. 值类型 -> Option<Vec<u8>>

impl From<ParamValue> for Option<Vec<u8>> {
    fn from(val: ParamValue) -> Self {
        match val {
            ParamValue::Blob(v) | ParamValue::Clob(v) => Some(v),
            _ => None,
        }
    }
}

impl From<ParamValue> for Vec<u8> {
    fn from(val: ParamValue) -> Self {
        match val {
            ParamValue::Blob(v) | ParamValue::Clob(v) => v,
            _ => Vec::new(),
        }
    }
}
// 2. 值类型 -> Vec<u8> (Result, 非 Option)

impl ParamValue {
    pub fn is_null(&self) -> bool {
        match self {
            ParamValue::Null => true,
            _ => false,
        }
    }

    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }

    pub fn to_string(&self) -> String {
        match self {
            ParamValue::U64(x) => x.to_string(),
            ParamValue::U32(x) => x.to_string(),
            ParamValue::U16(x) => x.to_string(),
            ParamValue::U8(x) => x.to_string(),
            ParamValue::I64(x) => x.to_string(),
            ParamValue::I32(x) => x.to_string(),
            ParamValue::I16(x) => x.to_string(),
            ParamValue::I8(x) => x.to_string(),
            ParamValue::Bool(x) => x.to_string(),
            ParamValue::String(x) => x.to_string(),
            ParamValue::DateTime(x) => x.to_string(),
            ParamValue::Blob(x) => String::from_utf8(x.to_vec()).unwrap_or_default(),
            ParamValue::Clob(x) => String::from_utf8(x.to_vec()).unwrap_or_default(),
            ParamValue::Null => "null".to_string(),
            ParamValue::F32(x) => x.to_string(),
            ParamValue::F64(x) => x.to_string(),
            ParamValue::Decimal(x) => x.to_string(),
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

pub trait IntoParamValue {
    fn into_param_value(self) -> ParamValue;
}

impl<T> IntoParamValue for T
where
    T: Into<ParamValue>,
{
    fn into_param_value(self) -> ParamValue {
        self.into()
    }
}
