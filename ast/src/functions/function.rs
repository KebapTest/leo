use crate::{ast::Rule, common::Identifier, functions::FunctionInput, statements::Statement, types::Type, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::function_definition))]
pub struct Function<'ast> {
    pub function_name: Identifier<'ast>,
    pub parameters: Vec<FunctionInput<'ast>>,
    pub returns: Vec<Type<'ast>>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
