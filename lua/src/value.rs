// Lua 值类型定义与操作
// 支持多种值类型，包括基本类型（nil, boolean, integer, float）与字符串优化
// 字符串采用分层存储以优化空间使用：短字符串直接存储、中等/长字符串用引用计数

use crate::vm::ExeState;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

/// 短字符串的最大长度（优化：直接在 Value 中存储小字符串）
const SHORT_STR_MAX: usize = 14; //sizeof(Value) - 1(tag)-1(len)
/// 中等字符串的最大长度
const MID_STR_MAX: usize = 48-1;


pub struct Table{
    pub array: Vec<Value>,
    pub map:HashMap<Value,Value>,
}
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
    Table(Rc<RefCell<Table>>),
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
            Value::Table(t) => write!(f, "table {:?}", Rc::as_ptr(t)),
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

impl Hash for Value {
    fn hash<H:Hasher>(&self,state:&mut H){
        match self {
            Value::Nil => {
                0u8.hash(state);
            },
            Value::Boolean(b) => {
                1u8.hash(state);
                b.hash(state);
            },
            Value::Integer(i) => {
                2u8.hash(state);
                i.hash(state);
            },
            Value::Float(f) => {
                3u8.hash(state);
                //将f64转为u64进行hash
                let bits = f.to_bits();
                bits.hash(state);
            },
            Value::ShortStr(_,s) => {
                4u8.hash(state);
                s.hash(state);
            },
            Value::Function(f) => {
                5u8.hash(state);
                let ptr = *f as usize;
                ptr.hash(state);
            },
            Value::Table(t) => {
                6u8.hash(state);
                let ptr = Rc::as_ptr(t) as usize;
                ptr.hash(state);
            },
            _ => {
                panic!("unhashable value");
            }
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