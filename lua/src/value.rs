// Lua 值类型定义与操作
// 支持多种值类型，包括基本类型（nil, boolean, integer, float）与字符串优化
// 字符串采用分层存储以优化空间使用：短字符串直接存储、中等/长字符串用引用计数

use crate::vm::ExeState;
use std::rc::Rc;
use std::fmt;

/// 短字符串的最大长度（优化：直接在 Value 中存储小字符串）
const SHORT_STR_MAX: usize = 14; //sizeof(Value) - 1(tag)-1(len)
/// 中等字符串的最大长度
const MID_STR_MAX: usize = 48-1;

/// Lua 值类型枚举
/// 采用分层字符串存储以平衡空间与性能
#[derive(Clone)]
pub enum Value {
    // String(String),  // 原始方案（已弃用）
    /// 函数值：指向虚拟机内置函数的指针
    Function(fn(&mut ExeState) -> i32),
    /// 布尔值
    Boolean(bool),
    /// 64 位整数
    Integer(i64),
    /// 64 位浮点数
    Float(f64),
    /// 空值
    Nil,
    /// 短字符串（直接存储在 Value 中）：(长度, 数据)
    ShortStr(u8, [u8; SHORT_STR_MAX]),
    /// 中等字符串（引用计数）
    MidStr(Rc<(u8, [u8; MID_STR_MAX])>),
    /// 长字符串（引用计数 String）
    LongStr(Rc<String>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(n) => write!(f, "{n:?}"),
            Value::ShortStr(l,s,) => write!(f, "{l},{s:?}"),
            Value::MidStr(s) => write!(f, "{s:?}"),
            Value::LongStr(s) => write!(f, "{s:?}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => *a == *b,
            (Value::Integer(a), Value::Integer(b)) => *a == *b,
            (Value::Float(a), Value::Float(b)) => *a == *b,
            (Value::ShortStr(a,b), Value::ShortStr(c,d)) => *b == *d,
            (Value::Function(a), Value::Function(b)) => std::ptr::eq(a, b),
            _ => false,
        }
    }
}


impl From<String> for Value {
    fn from(value: String) -> Self {
        let len = value.len();
        if len <= SHORT_STR_MAX {
            let mut buf = [0u8; SHORT_STR_MAX];
            buf[..len].copy_from_slice(value.as_bytes());
            Value::ShortStr(len as u8, buf)
        }else if len <= MID_STR_MAX {
            let mut buf = [0u8; MID_STR_MAX];
            buf[..len].copy_from_slice(value.as_bytes());
            Value::MidStr(Rc::new((len as u8, buf)))
        }else {
            Value::LongStr(Rc::new(value))
        }
    }
    
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
    
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<fn(&mut ExeState) -> i32> for Value {
    fn from(value: fn(&mut ExeState) -> i32) -> Self {
        Value::Function(value)
    }
}