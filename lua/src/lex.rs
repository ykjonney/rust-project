// Lua 词法分析器（Lexer）模块
// 负责将 Lua 源代码逐个字符读取并转换为 Token 流（Token 序列）
// 支持识别 Lua 的所有关键字、操作符、常数（数字、字符串）与标识符
// 使用单字符向前查看（lookahead）机制实现高效的多字符 Token 识别

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

/// Lua Token 类型枚举
/// 包括 Lua 语言的 24 个关键字、22 个操作符/分隔符，以及 4 种常数类型和 1 个文件尾标记
#[derive(Debug, PartialEq)]
pub enum Token {
    // Lua 关键字（24 个）
    And,      // and
    Break,    // break
    Do,       // do
    Else,     // else
    Elseif,   // elseif
    End,      // end
    False,    // false
    For,      // for
    Function, // function
    Goto,     // goto
    If,       // if
    In,       // in
    Local,    // local
    Nil,      // nil
    Not,      // not
    Or,       // or
    Repeat,   // repeat
    Return,   // return
    Then,     // then
    True,     // true
    Until,    // until
    While,    // while

    // 算术运算符
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Pow,      // ^
    Len,      // #
    // 位运算符
    BitAnd,   // &
    BitXor,   // ~
    BitOr,    // |
    ShiftL,   // <<
    ShiftR,   // >>
    Idiv,     // //（整除）
    // 比较运算符
    Equal,    // ==
    NotEq,    // ~=
    LesEq,    // <=
    GreEq,    // >=
    Less,     // <
    Greater,  // >
    Assign,   // =
    // 括号 & 分隔符
    ParL,     // (
    ParR,     // )
    CurlyL,   // {
    CurlyR,   // }
    SqurL,    // [
    SqurR,    // ]
    DoubColon, // ::
    // 其他分隔符
    SemiColon, // ;
    Colon,    // :
    Comma,    // ,
    Dot,      // .
    Concat,   // ..
    Dots,     // ...

    // 常数类型
    Integer(i64),   // 整数常量
    Float(f64),     // 浮点数常量
    String(String), // 字符串常量（已解析，不含引号）

    // 标识符（变量名或表键名）
    Name(String),

    // 文件尾标记
    Eos,
}

/// Lua 词法分析器结构体
/// - `input`: 输入文件流，用于逐字符读取源代码
/// - `ahead`: 预存的下一个 Token（向前查看机制），用于 `peek()` 和高效的 `next()` 实现
#[derive(Debug)]
pub struct Lex {
    input: File,
    ahead: Token,
}

impl Lex {
    /// 创建新的词法分析器实例，以指定的文件为输入源
    pub fn new(input: File) -> Self {
        Lex {
            input,
            ahead: Token::Eos,
        }
    }

    /// 获取下一个 Token
    /// 如果预存 Token（ahead）不为 Eos，则返回预存 Token 并清空预存区；
    /// 否则从文件中读取新 Token
    pub fn next(&mut self) -> Token {
        if self.ahead == Token::Eos {
            self.do_next()
        } else {
            mem::replace(&mut self.ahead, Token::Eos)
        }
    }

    /// 查看下一个 Token 而不消费它（向前查看 / Lookahead）
    /// 将 Token 缓存在 ahead 中以供后续 next() 使用
    pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::Eos {
            self.ahead = self.do_next();
        }
        &self.ahead
    }

    /// 主词法分析函数：读取下一个字符并根据其类型分发到相应的处理函数
    /// - 跳过空白字符（递归调用自身）
    /// - 识别单字符 Token（操作符、括号等）
    /// - 调用专用函数处理多字符 Token（数字、字符串、标识符等）
    /// 若遇到未知字符则 panic
    pub fn do_next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            '\n' | '\r' | '\t' | ' ' => self.do_next(),
            '+' => Token::Add,
            '*' => Token::Mul,
            '%' => Token::Mod,
            '^' => Token::Pow,
            '#' => Token::Len,
            '&' => Token::BitAnd,
            '|' => Token::BitOr,
            '(' => Token::ParL,
            ')' => Token::ParR,
            '{' => Token::CurlyL,
            '}' => Token::CurlyR,
            '[' => Token::SqurL,
            ']' => Token::SqurR,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '/' => self.check_ahead('/', Token::Idiv, Token::Div),
            '=' => self.check_ahead('=', Token::Equal, Token::Assign),
            '~' => self.check_ahead('=', Token::NotEq, Token::BitXor),
            ':' => self.check_ahead(':', Token::DoubColon, Token::Colon),
            '<' => self.check_ahead2('=', Token::LesEq, '<', Token::ShiftL, Token::Less),
            '>' => self.check_ahead2('=', Token::GreEq, '>', Token::ShiftR, Token::Greater),
            '\'' | '"' => self.read_string(ch),
            '.' => match self.read_char() {
                '.' => {
                    if self.read_char() == '.' {
                        Token::Dots
                    } else {
                        self.putback_char();
                        Token::Concat
                    }
                }
                '0'..='9' => {
                    self.putback_char();
                    self.read_number_fraction(0)
                }
                _ => {
                    self.putback_char();
                    Token::Dot
                }
            },
            '-' => {
                if self.read_char() == '-' {
                    self.read_comment();
                    self.do_next()
                } else {
                    self.putback_char();
                    Token::Sub
                }
            }
            '0'..='9' => self.read_number(ch),
            'A'..='Z' | 'a'..='z' | '_' => self.read_name(ch),
            '\0' => Token::Eos,
            _ => panic!("invalid char {ch}"),
        }
    }

    /// 从输入流读取一个字节，转为 char；若到达文件尾返回 '\\0'
    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 0 {
            '\0'
        } else {
            buf[0] as char
        }
    }

    /// 将文件位置指针回退一个字节（用于实现 lookahead）
    fn putback_char(&mut self) {
        self.input.seek(SeekFrom::Current(-1)).unwrap();
    }

    /// 向前查看 1 个字符：若匹配 ahead 则返回 long Token，否则回退并返回 short Token
    /// 用于识别两字符操作符（如 //, ==, <=, >=, ~=, ::）
    fn check_ahead(&mut self, ahead: char, long: Token, short: Token) -> Token {
        if self.read_char() == ahead {
            long
        } else {
            self.putback_char();
            short
        }
    }

    /// 向前查看 1 个字符：检测两种可能的后继字符
    /// 若第一个字符匹配 ahead1 则返回 long1，若匹配 ahead2 则返回 long2，否则回退返回 short
    /// 用于识别三字符操作符（如 <<, >>, 以及 <= / <）
    fn check_ahead2(
        &mut self,
        ahead1: char,
        long1: Token,
        ahead2: char,
        long2: Token,
        short: Token,
    ) -> Token {
        let ch = self.read_char();
        if ch == ahead1 {
            long1
        } else if ch == ahead2 {
            long2
        } else {
            self.putback_char();
            short
        }
    }

    /// 读取标识符或关键字（从 first 字符开始）
    /// 持续读取字母、数字、下划线直到遇到其他字符
    /// 然后通过关键字表匹配：若为关键字则返回对应 Token，否则返回 Name Token
    /// 性能考虑：当前用 match 字符串比较，TODO 建议用哈希表优化（参考注释）
    fn read_name(&mut self, first: char) -> Token {
        let mut s = first.to_string();
        s.push(first);
        loop {
            let ch = self.read_char();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                s.push(ch);
            } else {
                self.putback_char();
                break;
            }
        }
        match &s as &str {
            // TODO optimize by hash
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "end" => Token::End,
            "false" => Token::False,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "if" => Token::If,
            "in" => Token::In,
            "local" => Token::Local,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::True,
            "until" => Token::Until,
            "while" => Token::While,
            _ => Token::Name(s), // 不是关键字，作为标识符返回
        }
    }

    /// 读取并跳过注释
    /// Lua 支持两种注释形式：
    /// - 单行注释：-- 开头，读到行尾（'\\n' 或 '\\0'）
    /// - 多行注释：--[[ 开头，需要找 ]] 结尾（当前 TODO 未实现）
    fn read_comment(&mut self) {
        match self.read_char() {
            '[' => todo!("lex long comment"), // 多行注释：--[[...]] 尚未实现
            _ => loop {
                // 单行注释：读到换行或文件尾
                let ch = self.read_char();
                if ch == '\n' || ch == '\0' {
                    break;
                }
            },
        }
    }

    /// 读取字符串常量（由 quoto 字符：' 或 " 标记）
    /// 持续读取字符直到遇到结束引号，生成 String Token
    /// 性能考虑：
    /// - 当前不支持转义序列（\\t, \\n 等），有 TODO 标记
    /// - 到达文件尾而未找到闭合引号时 panic
    fn read_string(&mut self, quoto: char) -> Token {
        let mut s = String::new();
        loop {
            let ch = self.read_char();
            match ch {
                '\0' => panic!("unexpected end of file"),
                '\\' => todo!("escape"), // 转义序列尚未实现
                ch if ch == quoto => break, // 遇到结束引号
                ch => s.push(ch),
            }
        }
        Token::String(s)
    }

    /// 读取数字常量（整数或浮点数）
    /// first 为首字符（0-9）
    /// 处理流程：
    /// 1. 若以 0 开头且后跟 x/X，调用 read_heximal() 读十六进制
    /// 2. 否则作为十进制读取，遇 '.' 转移到浮点处理，遇 'e'/'E' 转移到科学记数法处理
    /// 性能考虑：科学记数法处理有 TODO（未实现）
    fn read_number(&mut self, first: char) -> Token {
        if first == '0' {
            let second = self.read_char();
            if second == 'x' || second == 'X' {
                return self.read_heximal();
            }
            self.putback_char();
        }
        // 十进制数
        let mut n = char::to_digit(first, 10).unwrap() as i64;
        loop {
            let ch = self.read_char();
            if let Some(d) = char::to_digit(ch, 10) {
                n = n * 10 + d as i64;
            } else if ch == '.' {
                return self.read_number_fraction(n);
            } else if ch == 'e' || ch == 'E' {
                return self.read_number_exponent(n);
            } else {
                self.putback_char();
                break;
            }
        }
        //check following
        Token::Integer(n)
    }

    /// 读取科学记数法形式的浮点数（如 1e10, 2.5e-3）
    /// i: 整数部分，尚未实现
    fn read_number_exponent(&mut self, i: i64) -> Token {
        todo!("lex number exponent")
    }

    /// 读取浮点数的小数部分
    /// i: 整数部分
    /// 持续读取小数位数字，返回组合后的 Float Token
    fn read_number_fraction(&mut self, i: i64) -> Token {
        let mut n: i64 = 0;       // 小数部分数字
        let mut x: f64 = 1.0;     // 小数位数（用于计算除数）
        loop {
            let ch = self.read_char();
            if let Some(d) = char::to_digit(ch, 10) {
                n = n * 10 + d as i64;
                x *= 10.0;
            } else {
                self.putback_char();
                break;
            }
        }
        // 整数部分 + (小数部分 / 小数位数) = 完整浮点数
        Token::Float(i as f64 + n as f64 / x)
    }

    /// 读取十六进制数（0x 或 0X 开头）
    /// 当前尚未实现
    fn read_heximal(&mut self) -> Token {
        todo!()
    }
}
