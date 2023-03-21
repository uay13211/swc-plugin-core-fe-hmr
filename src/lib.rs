use swc_core::common::DUMMY_SP;
use swc_core::ecma::{
    ast::{
        CallExpr, Expr, ExprStmt, IfStmt, ImportDecl, ImportSpecifier, Module, ModuleItem, Program,
        Stmt,
    },
    utils::{member_expr, ExprFactory},
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::metadata::TransformPluginProgramMetadata;
use swc_core::plugin::plugin_transform;

pub struct TransformVisitor {
    should_decline_hmr: bool,
}

fn decline_webpack_hmr_node() -> Stmt {
    let if_stmt_test = member_expr!(DUMMY_SP, module.hot);
    let if_stmt_cons = Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: member_expr!(DUMMY_SP, module.hot.decline).as_callee(),
            args: vec![],
            type_args: None,
        })),
    });

    Stmt::If(IfStmt {
        span: DUMMY_SP,
        test: if_stmt_test,
        cons: Box::new(if_stmt_cons),
        alt: None,
    })
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        if self.should_decline_hmr {
            module
                .body
                .push(ModuleItem::Stmt(decline_webpack_hmr_node()));
            self.should_decline_hmr = false
        }
    }

    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        import_decl.visit_mut_children_with(self);
        if &import_decl.src.value == "core-fe" {
            for s in import_decl.specifiers.iter() {
                if let ImportSpecifier::Named(named_import) = s {
                    if &named_import.local.sym == "Module" {
                        self.should_decline_hmr = true;
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor {
        should_decline_hmr: false,
    }))
}

#[cfg(test)]
mod test;
