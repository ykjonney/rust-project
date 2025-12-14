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
    pub fn execute<R: std::io::Read>(&mut self,proto:&ParseProto<R>){
        // 简单字节码执行循环实现说明：
        // - 栈（stack）被用作寄存器文件：字节码中的寄存器索引直接对应 stack 的位置
        // - 在访问某个寄存器前，保证 stack 有足够长度（不足则用 Value::Nil 扩展）
        // - 支持的指令包括：LoadConst/LoadNil/LoadBool/LoadInt/Move/Call/GetGlobal/SetGlobal*/等
        // - 目前只支持内置函数调用（Value::Function），以及通过全局表读写变量

        use crate::bytecode::ByteCode;

        // 辅助：确保 stack 至少有 len 个槽
        fn ensure_stack(state: &mut ExeState, len: usize) {
            while state.stack.len() <= len {
                state.stack.push(Value::Nil);
            }
        }

        // 辅助：从 Value 中提取字符串（仅用于全局变量名）
        fn value_to_string(v: &Value) -> Option<String> {
            match v {
                Value::ShortStr(len, buf) => {
                    let l = *len as usize;
                    let s = std::str::from_utf8(&buf[..l]).ok()?.to_string();
                    Some(s)
                }
                Value::MidStr(rc) => {
                    let (len, buf) = &**rc;
                    let l = *len as usize;
                    let s = std::str::from_utf8(&buf[..l]).ok()?.to_string();
                    Some(s)
                }
                Value::LongStr(rcs) => Some((**rcs).clone()),
                _ => None,
            }
        }

        for code in &proto.byte_codes {
            match code {
                ByteCode::LoadConst(dst, idx) => {
                    let const_idx = *idx as usize;
                    let val = proto.constants[const_idx].clone();
                    ensure_stack(self, *dst as usize);
                    self.stack[*dst as usize] = val;
                }
                ByteCode::LoadNil(dst) => {
                    ensure_stack(self, *dst as usize);
                    self.stack[*dst as usize] = Value::Nil;
                }
                ByteCode::LoadBool(dst, b) => {
                    ensure_stack(self, *dst as usize);
                    self.stack[*dst as usize] = Value::Boolean(*b);
                }
                ByteCode::LoadInt(dst, n) => {
                    ensure_stack(self, *dst as usize);
                    self.stack[*dst as usize] = Value::Integer(*n as i64);
                }
                ByteCode::Move(dst, src) => {
                    ensure_stack(self, *dst as usize);
                    ensure_stack(self, *src as usize);
                    self.stack[*dst as usize] = self.stack[*src as usize].clone();
                }
                ByteCode::GetGlobal(dst, cidx) => {
                    // cidx 是常数池中保存变量名的索引
                    let name = proto.constants[*cidx as usize].clone();
                    if let Some(key) = value_to_string(&name) {
                        let val = self.globals.get(&key).cloned().unwrap_or(Value::Nil);
                        ensure_stack(self, *dst as usize);
                        self.stack[*dst as usize] = val;
                    } else {
                        ensure_stack(self, *dst as usize);
                        self.stack[*dst as usize] = Value::Nil;
                    }
                }
                ByteCode::SetGlobal(dst_const, src_reg) => {
                    // dst_const: 常数池中保存目标全局变量名的索引
                    let name = proto.constants[*dst_const as usize].clone();
                    if let Some(key) = value_to_string(&name) {
                        ensure_stack(self, *src_reg as usize);
                        let val = self.stack[*src_reg as usize].clone();
                        self.globals.insert(key, val);
                    }
                }
                ByteCode::SetGlobalConst(dst_const, cidx) => {
                    let name = proto.constants[*dst_const as usize].clone();
                    if let Some(key) = value_to_string(&name) {
                        let val = proto.constants[*cidx as usize].clone();
                        self.globals.insert(key, val);
                    }
                }
                ByteCode::SetGlobalGlobal(dst_const, src_const) => {
                    let name_dst = proto.constants[*dst_const as usize].clone();
                    let name_src = proto.constants[*src_const as usize].clone();
                    if let (Some(kd), Some(ks)) = (value_to_string(&name_dst), value_to_string(&name_src)) {
                        if let Some(v) = self.globals.get(&ks) {
                            self.globals.insert(kd, v.clone());
                        } else {
                            self.globals.insert(kd, Value::Nil);
                        }
                    }
                }
                ByteCode::Call(arg_reg, _ret) => {
                    // 约定：函数位于 arg_reg-1，参数从 arg_reg 开始
                    let arg_index = *arg_reg as usize;
                    if arg_index == 0 { continue; }
                    let func_reg = arg_index - 1;
                    ensure_stack(self, func_reg);
                    match &self.stack[func_reg] {
                        Value::Function(f) => {
                            // 内置函数接口为 fn(&mut ExeState) -> i32
                            // 被调用函数可以直接访问 state.stack 中的参数
                            let _ret_count = f(self);
                            // 返回值处理：当前简化为不设置返回寄存器
                        }
                        _ => {
                            // 非函数值调用，忽略或报错（这里仅打印）
                            eprintln!("attempt to call non-function value");
                        }
                    }
                }
            }
        }
    }
}