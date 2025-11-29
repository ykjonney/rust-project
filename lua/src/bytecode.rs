// Lua 字节码（ByteCode）定义
// 虚拟机执行的指令集，由解析器生成
// 包括变量操作（加载、存储）、常数加载、函数调用等指令

#[derive(Debug)]
pub enum ByteCode{
    /// 从全局表读取变量到寄存器
    GetGlobal(u8,u8),
    /// 将寄存器值写回全局表
    SetGlobal(u8,u8),
    /// 从常数池设置全局变量
    SetGlobalConst(u8,u8),
    /// 用另一个全局变量的值设置全局变量
    SetGlobalGlobal(u8,u8),
    /// 将常数加载到寄存器
    LoadConst(u8,u16),
    /// 将 nil 加载到寄存器
    LoadNil(u8),
    /// 将布尔值加载到寄存器
    LoadBool(u8,bool),
    /// 将小整数加载到寄存器
    LoadInt(u8,i16),
    /// 在寄存器间移动值
    Move(u8,u8),
    /// 函数调用：(参数寄存器, 返回值数)
    Call(u8,u8),
}