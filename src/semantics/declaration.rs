use crate::parser::declaratons::Declaration;
use crate::parser::procedure::Parameter;

use super::context::{Context, Scope};
use super::procedure::AnalyzedProcedure;
use super::traits::Analyze;
use super::value::{NamedValue, Type};
use super::SemanticsError;

impl Analyze<Option<AnalyzedProcedure>> for Declaration {
    fn analyze(
        self,
        context: &mut Context,
        scope: &Scope,
    ) -> Result<Option<AnalyzedProcedure>, SemanticsError> {
        match self {
            Declaration::Variable(is_global, variable) => {
                if let Some(bound) = variable.array_bound {
                    let bound: usize = bound.number.try_into()?;
                    let value_type = Type::Array(Box::new(variable.type_mark.into()), bound);
                    context.set_type(
                        scope == &Scope::Global || is_global,
                        variable.identifier,
                        value_type,
                    )?;
                } else {
                    context.set_type(
                        scope == &Scope::Global || is_global,
                        variable.identifier,
                        variable.type_mark.into(),
                    )?;
                }
                Ok(None)
            }
            Declaration::Procedure(is_global, proceedure) => {
                let curr_scope = if is_global { &Scope::Global } else { scope };
                Ok(Some(proceedure.analyze(context, curr_scope)?))
            }
        }
    }
}

impl TryFrom<Parameter> for NamedValue {
    type Error = SemanticsError;

    fn try_from(value: Parameter) -> Result<Self, SemanticsError> {
        if let Some(bound) = value.variable_declaration.array_bound {
            Ok(NamedValue(
                value.variable_declaration.identifier,
                super::value::Type::Array(
                    Box::new(value.variable_declaration.type_mark.into()),
                    bound.number.try_into()?,
                ),
            ))
        } else {
            Ok(NamedValue(
                value.variable_declaration.identifier,
                value.variable_declaration.type_mark.into(),
            ))
        }
    }
}
