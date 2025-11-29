// Lua 语法分析器（Parser）模块
// 负责将 Token 流转换为字节码（ByteCode）序列
// 采用递归下降解析器（Recursive Descent Parser）模式，支持以下语法构造：
// - 赋值语句（assignment）：name = expression
// - 局部变量声明（local statement）：local var = expression
// - 函数调用（function call）：func(args)
// - 表达式解析（expression）：常数、变量名、一元运算等
// 最终生成可被虚拟机执行的字节码

use crate::bytecode::ByteCode;
use crate::lex::Lex;
use crate::lex::Token;
use crate::value::Value;
use std::fs::File;

/// Lua 解析原型结构体
/// - `constants`: 常数池，存储所有字面量常数（整数、浮点、字符串等）
/// - `byte_codes`: 生成的字节码序列，待虚拟机执行
/// - `locals`: 局部变量表，记录当前作用域内声明的所有变量名
/// - `lex`: 词法分析器实例，提供 Token 流
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
    locals: Vec<String>,
    lex: Lex,
}

impl ParseProto {
    /// 从文件加载并解析 Lua 源代码，返回包含字节码的 ParseProto
    /// 主入口函数：初始化解析器并调用 chunk() 开始递归解析
    pub fn load(input: File) -> Self {
        let mut proto = Self {
            constants: Vec::new(),
            byte_codes: Vec::new(),
            locals: Vec::new(),
            lex: Lex::new(input),
        };

        proto.chunk();

        proto
    }

    /// 块（chunk）解析函数：递归下降解析的顶层入口
    /// 循环读取 Token 并根据类型分发到相应的语句解析函数：
    /// - Name Token：检查后续是否为赋值 (=) 或函数调用
    /// - Local Token：本地变量声明
    /// - Eos Token：文件尾，终止解析
    fn chunk(&mut self) {
        loop {
            match self.lex.next() {
                Token::Name(name) => {
                    if self.lex.peek() == &Token::Assign {
                        self.assignment(name);
                    } else {
                        self.function_call(name);
                    }
                }
                Token::Local => self.local(),
                Token::Eos => break,
                t => panic!("unexpected token {:?}", t),
            }
        }
    }

    /// 向常数池中添加常数，若常数已存在则返回其索引，否则添加并返回新索引
    /// 用于消除常数重复，实现常数池复用
    fn add_const(&mut self, c: Value) -> usize {
        self.constants
            .iter()
            .position(|x| *x == c)
            .unwrap_or_else(|| {
                self.constants.push(c);
                self.constants.len() - 1
            })
    }

    /// 生成 LoadConst 字节码：将索引 c 处的常数加载到目标寄存器 dst
    fn load_const(&mut self, dst: usize, c: Value) -> ByteCode {
        ByteCode::LoadConst(dst as u8, self.add_const(c) as u16)
    }

    /// 表达式解析函数：识别并加载不同类型的表达式到目标寄存器 dst
    /// 支持的表达式类型：
    /// - Nil、True、False：常数 Token
    /// - Integer、Float、String：字面量常数
    /// - Name：变量加载
    /// 生成相应的字节码并推入 byte_codes 序列
    fn load_exp(&mut self, dst: usize) {
        let code = match self.lex.next() {
            Token::Nil => ByteCode::LoadNil(dst as u8),
            Token::True => ByteCode::LoadBool(dst as u8, true),
            Token::False => ByteCode::LoadBool(dst as u8, false),
            Token::Integer(i) => {
                if let Ok(n) = i16::try_from(i) {
                    ByteCode::LoadInt(dst as u8, n)
                } else {
                    self.load_const(dst, Value::Integer(i))
                }
            }
            Token::Float(f) => self.load_const(dst, Value::Float(f)),
            Token::String(s) => self.load_const(dst, Value::String(s)),
            Token::Name(name) => self.load_var(dst, name),
            _ => panic!("unexpected token"),
        };
        self.byte_codes.push(code);
    }

    /// 本地变量声明处理：local var = expression
    /// 流程：
    /// 1. 读取变量名
    /// 2. 期望 '=' Token
    /// 3. 将表达式加载到寄存器（索引为当前 locals 长度）
    /// 4. 将变量名添加到 locals 表
    fn local(&mut self) {
        let var = if let Token::Name(var) = self.lex.next() {
            var
        } else {
            panic!("expected variable name")
        };
        if self.lex.next() != Token::Assign {
            panic!("expected assign");
        }
        self.load_exp(self.locals.len());
        self.locals.push(var);
    }

    /// 函数调用处理：func(args) 或 func "string"
    /// 流程：
    /// 1. 加载函数变量到寄存器 ifunc
    /// 2. 加载参数（括号表达式 或 字符串直接调用）到寄存器 iargs
    /// 3. 生成 Call 字节码
    fn function_call(&mut self, name: String) {
        let ifunc = self.locals.len();
        let iargs = ifunc + 1;
        //function,variable
        let code = self.load_var(ifunc, name);
        self.byte_codes.push(code);
        // argument,(exp) or "string"
        match self.lex.next() {
            Token::ParL => {
                self.load_exp(iargs);
                if self.lex.next() != Token::ParR {
                    panic!("expected )")
                }
            }
            Token::String(s) => {
                let code = self.load_const(iargs, Value::String(s));
                self.byte_codes.push(code);
            }
            _ => panic!("expected ( or string"),
        }
        self.byte_codes.push(ByteCode::Call(iargs as u8, 1));
    }

    /// 获取本地变量的索引（在 locals 表中的位置）
    /// 返回 Some(index) 若变量存在，否则返回 None
    fn get_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().position(|x| x == name)
    }

    /// 变量加载：若变量在 locals 表中则生成 Move 字节码，否则作为全局变量处理
    fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
        if let Some(i) = self.get_local(&name) {
            ByteCode::Move(dst as u8, i as u8)
        } else {
            let ic = self.add_const(Value::String(name));
            ByteCode::LoadConst(dst as u8, ic as u16)
        }
    }

    fn assignment(&mut self, var: String) {
        self.lex.next();
        //先判断变量是否重复，重复就覆盖
        if let Some(i) = self.get_local(&var) {
            self.load_exp(i);
        } else {
            //先判断全局变量，返回索引
            let dst = self.add_const(Value::String(var)) as u8;
            let code = match self.lex.next() {
                // from const values
                Token::Nil => ByteCode::SetGlobalConst(dst, self.add_const(Value::Nil) as u8),
                Token::True => {
                    ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(true)) as u8)
                }
                Token::False => {
                    ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(false)) as u8)
                }
                Token::Integer(i) => {
                    ByteCode::SetGlobalConst(dst, self.add_const(Value::Integer(i)) as u8)
                }
                Token::Float(f) => {
                    ByteCode::SetGlobalConst(dst, self.add_const(Value::Float(f)) as u8)
                }
                Token::String(s) => {
                    ByteCode::SetGlobalConst(dst, self.add_const(Value::String(s)) as u8)
                }
                //from variable
                Token::Name(var) => {
                    if let Some(i) = self.get_local(&var) {
                        //local var
                        ByteCode::SetGlobal(dst, i as u8)
                    } else {
                        //global var
                        ByteCode::SetGlobalGlobal(dst, self.add_const(Value::String(var)) as u8)
                    }
                }
                _ => panic!("unexpected token"),
            };
            self.byte_codes.push(code);
        }
    }
}
