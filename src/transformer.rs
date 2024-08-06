use crate::{context::Context, luau::{Array, BinaryOp, Block, Call, Deref, For, Function, If, Let, LuauNode, LuauType, Param, Range, Return, Value}};
use syn::{
    BinOp, Expr, ExprForLoop, ExprIf, ExprReturn, File, FnArg, Item, ItemFn, Lit, Pat,
    ReturnType, Stmt, Type, UnOp,
};

pub fn transform_file_to_luau(file: &File) -> LuauNode {
    let mut context = Context::new();
    let mut nodes = vec![];

    for item in &file.items {
        nodes.push(transform_item_to_luau(item, &mut context));
    }

    LuauNode::Block(Block { statements: nodes })
}

pub fn transform_item_to_luau(item: &Item, context: &mut Context) -> LuauNode {
    match item {
        Item::Fn(item_fn) => transform_fn_to_luau(item_fn, context),
        _ => unimplemented!(),
    }
}

fn transform_fn_to_luau(item_fn: &ItemFn, context: &mut Context) -> LuauNode {
    let name = item_fn.sig.ident.to_string();
    let mut fn_context = context.clone();

    let params: Vec<Param> = item_fn.sig.inputs.iter().map(|arg| transform_param_to_luau(arg, &mut fn_context)).collect();
    let ret_type = match &item_fn.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(transform_type_to_luau(ty)),
    };

    LuauNode::Function(Function {
        name,
        params: params.clone(),
        ret_type,
        body: Box::new(transform_block_to_luau_with_params(&item_fn.block, &params, &mut fn_context)),
    })
}

fn transform_block_to_luau_with_params(block: &syn::Block, params: &[Param], context: &mut Context) -> LuauNode {
    let stmts = &block.stmts;
    let mut luau_nodes = vec![];

    for param in params {
        context.add_binding(&param.name, param.typ.clone());
    }

    for stmt in stmts {
        match stmt {
            Stmt::Expr(expr, semi) => {
                if let Some(_) = semi {
                    luau_nodes.push(transform_expr_to_luau_with_context(expr, context));
                } else if should_be_returned(expr) {
                    luau_nodes.push(LuauNode::Return(Return {
                        value: Some(Box::new(transform_expr_to_luau_with_context(expr, context))),
                    }));
                } else {
                    luau_nodes.push(transform_expr_to_luau_with_context(expr, context));
                }
            }

            _ => luau_nodes.push(transform_stmt_to_luau_with_context(stmt, context)),
        }
    }

    LuauNode::Block(Block {
        statements: luau_nodes,
    })
}

fn transform_stmt_to_luau_with_context(stmt: &Stmt, context: &mut Context) -> LuauNode {
    match stmt {
        Stmt::Local(local) => {
            let name = extract_pat_ident_name(&local.pat);
            let expr = local.init.as_ref().map_or_else(
                || LuauNode::Value(Value { value: "".to_string() }),
                |init| transform_expr_to_luau_with_context(&init.expr, context),
            );

            let node = LuauNode::Let(Let {
                name: name.clone(),
                expr: Box::new(expr),
            });

            // Add the local binding to the context
            context.add_binding(&name, LuauType { type_name: "unknown".to_string(), is_ref: false, is_mut: false });

            node
        }

        Stmt::Expr(expr, _semi) => transform_expr_to_luau_with_context(expr, context),
        Stmt::Item(item) => transform_item_to_luau(item, context),

        _ => panic!("unsupported statement type: {:?}", stmt),
    }
}

fn transform_expr_to_luau_with_context(expr: &Expr, context: &mut Context) -> LuauNode {
    match expr {
        Expr::Path(expr_path) => {
            let path_str = expr_path.path.segments.last().unwrap().ident.to_string();
            LuauNode::Value(Value { value: path_str })
        }

        _ => transform_expr_to_luau(expr, context),
    }
}

fn transform_expr_to_luau(expr: &Expr, context: &mut Context) -> LuauNode {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(lit_int) => LuauNode::Value(Value { value: lit_int.base10_digits().to_string() }),
            Lit::Bool(lit_bool) => LuauNode::Value(Value { value: lit_bool.value.to_string() }),
            Lit::Str(lit_str) => LuauNode::Value(Value { value: lit_str.value() }),

            _ => unimplemented!(),
        },

        Expr::Array(expr_array) => {
            let elements = expr_array.elems.iter().map(|e| transform_expr_to_luau(e, context)).collect();
            LuauNode::Array(Array { elements })
        }

        Expr::Call(expr_call) => {
            let func_name = if let Expr::Path(expr_path) = &*expr_call.func {
                expr_path.path.segments.last().unwrap().ident.to_string()
            } else {
                unimplemented!()
            };

            let args = expr_call.args.iter().map(|a| transform_expr_to_luau(a, context)).collect();

            LuauNode::Call(Call {
                func: func_name,
                args,
            })
        }

        Expr::Return(ret_expr) => transform_return_to_luau(ret_expr, context),
        Expr::Binary(expr_binary) => {
            let left = Box::new(transform_expr_to_luau(&expr_binary.left, context));
            let right = Box::new(transform_expr_to_luau(&expr_binary.right, context));
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

            LuauNode::BinaryOp(BinaryOp { op, left, right })
        }

        Expr::Path(expr_path) => {
            let path_str = expr_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            LuauNode::Value(Value { value: path_str })
        }

        Expr::Reference(expr_ref) => {
            let name = if let Expr::Path(expr_path) = &*expr_ref.expr {
                expr_path.path.segments.last().unwrap().ident.to_string()
            } else {
                unimplemented!()
            };

            LuauNode::Value(Value { value: name })
        }

        Expr::Unary(expr_unary) => {
            if let UnOp::Deref(_) = expr_unary.op {
                LuauNode::Deref(Deref {
                    expr: Box::new(transform_expr_to_luau(&expr_unary.expr, context)),
                })
            } else {
                unimplemented!()
            }
        }

        Expr::Block(expr_block) => transform_block_to_luau(&expr_block.block, context),
        Expr::If(expr_if) => transform_if_expr(expr_if, context),
        Expr::ForLoop(expr_for) => transform_for_loop(expr_for, context),
        Expr::Range(expr_range) => {
            let start = expr_range
                .start
                .as_ref()
                .map(|s| Box::new(transform_expr_to_luau(s, context)));
            let end = expr_range
                .end
                .as_ref()
                .map(|e| Box::new(transform_expr_to_luau(e, context)));

            LuauNode::Range(Range { start, end })
        }

        _ => panic!("unsupported expression type: {:?}", expr),
    }
}

fn transform_block_to_luau(block: &syn::Block, context: &mut Context) -> LuauNode {
    let stmts = &block.stmts;
    let mut luau_nodes = vec![];

    for stmt in stmts {
        match stmt {
            Stmt::Expr(expr, semi) => {
                if let Some(_) = semi {
                    luau_nodes.push(transform_expr_to_luau(expr, context));
                } else if should_be_returned(expr) {
                    luau_nodes.push(LuauNode::Return(Return {
                        value: Some(Box::new(transform_expr_to_luau(expr, context))),
                    }));
                } else {
                    luau_nodes.push(transform_expr_to_luau(expr, context));
                }
            }

            _ => luau_nodes.push(transform_stmt_to_luau(stmt, context)),
        }
    }

    LuauNode::Block(Block {
        statements: luau_nodes,
    })
}

fn transform_stmt_to_luau(stmt: &Stmt, context: &mut Context) -> LuauNode {
    match stmt {
        Stmt::Local(local) => {
            let name = extract_pat_ident_name(&local.pat);
            let expr = local.init.as_ref().map_or_else(
                || LuauNode::Value(Value { value: "".to_string() }),
                |init| transform_expr_to_luau(&init.expr, context),
            );

            let node = LuauNode::Let(Let {
                name: name.clone(),
                expr: Box::new(expr),
            });

            // Add the local binding to the context
            context.add_binding(&name, LuauType { type_name: "unknown".to_string(), is_ref: false, is_mut: false });

            node
        }

        Stmt::Expr(expr, _semi) => transform_expr_to_luau(expr, context),
        Stmt::Item(item) => transform_item_to_luau(item, context),

        _ => panic!("unsupported statement type: {:?}", stmt),
    }
}

fn transform_param_to_luau(arg: &FnArg, context: &mut Context) -> Param {
    match arg {
        FnArg::Typed(pat_type) => {
            let name = extract_pat_ident_name(&pat_type.pat);
            let typ = transform_type_to_luau(&pat_type.ty);
            context.add_binding(&name, typ.clone());
            Param { name, typ }
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

fn transform_type_to_luau(ty: &Type) -> LuauType {
    let is_ref = matches!(ty, Type::Reference(_));
    let type_name = map_rust_type_to_luau(ty);
    LuauType {
        type_name,
        is_ref,
        is_mut: false,
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

        Type::Reference(type_ref) => map_rust_type_to_luau(&type_ref.elem),

        Type::Array(type_array) => format!("{}[]", map_rust_type_to_luau(&type_array.elem)),

        _ => panic!("unsupported rust type: {:?}", ty),
    }
}

fn transform_return_to_luau(ret_expr: &ExprReturn, context: &mut Context) -> LuauNode {
    let value = ret_expr
        .expr
        .as_ref()
        .map(|expr| Box::new(transform_expr_to_luau(expr, context)));

    LuauNode::Return(Return { value })
}

fn transform_if_expr(expr_if: &ExprIf, context: &mut Context) -> LuauNode {
    let condition = transform_expr_to_luau(&expr_if.cond, context);
    let then_branch = transform_block_to_luau(&expr_if.then_branch, context);

    let else_branch = if let Some((_, else_expr)) = &expr_if.else_branch {
        match else_expr.as_ref() {
            Expr::If(expr_if) => Some(Box::new(transform_if_expr(expr_if, context))),
            Expr::Block(expr_block) => Some(Box::new(transform_block_to_luau(&expr_block.block, context))),

            _ => None,
        }
    } else {
        None
    };

    LuauNode::If(If {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch,
    })
}

fn transform_for_loop(expr_for: &ExprForLoop, context: &mut Context) -> LuauNode {
    let vars = vec![extract_pat_ident_name(&expr_for.pat)];
    let iter = Box::new(transform_expr_to_luau(&expr_for.expr, context));
    let mut loop_context = context.clone();
    let body = Box::new(transform_block_to_luau(&expr_for.body, &mut loop_context));

    // Add the loop variables to the context
    for var in &vars {
        loop_context.add_binding(var, LuauType { type_name: "unknown".to_string(), is_ref: false, is_mut: false });
    }

    LuauNode::For(For { vars, iter, body })
}

fn should_be_returned(expr: &Expr) -> bool {
    match expr {
        Expr::If(_) | Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => false,

        _ => true,
    }
}