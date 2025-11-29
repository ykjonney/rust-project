// Lua 虚拟机（Virtual Machine）模块
// 负责执行由解析器生成的字节码
// 维护全局变量表、运行栈、执行环境状态

use crate::value::Value;
use std::collections::HashMap;
use crate::parse::ParseProto;

/// 内置库函数：print() 的实现
/// 输出栈上索引 1 的值（第一个参数）
fn lib_print(state:&mut ExeState)-> i32{
    println!("{}", state.stack[1]);
    0
}

/// 虚拟机执行状态结构体
/// - `globals`: 全局变量表，存储全局变量与内置函数
/// - `stack`: 运行栈，存储临时变量、函数调用时的本地变量、参数等
pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl ExeState {
    /// 创建新虚拟机实例，初始化全局变量表与内置函数
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        globals.insert("print".to_string(), Value::Function(lib_print));
        Self {
            globals,
            stack: Vec::new(),
        }
    }

    /// 执行字节码：遍历 ParseProto 中的字节码序列并逐个执行
    pub fn execute(&mut self,proto:&ParseProto){
        // TODO: 实现字节码执行循环
        // 依据 ByteCode 类型分发到对应的处理函数
    }
}