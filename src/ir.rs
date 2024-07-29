#![allow(dead_code)]

#[derive(Debug)]
pub enum IRNode<'a> {
    Function {
        name: &'a str,
        params: Vec<IRParam<'a>>,
        ret_type: Option<IRType<'a>>,
        body: Vec<IRNode<'a>>,
    },

    Return {
        value: Option<Box<IRNode<'a>>>,
    },

    Let {
        name: &'a str,
        expr: Box<IRNode<'a>>,
    },

    Call {
        func: &'a str,
        args: Vec<IRNode<'a>>,
    },

    BinaryOp {
        op: &'a str,
        left: Box<IRNode<'a>>,
        right: Box<IRNode<'a>>,
    },

    Value(&'a str),
}

#[derive(Debug)]
pub struct IRParam<'a> {
    pub name: &'a str,
    pub typ: &'a str,
}

#[derive(Debug)]
pub struct IRField<'a> {
    pub name: &'a str,
    pub typ: &'a str,
}

#[derive(Debug)]
pub struct IRType<'a> {
    pub type_name: &'a str,
}
