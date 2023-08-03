use crate::parser::statement::Statement;

use super::{traits::Analyze, value::Type, SemanticsError};

#[derive(Debug)]
pub struct AnalyzedBlock(pub Vec<AnalyzedStatement>);

#[derive(Debug)]
pub enum AnalyzedStatement {
    Assignment,
    If,
    Loop,
    Return,
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
            Statement::Assignment(node) => todo!(),
            Statement::If(_) => todo!(),
            Statement::Loop(_) => todo!(),
            Statement::Return(_) => todo!(),
        };
    }
}

#[derive(Debug)]
pub struct AnalyzedAssignment {
    pub destination: AnalyzedDestination,
    pub expression: (),
}

#[derive(Debug)]
pub struct AnalyzedDestination {
    pub identifier: String,
    pub expression: (),
    pub value_type: Type,
}

#[derive(Debug)]
pub struct AnalyzedIf {}

#[derive(Debug)]
pub struct AnalyzedLoop {}

#[derive(Debug)]
pub struct AnalyzedReturn {}
