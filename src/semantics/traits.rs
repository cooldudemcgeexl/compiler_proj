use super::context::{Context, Scope};
use super::value::Type;
use super::SemanticsError;

pub trait Analyze<T> {
    fn analyze(self, context: &mut Context, scope: &Scope) -> Result<T, SemanticsError>;
}

pub trait AnalyzeExpression<T>: Sized {
    fn analyze_expression(value: T, context: &mut Context) -> Result<Self, SemanticsError>;
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError>;
}
