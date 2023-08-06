use thiserror::Error;

use crate::parser::declaratons::ProcedureDeclaration;
use crate::parser::procedure::{ParamList, ProcedureCall};
use crate::parser::types::Identifier;

use super::context::{Context, Scope, ScopeContext};
use super::expression::AnalyzedExpression;
use super::statement::AnalyzedBlock;
use super::traits::{Analyze, AnalyzeExpression};
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

#[derive(Debug)]
pub struct AnalyzedProcedureCall {
    pub identifier: String,
    pub arg_list: Vec<AnalyzedExpression>,
    pub ret_type: Type,
}

impl AnalyzeExpression<ProcedureCall> for AnalyzedProcedureCall {
    fn analyze_expression(
        value: ProcedureCall,
        context: &mut Context,
    ) -> Result<Self, SemanticsError> {
        let proc_sig = context
            .get_procedure_signature(&value.identifier.identifier_string)?
            .clone();
        let identifier = &value.identifier;

        let passed_args = value
            .arg_list
            .map_or(Vec::new(), |expression| expression.expr_list);

        if passed_args.len() != proc_sig.0.len() {
            return Err(SemanticsError::ParamCountMismatch(
                passed_args.len(),
                proc_sig.0.len(),
            ));
        }
        let args = passed_args
            .into_iter()
            .zip(proc_sig.0.into_iter())
            .map(move |(passed_arg, sig_arg)| {
                let expression = AnalyzedExpression::analyze_expression(passed_arg, context)?;
                let exp_type = expression.get_type(context)?;
                if exp_type != sig_arg.1 {
                    Err(SemanticsError::TypeMismatch(exp_type, sig_arg.1))
                } else {
                    Ok(expression)
                }
            })
            .collect::<Result<Vec<AnalyzedExpression>, SemanticsError>>()?;

        Ok(AnalyzedProcedureCall {
            identifier: identifier.identifier_string.clone(),
            arg_list: args,
            ret_type: proc_sig.1,
        })
    }

    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        Ok(self.ret_type.clone())
    }
}
