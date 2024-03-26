// parser generated by Antlr4Rust: "https://github.com/rrevenantt/antlr4rust"
#![allow(unused)]

mod mathlexer;
mod mathparser;
mod mathlistener;
mod mathvisitor;

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeListener, ParseTreeVisitorCompat, VisitChildren},
    InputStream,
};
use std::{collections::HashMap, f64::consts::E, process::Child};
use maplit::hashmap;
use once_cell::sync::Lazy;
use self::{
    mathlexer::*,
    mathparser::*,
    mathlistener::*,
    mathvisitor::*,
};
use crate::functions::*;
use crate::utilities::*;


pub struct FnStruct {
    definition: ChildFn,
}

impl FnStruct {
    pub fn apply(&self, args: &FnArgs) -> FnResult {
        self.definition.apply(args)
    } 
}

impl Default for FnStruct {
    fn default() -> Self {
        Self {
            definition: "x".to_child_fn()
        }
    }
}

fn string_to_fn(name: &str, mut args: Vec<ChildFn>) -> Option<ChildFn> {
    Some (
        match name {
            "sin" => args.pop().map(SinFn::new).to_child_fn(),
            "cos" => args.pop().map(CosFn::new).to_child_fn(),
            "tan" => args.pop().map(TanFn::new).to_child_fn(),
            // "log" => args.pop().map(LogFn::new).to_child_fn(),
            // "ln" => args.pop().map(LogFn::new).to_child_fn(),
            _ => return None
        }
    )
}


struct MathVisitor(ChildFn);

impl MathVisitor {
    pub fn new() -> Self {
        Self("x".to_child_fn())
    }
}


impl ParseTreeVisitorCompat<'_> for MathVisitor {
    type Node = mathParserContextType;
    type Return = ChildFn;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        todo!()
    }
}

impl mathVisitorCompat<'_> for MathVisitor {
    fn visit_prog(&mut self, ctx: &ProgContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_number(&mut self, ctx: &NumberContext<'_>) -> Self::Return {
        ChildFn::Const(
            ctx.NUMBER()
                .unwrap()
                .get_text()
                .parse()
                .unwrap()
        )
    }

    fn visit_pi(&mut self, ctx: &PiContext<'_>) -> Self::Return {
        ChildFn::Const(std::f64::consts::PI)
    }

    fn visit_e(&mut self, ctx: &EContext<'_>) -> Self::Return {
        ChildFn::Const(std::f64::consts::E)
    }

    fn visit_var(&mut self, ctx: &VarContext<'_>) -> Self::Return {
        ctx.ID()
            .unwrap()
            .get_text()
            .to_child_fn()
    }

    fn visit_parens(&mut self, ctx: &ParensContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_add(&mut self, ctx: &AddContext<'_>) -> Self::Return {
        let children: Vec<_> = ctx.expr_all()
            .into_iter()
            .map(|x| self.visit(&*x))
            .collect();

        AddFn::new(children).to_child_fn()
    }

    fn visit_multiply(&mut self, ctx: &MultiplyContext<'_>) -> Self::Return {
        let children: Vec<_> = ctx.expr_all()
            .into_iter()
            .map(|x| self.visit(&*x))
            .collect();

        MulFn::new(children).to_child_fn()
    }

    fn visit_power(&mut self, ctx: &PowerContext<'_>) -> Self::Return {
        let base = self.visit(&*ctx.expr(0).unwrap());
        let power = self.visit(&*ctx.expr(1).unwrap());
        ExpFn::new(base, power).to_child_fn()
    }

    fn visit_log(&mut self, ctx: &LogContext<'_>) -> Self::Return {
        let base = self.visit(&*ctx.expr(0).unwrap());
        let arg = self.visit(&*ctx.expr(1).unwrap());
        LogFn::new(base, arg).to_child_fn()
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'_>) -> Self::Return {
        let name = ctx.ID().unwrap().get_text();
        let args: Vec<_> = ctx.expr_all()
            .into_iter()
            .map(|x| self.visit(&*x))
            .collect();
        

        panic!("Unrecognized function name")
    }
}

// --> see listener and visitor on https://github.com/rrevenantt/antlr4rust/blob/master/tests/visitors_tests.rs

// #[should_panic]
#[test]
fn test_parser() {
    let lexer = mathLexer::new(InputStream::new("2^(3 - 1) * (1 - cos(pi/2)) + log_5(4 + ln(e))".into()));

    let token_source = CommonTokenStream::new(lexer);
    let mut parser = mathParser::new(token_source);

    let root = parser.prog().unwrap();

    let result = MathVisitor::new().visit(&*root);

    assert_eq!(result, 5.0);
}


