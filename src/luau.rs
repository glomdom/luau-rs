#![allow(dead_code)]

#[derive(Debug)]
pub enum LuauNode {
    BinaryOp {
        op: String,
        left: Box<LuauNode>,
        right: Box<LuauNode>,
    },

    Block {
        statements: Vec<LuauNode>,
    },

    Call {
        func: String,
        args: Vec<LuauNode>,
    },

    Deref {
        expr: Box<LuauNode>,
    },

    Range {
        start: Option<Box<LuauNode>>,
        end: Option<Box<LuauNode>>,
    },

    ForIn {
        vars: Vec<String>,
        iter: Box<LuauNode>,
        body: Box<LuauNode>,
    },

    Function {
        name: String,
        params: Vec<LuauParam>,
        ret_type: Option<LuauType>,
        body: Box<LuauNode>,
    },

    If {
        condition: Box<LuauNode>,
        then_branch: Box<LuauNode>,
        else_branch: Option<Box<LuauNode>>,
    },

    Let {
        name: String,
        expr: Box<LuauNode>,
    },

    Ref {
        name: String,
        mutable: bool,
    },

    Return {
        value: Option<Box<LuauNode>>,
    },

    While {
        condition: Box<LuauNode>,
        body: Box<LuauNode>,
    },

    Value(String),
}

#[derive(Debug)]
pub struct LuauParam {
    pub name: String,
    pub typ: String,
    pub is_ref: bool,
}

#[derive(Debug)]
pub struct LuauField {
    pub name: String,
    pub typ: String,
}

#[derive(Debug)]
pub struct LuauType {
    pub type_name: String,
}
