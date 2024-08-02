#![allow(dead_code)]

#[derive(Debug)]
pub enum IRNode {
    Function {
        name: String,
        params: Vec<IRParam>,
        ret_type: Option<IRType>,
        body: Vec<IRNode>,
    },

    Return {
        value: Option<Box<IRNode>>,
    },

    Let {
        name: String,
        expr: Box<IRNode>,
    },

    Call {
        func: String,
        args: Vec<IRNode>,
    },

    BinaryOp {
        op: String,
        left: Box<IRNode>,
        right: Box<IRNode>,
    },

    Value(String),
}

#[derive(Debug)]
pub struct IRParam {
    pub name: String,
    pub typ: String,
}

#[derive(Debug)]
pub struct IRField {
    pub name: String,
    pub typ: String,
}

#[derive(Debug)]
pub struct IRType {
    pub type_name: String,
}
