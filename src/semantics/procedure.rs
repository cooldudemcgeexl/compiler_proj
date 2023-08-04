use thiserror::Error;

use crate::parser::declaratons::ProcedureDeclaration;
use crate::parser::procedure::ParamList;

use super::context::{Context, Scope, ScopeContext};
use super::statement::AnalyzedBlock;
use super::traits::Analyze;
use super::value::{NamedValue, ProcedureSignature, Type};
use super::SemanticsError;

#[derive(Debug)]
pub struct AnalyzedProcedure {
    pub identifier: String,
    pub arg_list: Vec<NamedValue>,
    pub declarations: ScopeContext,
    pub procedures: Vec<Box<AnalyzedProcedure>>,
    pub block: AnalyzedBlock,
}

impl Analyze<AnalyzedProcedure> for ProcedureDeclaration {
    fn analyze(
        self,
        context: &mut Context,
        scope: &Scope,
    ) -> Result<AnalyzedProcedure, SemanticsError> {
        let arg_list = match self.procedure_header.param_list {
            Some(ParamList { param_list }) => param_list
                .into_iter()
                .map(|param| param.try_into())
                .collect::<Result<Vec<NamedValue>, SemanticsError>>()?,
            None => Vec::new(),
        };
        let identifier = self.procedure_header.identifier;
        let return_type: Type = self.procedure_header.type_mark.into();
        let signature = ProcedureSignature(arg_list.clone(), return_type.clone());
        context.set_procedure(scope == &Scope::Global, identifier.clone(), signature)?;

        context.start_stack(return_type);

        for arg in arg_list.iter() {
            context.set_type(false, arg.0.clone(), arg.1.clone())?;
        }

        let mut procedures = Vec::new();
        for declaration in self.procedure_body.declarations {
            if let Some(procedure) = declaration.analyze(context, &Scope::Local)? {
                procedures.push(Box::new(procedure))
            }
        }

        let block = self
            .procedure_body
            .statements
            .analyze(context, &Scope::Local)?;

        Ok(AnalyzedProcedure {
            identifier,
            arg_list,
            declarations: context.end_stack()?,
            procedures,
            block,
        })
    }
}
