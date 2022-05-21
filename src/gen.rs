use crate::ast::{Expr, Lit, Stat};
use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    fn gen_lit(&self, lit: Lit) -> Option<BasicValueEnum<'ctx>> {
        match lit {
            Lit::Int(int) => {
                let i32_type = self.context.i32_type();
                Some(BasicValueEnum::IntValue(
                    i32_type.const_int(int.parse().unwrap(), true),
                ))
            }
        }
    }
    fn gen_expr(&self, expr: Expr) -> Option<BasicValueEnum<'ctx>> {
        match expr {
            Expr::Lit(lit) => self.gen_lit(lit),
            Expr::If(_, _, _) => todo!(),
            Expr::While(_, _) => todo!(),
            Expr::Block(mut _stats) => todo!(),
        }
    }
    fn gen_stat(&self, stat: Stat) {
        match stat {
            Stat::Var(_name, _init) => todo!(),
        }
    }
}

pub fn gen_ir(stat: Stat) -> String {
    let context = Context::create();
    let codegen = CodeGenerator {
        context: &context,
        module: context.create_module("main"),
        builder: context.create_builder(),
    };
    codegen.gen_stat(stat);

    codegen.module.print_to_string().to_string()
}
