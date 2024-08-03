use crate::luau::{LuauNode, LuauParam, LuauType};
use syn::{
    Expr, ExprReturn, FnArg, Item, ItemFn, Lit, Pat, ReturnType, Stmt, Type, UnOp,
};

pub fn transform_item_to_luau(item: &Item) -> LuauNode {
    match item {
        Item::Fn(item_fn) => transform_fn_to_luau(item_fn),

        _ => unimplemented!(),
    }
}

pub fn transform_fn_to_luau(item_fn: &ItemFn) -> LuauNode {
    let name = item_fn.sig.ident.to_string();
    let ret_type = match &item_fn.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(transform_return_type_to_luau(ty)),
    };

    LuauNode::Function {
        name,
        params: item_fn
            .sig
            .inputs
            .iter()
            .map(transform_param_to_luau)
            .collect(),
        ret_type,
        body: transform_block_to_luau(&item_fn.block.stmts)
    }
}

fn transform_block_to_luau(stmts: &[Stmt]) -> Vec<LuauNode> {
    let mut luau_nodes = vec![];

    for (i, stmt) in stmts.iter().enumerate() {
        let is_last = i == stmts.len() - 1;

        if let Stmt::Expr(expr, None) = stmt {
            if is_last {
                luau_nodes.push(LuauNode::Return {
                    value: Some(Box::new(transform_expr_to_luau(expr))),
                });
            } else {
                luau_nodes.push(transform_expr_to_luau(expr));
            }
        } else {
            luau_nodes.push(transform_stmt_to_luau(&stmt));
        }
    }

    luau_nodes
}

fn transform_param_to_luau(arg: &FnArg) -> LuauParam {
    match arg {
        FnArg::Typed(pat_type) => {
            let name = extract_pat_ident_name(&pat_type.pat);
            let typ = map_rust_type_to_luau(&pat_type.ty);

            LuauParam { name, typ }

        }

        _ => panic!("unsupported FnArg type: {:?}", arg),
    }
}

fn extract_pat_ident_name(pat: &Pat) -> String {
    match pat {
        Pat::Ident(ident) => ident.ident.to_string(),

        _ => panic!("unsupported Pat type: {:?}", pat),
    }
}

fn map_rust_type_to_luau(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();

            match type_name.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
                "bool" => "boolean".to_string(),
                "String" => "string".to_string(),

                _ => type_name,
            }
        }

        Type::Reference(type_ref) => {
            let inner = map_rust_type_to_luau(&type_ref.elem);

            if type_ref.mutability.is_some() {
                format!("{}_mutref", inner)
            } else {
                format!("{}_ref", inner)
            }
        }

        _ => panic!("unsupported rust type: {:?}", ty),
    }
}

fn transform_return_type_to_luau(ty: &Type) -> LuauType {
    LuauType {
        type_name: map_rust_type_to_luau(ty),
    }
}

fn transform_return_to_luau(ret_expr: &ExprReturn) -> LuauNode {
    let value = ret_expr
        .expr
        .as_ref()
        .map(|expr| Box::new(transform_expr_to_luau(expr)));

    LuauNode::Return { value }
}

fn transform_stmt_to_luau(stmt: &Stmt) -> LuauNode {
    match stmt {
        Stmt::Local(local) => {
            let name = extract_pat_ident_name(&local.pat);
            let expr = local.init.as_ref().map_or_else(
                || LuauNode::Value("".to_string()),
                |init| transform_expr_to_luau(&init.expr),
            );

            LuauNode::Let {
                name,
                expr: Box::new(expr),
            }
        }

        Stmt::Expr(expr, _semi) => transform_expr_to_luau(expr),
        Stmt::Item(item) => transform_item_to_luau(item),

        _ => panic!("unsupported statement type: {:?}", stmt),
    }
}

fn transform_expr_to_luau(expr: &Expr) -> LuauNode {
    match expr {
        Expr::Lit(expr_lit) => {
            if let Lit::Int(lit_int) = &expr_lit.lit {
                LuauNode::Value(lit_int.base10_digits().to_string())
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

            let args = expr_call.args.iter().map(transform_expr_to_luau).collect();

            LuauNode::Call {
                func: func_name,
                args,
            }
        }

        Expr::Return(ret_expr) => transform_return_to_luau(ret_expr),
        Expr::Binary(expr_binary) => {
            let left = Box::new(transform_expr_to_luau(&expr_binary.left));
            let right = Box::new(transform_expr_to_luau(&expr_binary.right));
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

            LuauNode::BinaryOp { op, left, right }
        }

        Expr::Path(expr_path) => {
            let path_str = expr_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            LuauNode::Value(path_str)
        }

        Expr::Reference(expr_ref) => {
            let name = if let Expr::Path(expr_path) = &*expr_ref.expr {
                expr_path.path.segments.last().unwrap().ident.to_string()
            } else {
                unimplemented!()
            };

            LuauNode::Ref {
                name,
                mutable: expr_ref.mutability.is_some()
            }
        }

        Expr::Unary(expr_unary) => {
            if let UnOp::Deref(_) = expr_unary.op {
                LuauNode::Deref {
                    expr: Box::new(transform_expr_to_luau(&expr_unary.expr)),
                }
            } else {
                unimplemented!()
            }
        }

        _ => panic!("unsupported expression type: {:?}", expr),
    }
}
