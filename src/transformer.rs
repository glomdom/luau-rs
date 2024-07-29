use crate::ir::{IRNode, IRParam, IRType};
use syn::{Expr, FnArg, Item, ItemFn, Lit, LocalInit, Pat, PatType, ReturnType, Stmt, Type};

pub fn transform_item_to_ir<'a>(item: &'a Item) -> IRNode<'a> {
    match item {
        Item::Fn(item_fn) => transform_fn_to_ir(item_fn),

        _ => unimplemented!(),
    }
}

pub fn transform_fn_to_ir<'a>(item_fn: &'a ItemFn) -> IRNode<'a> {
    let name = item_fn.sig.ident.to_string();
    let name_ref: &'a str = Box::leak(name.into_boxed_str());

    let ret_type = match &item_fn.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(transform_return_type_to_ir(ty)),
    };

    IRNode::Function {
        name: name_ref,
        params: item_fn
            .sig
            .inputs
            .iter()
            .map(transform_param_to_ir)
            .collect(),

        ret_type,
        body: item_fn
            .block
            .stmts
            .iter()
            .map(transform_stmt_to_ir)
            .collect(),
    }
}

fn transform_param_to_ir<'a>(arg: &'a FnArg) -> IRParam<'a> {
    match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => IRParam {
            name: Box::leak(extract_pat_ident_name(pat).into_boxed_str()),
            typ: Box::leak(extract_type_name(ty).into_boxed_str()),
        },

        _ => unimplemented!(),
    }
}

fn extract_pat_ident_name(pat: &Pat) -> String {
    if let Pat::Ident(ref ident) = pat {
        ident.ident.to_string()
    } else {
        unimplemented!()
    }
}

fn extract_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),

        _ => unimplemented!(),
    }
}

fn transform_return_type_to_ir<'a>(ty: &Type) -> IRType<'a> {
    IRType {
        type_name: Box::leak(extract_type_name(ty).into_boxed_str()),
    }
}

fn transform_stmt_to_ir<'a>(stmt: &'a Stmt) -> IRNode<'a> {
    match stmt {
        Stmt::Local(local) => {
            let pat_name = extract_pat_ident_name(&local.pat).into_boxed_str();
            let init_expr = if let Some(LocalInit { expr, .. }) = &local.init {
                transform_expr_to_ir(expr)
            } else {
                IRNode::Value("")
            };

            IRNode::Let {
                name: Box::leak(pat_name),
                expr: Box::new(init_expr),
            }
        }

        Stmt::Expr(expr, _) => transform_expr_to_ir(expr),

        _ => unimplemented!(),
    }
}

fn transform_expr_to_ir<'a>(expr: &'a Expr) -> IRNode<'a> {
    match expr {
        Expr::Lit(expr_lit) => {
            if let Lit::Int(lit_int) = &expr_lit.lit {
                IRNode::Value(Box::leak(
                    lit_int.base10_digits().to_string().into_boxed_str(),
                ))
            } else {
                unimplemented!()
            }
        }

        Expr::Call(expr_call) => {
            let func_name = if let Expr::Path(expr_path) = &*expr_call.func {
                expr_path.path.segments.last().unwrap().ident.to_string()
            } else {
                unimplemented!()
            };

            let args = expr_call
                .args
                .iter()
                .map(|arg| transform_expr_to_ir(arg))
                .collect();

            IRNode::Call {
                func: Box::leak(func_name.into_boxed_str()),
                args,
            }
        }

        _ => unimplemented!(),
    }
}
