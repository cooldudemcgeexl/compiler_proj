use crate::parser::expression::Expression;

use super::context::Context;
use super::traits::AnalyzeExpression;
use super::value::Type;
use super::SemanticsError;

#[derive(Debug)]
pub enum AnalyzedExpression {
    BitwiseAnd(Box<AnalyzedExpression>, AnalyzedArithOp),
    BitwiseOr(Box<AnalyzedExpression>, AnalyzedArithOp),
    BitwiseNot(AnalyzedArithOp),
    LogicalAnd(Box<AnalyzedExpression>, AnalyzedArithOp),
    LogicalOr(Box<AnalyzedExpression>, AnalyzedArithOp),
    LogicalNot(AnalyzedArithOp),
    Cast(Box<AnalyzedExpression>, Type),
    BasicExp(AnalyzedArithOp),
}

impl AnalyzedExpression {
    pub fn cast_expr(self, value_type: Type) -> AnalyzedExpression {
        AnalyzedExpression::Cast(Box::new(self), value_type)
    }

    pub fn cond_expr(self, context: &Context) -> Result<AnalyzedExpression, SemanticsError> {
        todo!()
    }
}

impl AnalyzeExpression<Expression> for AnalyzedExpression {
    fn analyze_expression(
        value: Expression,
        context: &mut Context,
    ) -> Result<Self, SemanticsError> {
        todo!()
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum AnalyzedArithOp {}
