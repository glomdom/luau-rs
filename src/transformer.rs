use crate::luau::{LuauNode, LuauParam, LuauType};
use syn::{
    BinOp, Block, Expr, ExprIf, ExprReturn, File, FnArg, Item, ItemFn, Lit, Pat, ReturnType, Stmt, Type, UnOp
};

pub fn transform_file_to_luau(file: &File) -> LuauNode {
    let mut nodes = vec![];

    for item in &file.items {
        nodes.push(transform_item_to_luau(item));
    }

    LuauNode::Block { statements: nodes }
}

pub fn transform_item_to_luau(item: &Item) -> LuauNode {
    match item {
        Item::Fn(item_fn) => transform_fn_to_luau(item_fn),

        _ => unimplemented!(),
    }
}

fn transform_fn_to_luau(item_fn: &ItemFn) -> LuauNode {
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
        body: Box::new(transform_block_to_luau(&item_fn.block)),
    }
}

fn transform_block_to_luau(block: &Block) -> LuauNode {
    let stmts = &block.stmts;
    let mut luau_nodes = vec![];

    for stmt in stmts {
        match stmt {
            Stmt::Expr(expr, semi) => {
                if let Some(_) = semi {
                    luau_nodes.push(transform_expr_to_luau(expr));
                } else if should_be_returned(expr) {
                    luau_nodes.push(LuauNode::Return {
                        value: Some(Box::new(transform_expr_to_luau(expr))),
                    });
                } else {
                    luau_nodes.push(transform_expr_to_luau(expr));
                }
            }

            _ => luau_nodes.push(transform_stmt_to_luau(stmt)),
        }
    }

    LuauNode::Block {
        statements: luau_nodes,
    }
}

fn transform_param_to_luau(arg: &FnArg) -> LuauParam {
    match arg {
        FnArg::Typed(pat_type) => {
            let name = extract_pat_ident_name(&pat_type.pat);
            let typ = map_rust_type_to_luau(&pat_type.ty);
            let is_ref = match &*pat_type.ty {
                Type::Reference(_) => true,

                _ => false,
            };

            LuauParam { name, typ, is_ref }
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
                "i8" | "u8" | "i16" | "u16" | "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                    "number".to_string()
                }

                "bool" => "boolean".to_string(),
                "String" => "string".to_string(),

                _ => type_name,
            }
        }

        Type::Reference(type_ref) => {
            map_rust_type_to_luau(&type_ref.elem) // return the type name, but the resulting struct will have `is_ref: true``
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

fn transform_if_expr(expr_if: &ExprIf) -> LuauNode {
    let condition = transform_expr_to_luau(&expr_if.cond);
    let then_branch = transform_block_to_luau(&expr_if.then_branch);

    let else_branch = if let Some((_, else_expr)) = &expr_if.else_branch {
        match else_expr.as_ref() {
            Expr::If(expr_if) => Some(Box::new(transform_if_expr(expr_if))),
            Expr::Block(expr_block) => Some(Box::new(transform_block_to_luau(&expr_block.block))),

            _ => None,
        }
    } else {
        None
    };

    LuauNode::If {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch,
    }
}

fn transform_expr_to_luau(expr: &Expr) -> LuauNode {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(lit_int) => LuauNode::Value(lit_int.base10_digits().to_string()),
            Lit::Bool(lit_bool) => LuauNode::Value(lit_bool.value.to_string()),
            Lit::Str(lit_str) => LuauNode::Value(lit_str.value()),

            _ => unimplemented!(),
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
                BinOp::Add(_) => "+".to_string(),
                BinOp::Sub(_) => "-".to_string(),
                BinOp::Mul(_) => "*".to_string(),
                BinOp::Div(_) => "/".to_string(),
                BinOp::Rem(_) => "%".to_string(),
                BinOp::And(_) => "&&".to_string(),
                BinOp::Or(_) => "||".to_string(),
                BinOp::BitXor(_) => "^".to_string(),
                BinOp::BitAnd(_) => "&".to_string(),
                BinOp::BitOr(_) => "|".to_string(),
                BinOp::Shl(_) => "<<".to_string(),
                BinOp::Shr(_) => ">>".to_string(),
                BinOp::Eq(_) => "==".to_string(),
                BinOp::Lt(_) => "<".to_string(),
                BinOp::Le(_) => "<=".to_string(),
                BinOp::Ne(_) => "!=".to_string(),
                BinOp::Ge(_) => ">=".to_string(),
                BinOp::Gt(_) => ">".to_string(),

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
                mutable: expr_ref.mutability.is_some(),
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

        Expr::Block(expr_block) => transform_block_to_luau(&expr_block.block),
        Expr::If(expr_if) => transform_if_expr(expr_if),

        _ => panic!("unsupported expression type: {:?}", expr),
    }
}

fn should_be_returned(expr: &Expr) -> bool {
    match expr {
        Expr::If(_) | Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => false,

        _ => true,
    }
}