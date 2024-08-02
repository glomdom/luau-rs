use crate::ir::{IRNode, IRParam, IRType};
use syn::{
    Expr, ExprReturn, FnArg, Item, ItemFn, Lit, Pat, ReturnType, Stmt, Type,
};

pub fn transform_item_to_ir(item: &Item) -> IRNode {
    match item {
        Item::Fn(item_fn) => transform_fn_to_ir(item_fn),

        _ => unimplemented!(),
    }
}

pub fn transform_fn_to_ir(item_fn: &ItemFn) -> IRNode {
    let name = item_fn.sig.ident.to_string();
    let ret_type = match &item_fn.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(transform_return_type_to_ir(ty)),
    };

    IRNode::Function {
        name,
        params: item_fn
            .sig
            .inputs
            .iter()
            .map(transform_param_to_ir)
            .collect(),
        ret_type,
        body: transform_block_to_ir(&item_fn.block.stmts)
    }
}

fn transform_block_to_ir(stmts: &[Stmt]) -> Vec<IRNode> {
    let mut ir_nodes = vec![];

    for (i, stmt) in stmts.iter().enumerate() {
        let is_last = i == stmts.len() - 1;

        if let Stmt::Expr(expr, None) = stmt {
            if is_last {
                ir_nodes.push(IRNode::Return {
                    value: Some(Box::new(transform_expr_to_ir(expr))),
                });
            } else {
                ir_nodes.push(transform_expr_to_ir(expr));
            }
        } else {
            ir_nodes.push(transform_stmt_to_ir(&stmt));
        }
    }

    ir_nodes
}

fn transform_param_to_ir(arg: &FnArg) -> IRParam {
    match arg {
        FnArg::Typed(pat_type) => {
            let name = extract_pat_ident_name(&pat_type.pat);
            let typ = map_rust_type_to_luau(&pat_type.ty);

            IRParam { name, typ }
        }

        _ => unimplemented!(),
    }
}

fn extract_pat_ident_name(pat: &Pat) -> String {
    match pat {
        Pat::Ident(ident) => ident.ident.to_string(),

        _ => unimplemented!(),
    }
}

fn map_rust_type_to_luau(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();

            match type_name.as_str() {
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
                "bool" => "boolean".to_string(),
                "String" | "&str" => "string".to_string(),

                _ => type_name,
            }
        }

        _ => panic!(":( expected Type::Path but got {:?}", ty),
    }
}

fn transform_return_type_to_ir(ty: &Type) -> IRType {
    IRType {
        type_name: map_rust_type_to_luau(ty),
    }
}

fn transform_return_to_ir(ret_expr: &ExprReturn) -> IRNode {
    let value = ret_expr
        .expr
        .as_ref()
        .map(|expr| Box::new(transform_expr_to_ir(expr)));

    IRNode::Return { value }
}

fn transform_stmt_to_ir(stmt: &Stmt) -> IRNode {
    match stmt {
        Stmt::Local(local) => {
            let name = extract_pat_ident_name(&local.pat);
            let expr = local.init.as_ref().map_or_else(
                || IRNode::Value("".to_string()),
                |init| transform_expr_to_ir(&init.expr),
            );

            IRNode::Let {
                name,
                expr: Box::new(expr),
            }
        }

        Stmt::Expr(expr, _semi) => transform_expr_to_ir(expr),
        Stmt::Item(item) => transform_item_to_ir(item),

        _ => panic!("Unsupported statement type: {:?}", stmt),
    }
}

fn transform_expr_to_ir(expr: &Expr) -> IRNode {
    match expr {
        Expr::Lit(expr_lit) => {
            if let Lit::Int(lit_int) = &expr_lit.lit {
                IRNode::Value(lit_int.base10_digits().to_string())
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

            let args = expr_call.args.iter().map(transform_expr_to_ir).collect();

            IRNode::Call {
                func: func_name,
                args,
            }
        }

        Expr::Return(ret_expr) => transform_return_to_ir(ret_expr),
        Expr::Binary(expr_binary) => {
            let left = Box::new(transform_expr_to_ir(&expr_binary.left));
            let right = Box::new(transform_expr_to_ir(&expr_binary.right));
            let op = match &expr_binary.op {
                syn::BinOp::Add(_) => "+".to_string(),
                syn::BinOp::Sub(_) => "-".to_string(),
                syn::BinOp::Mul(_) => "*".to_string(),
                syn::BinOp::Div(_) => "/".to_string(),
                syn::BinOp::Rem(_) => "%".to_string(),
                syn::BinOp::And(_) => "&&".to_string(),
                syn::BinOp::Or(_) => "||".to_string(),
                syn::BinOp::BitXor(_) => "^".to_string(),
                syn::BinOp::BitAnd(_) => "&".to_string(),
                syn::BinOp::BitOr(_) => "|".to_string(),
                syn::BinOp::Shl(_) => "<<".to_string(),
                syn::BinOp::Shr(_) => ">>".to_string(),
                syn::BinOp::Eq(_) => "==".to_string(),
                syn::BinOp::Lt(_) => "<".to_string(),
                syn::BinOp::Le(_) => "<=".to_string(),
                syn::BinOp::Ne(_) => "!=".to_string(),
                syn::BinOp::Ge(_) => ">=".to_string(),
                syn::BinOp::Gt(_) => ">".to_string(),

                _ => panic!("Unsupported binary operator"),
            };

            IRNode::BinaryOp { op, left, right }
        }

        Expr::Path(expr_path) => {
            let path_str = expr_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            IRNode::Value(path_str)
        }

        _ => panic!("Unsupported expression type: {:?}", expr),
    }
}
