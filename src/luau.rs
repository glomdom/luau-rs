#![allow(dead_code)]

#[derive(Debug)]
pub enum LuauNode {
    Function {
        name: String,
        params: Vec<LuauParam>,
        ret_type: Option<LuauType>,
        body: Vec<LuauNode>,
    },

    Return {
        value: Option<Box<LuauNode>>,
    },

    Let {
        name: String,
        expr: Box<LuauNode>,
    },

    Call {
        func: String,
        args: Vec<LuauNode>,
    },

    BinaryOp {
        op: String,
        left: Box<LuauNode>,
        right: Box<LuauNode>,
    },

    Ref {
        name: String,
        mutable: bool,
    },

    Deref {
        expr: Box<LuauNode>,
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
