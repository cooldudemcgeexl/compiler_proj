use crate::parser::statement::{
    AssignmentStatement, Destination, IfStatement, LoopStatement, ReturnStatement, Statement,
};

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
            Statement::Assignment(statement) => {
                AnalyzedStatement::Assignment(statement.analyze(context, scope)?)
            }
            Statement::If(statement) => AnalyzedStatement::If(statement.analyze(context, scope)?),
            Statement::Loop(statement) => {
                AnalyzedStatement::Loop(statement.analyze(context, scope)?)
            }
            Statement::Return(statement) => {
                AnalyzedStatement::Return(statement.analyze(context, scope)?)
            }
        };

        Ok(statement)
    }
}

#[derive(Debug)]
pub struct AnalyzedAssignment {
    pub destination: AnalyzedDestination,
    pub expression: AnalyzedExpression,
}
impl Analyze<AnalyzedAssignment> for AssignmentStatement {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedAssignment, SemanticsError> {
        let destination = self.destination.analyze(context, scope)?;
        let mut expression = AnalyzedExpression::analyze_expression(self.expression, context)?;
        let expression_type = expression.get_type(context)?;

        if &destination.value_type != &expression_type {
            expression = match (&destination.value_type, expression_type) {
                (Type::Int, Type::Bool | Type::Float) => expression.cast_expr(Type::Int),
                (Type::Bool, Type::Int) => expression.cast_expr(Type::Bool),
                (Type::Float, Type::Int) => expression.cast_expr(Type::Float),
                (dest_type, expr_type) => {
                    return Err(SemanticsError::TypeMismatch(dest_type.clone(), expr_type))
                }
            }
        }
        Ok(AnalyzedAssignment {
            destination,
            expression,
        })
    }
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
pub struct AnalyzedIf {
    pub conditional_expr: AnalyzedExpression,
    pub then_block: AnalyzedBlock,
    pub else_block: Option<AnalyzedBlock>,
}

impl Analyze<AnalyzedIf> for IfStatement {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedIf, SemanticsError> {
        let conditional_expr = AnalyzedExpression::analyze_expression(self.condition, context)?;
        let then_block = self.then_statement.analyze(context, scope)?;
        let else_block = self
            .else_statement
            .map(move |block| block.analyze(context, scope))
            .transpose()?;

        Ok(AnalyzedIf {
            conditional_expr,
            then_block,
            else_block,
        })
    }
}

#[derive(Debug)]
pub struct AnalyzedLoop {
    pub assignment: Box<AnalyzedAssignment>,
    pub condition: AnalyzedExpression,
    pub loop_body: AnalyzedBlock,
}

impl Analyze<AnalyzedLoop> for LoopStatement {
    fn analyze(
        self,
        context: &mut super::context::Context,
        scope: &super::context::Scope,
    ) -> Result<AnalyzedLoop, SemanticsError> {
        let assignment = self.assignment_statement.analyze(context, scope)?;
        let condition =
            AnalyzedExpression::analyze_expression(self.condition, context)?.cond_expr(context)?;

        let loop_body = self.loop_body.analyze(context, scope)?;

        Ok(AnalyzedLoop {
            assignment: Box::new(assignment),
            condition,
            loop_body,
        })
    }
}

#[derive(Debug)]
pub struct AnalyzedReturn {
    pub expression: AnalyzedExpression,
}

impl Analyze<AnalyzedReturn> for ReturnStatement {
    fn analyze(
        self,
        context: &mut super::context::Context,
        _scope: &super::context::Scope,
    ) -> Result<AnalyzedReturn, SemanticsError> {
        let exprected_ret_type = context.get_return_type().clone();
        if exprected_ret_type == Type::Void {
            return Err(SemanticsError::UnexpectedReturn);
        }

        let mut expression = AnalyzedExpression::analyze_expression(self.expression, context)?;
        let exp_type = expression.get_type(context)?;
        if exprected_ret_type != exp_type {
            expression = match (exprected_ret_type, exp_type) {
                (Type::Int, Type::Bool | Type::Float) => expression.cast_expr(Type::Int),
                (Type::Bool, Type::Int) => expression.cast_expr(Type::Bool),
                (Type::Float, Type::Int) => expression.cast_expr(Type::Float),
                (ret_type, exp_type) => {
                    return Err(SemanticsError::TypeMismatch(ret_type, exp_type))
                }
            }
        }
        Ok(AnalyzedReturn { expression })
    }
}
