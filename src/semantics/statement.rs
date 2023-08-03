use crate::parser::statement::{Destination, Statement};

use super::expression::AnalyzedExpression;
use super::traits::{Analyze, AnalyzeExpression};
use super::value::Type;
use super::SemanticsError;

#[derive(Debug)]
pub struct AnalyzedBlock(pub Vec<AnalyzedStatement>);

#[derive(Debug)]
pub enum AnalyzedStatement {
    Assignment(AnalyzedAssignment),
    If(AnalyzedIf),
    Loop(AnalyzedLoop),
    Return(AnalyzedReturn),
}

impl Analyze<AnalyzedBlock> for Vec<Statement> {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedBlock, super::SemanticsError> {
        let statements = self
            .into_iter()
            .map(|statment| statment.analyze(context, scope))
            .collect::<Result<Vec<AnalyzedStatement>, SemanticsError>>()?;

        Ok(AnalyzedBlock(statements))
    }
}

impl Analyze<AnalyzedStatement> for Statement {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedStatement, super::SemanticsError> {
        let statement = match self {
            Statement::Assignment(statement) => todo!(),
            Statement::If(statement) => todo!(),
            Statement::Loop(_) => todo!(),
            Statement::Return(_) => todo!(),
        };
    }
}

#[derive(Debug)]
pub struct AnalyzedAssignment {
    pub destination: AnalyzedDestination,
    pub expression: AnalyzedExpression,
}

#[derive(Debug)]
pub struct AnalyzedDestination {
    pub identifier: String,
    pub expression: Option<AnalyzedExpression>,
    pub value_type: Type,
}

impl Analyze<AnalyzedDestination> for Destination {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedDestination, SemanticsError> {
        let value_type = context
            .get_variable_type(&self.identifier.identifier_string)?
            .clone();

        if let Some(curr_expr) = self.expression {
            if let Type::Array(arr_type, _) = value_type {
                let analyzed_expr = AnalyzedExpression::analyze_expression(curr_expr, context)?;
                let curr_type = analyzed_expr.get_type(context)?;
                if curr_type != Type::Int {
                    Err(SemanticsError::TypeMismatch(Type::Int, curr_type))
                } else {
                    Ok(AnalyzedDestination {
                        identifier: self.identifier.identifier_string,
                        expression: Some(analyzed_expr),
                        value_type: *arr_type,
                    })
                }
            } else {
                Err(SemanticsError::IndexOnNonArray(
                    self.identifier.identifier_string,
                ))
            }
        } else {
            Ok(AnalyzedDestination {
                identifier: self.identifier.identifier_string,
                expression: None,
                value_type,
            })
        }
    }
}

#[derive(Debug)]
pub struct AnalyzedIf {}

#[derive(Debug)]
pub struct AnalyzedLoop {}

#[derive(Debug)]
pub struct AnalyzedReturn {}
