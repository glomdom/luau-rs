#![allow(dead_code)]

#[derive(Debug)]
pub struct Array {
    pub elements: Vec<LuauNode>,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op: String,
    pub left: Box<LuauNode>,
    pub right: Box<LuauNode>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<LuauNode>,
}

#[derive(Debug)]
pub struct Call {
    pub func: String,
    pub args: Vec<LuauNode>,
}

#[derive(Debug)]
pub struct Deref {
    pub expr: Box<LuauNode>,
}

#[derive(Debug)]
pub struct Range {
    pub start: Option<Box<LuauNode>>,
    pub end: Option<Box<LuauNode>>,
}

#[derive(Debug)]
pub struct For {
    pub vars: Vec<String>,
    pub iter: Box<LuauNode>,
    pub body: Box<LuauNode>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<LuauType>,
    pub body: Box<LuauNode>,
}

#[derive(Debug)]
pub struct If {
    pub condition: Box<LuauNode>,
    pub then_branch: Box<LuauNode>,
    pub else_branch: Option<Box<LuauNode>>,
}

#[derive(Debug)]
pub struct Let {
    pub name: String,
    pub expr: Box<LuauNode>,
}

#[derive(Debug)]
pub struct Ref {
    pub name: String,
    pub mutable: bool,
}

#[derive(Debug)]
pub struct Return {
    pub value: Option<Box<LuauNode>>,
}

#[derive(Debug)]
pub struct While {
    pub condition: Box<LuauNode>,
    pub body: Box<LuauNode>,
}

#[derive(Debug)]
pub struct Value {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub typ: LuauType,
}

/// Enum encapsulating every Luau node.
#[derive(Debug)]
pub enum LuauNode {
    Array(Array),
    BinaryOp(BinaryOp),
    Block(Block),
    Call(Call),
    Deref(Deref),
    Range(Range),
    For(For),
    Function(Function),
    If(If),
    Let(Let),
    Ref(Ref),
    Return(Return),
    While(While),
    Param(Param),
    Value(Value),
}

#[derive(Debug)]
pub struct LuauField {
    pub name: String,
    pub typ: String,
}

#[derive(Debug, Clone)]
pub struct LuauType {
    pub type_name: String,
    pub is_mut: bool,
    pub is_ref: bool,
}
