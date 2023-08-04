use crate::parser::expression::{ArtihOp, Expression, Factor, Name, Relation, Term};
use crate::parser::types::Number;

use super::context::Context;
use super::procedure::AnalyzedProcedureCall;
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
    ArithOp(AnalyzedArithOp),
}

impl AnalyzedExpression {
    pub fn cast_expr(self, value_type: Type) -> AnalyzedExpression {
        AnalyzedExpression::Cast(Box::new(self), value_type)
    }

    pub fn cond_expr(self, context: &Context) -> Result<AnalyzedExpression, SemanticsError> {
        match self.get_type(context)? {
            Type::Bool => Ok(self),
            Type::Int => Ok(self.cast_expr(Type::Bool)),
            value_type => Err(SemanticsError::InvalidType(
                String::from("Bool,Int"),
                value_type,
            )),
        }
    }
}

impl AnalyzeExpression<Expression> for AnalyzedExpression {
    fn analyze_expression(
        value: Expression,
        context: &mut Context,
    ) -> Result<Self, SemanticsError> {
        match value {
            Expression::BasicExp(expression) => Ok(AnalyzedExpression::ArithOp(
                AnalyzedArithOp::analyze_expression(expression, context)?,
            )),
            Expression::NotExp(expression) => {
                let arith_op = AnalyzedArithOp::analyze_expression(expression, context)?;
                match arith_op.get_type(context)? {
                    Type::Bool => Ok(AnalyzedExpression::LogicalNot(arith_op)),
                    Type::Int => Ok(AnalyzedExpression::BitwiseNot(arith_op)),
                    value_type => Err(SemanticsError::InvalidType(
                        String::from("Bool,Int"),
                        value_type,
                    )),
                }
            }
            Expression::AndExp(box expression, arith_op) => {
                let arith_op = AnalyzedArithOp::analyze_expression(arith_op, context)?;
                let expression = AnalyzedExpression::analyze_expression(expression, context)?;

                match (arith_op.get_type(context)?, expression.get_type(context)?) {
                    (Type::Int, Type::Int) => Ok(AnalyzedExpression::BitwiseAnd(
                        Box::new(expression),
                        arith_op,
                    )),
                    (Type::Bool, Type::Bool) => Ok(AnalyzedExpression::LogicalAnd(
                        Box::new(expression),
                        arith_op,
                    )),
                    (arith_type, exp_type) => {
                        Err(SemanticsError::TypeMismatch(arith_type, exp_type))
                    }
                }
            }
            Expression::OrExp(box expression, arith_op) => {
                let arith_op = AnalyzedArithOp::analyze_expression(arith_op, context)?;
                let expression = AnalyzedExpression::analyze_expression(expression, context)?;

                match (arith_op.get_type(context)?, expression.get_type(context)?) {
                    (Type::Int, Type::Int) => Ok(AnalyzedExpression::BitwiseOr(
                        Box::new(expression),
                        arith_op,
                    )),
                    (Type::Bool, Type::Bool) => Ok(AnalyzedExpression::LogicalOr(
                        Box::new(expression),
                        arith_op,
                    )),
                    (arith_type, exp_type) => {
                        Err(SemanticsError::TypeMismatch(arith_type, exp_type))
                    }
                }
            }
        }
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedExpression::BitwiseAnd(_, _)
            | AnalyzedExpression::BitwiseOr(_, _)
            | AnalyzedExpression::BitwiseNot(_) => Ok(Type::Int),
            AnalyzedExpression::LogicalAnd(_, _)
            | AnalyzedExpression::LogicalOr(_, _)
            | AnalyzedExpression::LogicalNot(_) => Ok(Type::Bool),

            AnalyzedExpression::Cast(_, cast_type) => Ok(cast_type.clone()),
            AnalyzedExpression::ArithOp(arith_op) => arith_op.get_type(context),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedArithOp {
    Plus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ArrayScalarPlus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ScalarArrayPlus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ArrayPlus(Box<AnalyzedArithOp>, AnalyzedRelation),

    Minus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ArrayScalarMinus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ScalarArrayMinus(Box<AnalyzedArithOp>, AnalyzedRelation),
    ArrayMinus(Box<AnalyzedArithOp>, AnalyzedRelation),

    Cast(Box<AnalyzedArithOp>, Type),
    Relation(AnalyzedRelation),
}

impl AnalyzedArithOp {
    pub fn cast(self, value_type: Type) -> AnalyzedArithOp {
        AnalyzedArithOp::Cast(Box::new(self), value_type)
    }
}

impl AnalyzeExpression<ArtihOp> for AnalyzedArithOp {
    fn analyze_expression(value: ArtihOp, context: &mut Context) -> Result<Self, SemanticsError> {
        match value {
            ArtihOp::AddOp(box arith_op, relation) => {
                let arith_op = AnalyzedArithOp::analyze_expression(arith_op, context)?;
                let relation = AnalyzedRelation::analyze_expression(relation, context)?;

                let arith_type = arith_op.get_type(context)?;
                let relation_type = relation.get_type(context)?;
                match (arith_type, relation_type) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(AnalyzedArithOp::Plus(Box::new(arith_op), relation))
                    }
                    (Type::Array(box Type::Int, l_bound), Type::Array(box Type::Int, r_bound))
                    | (
                        Type::Array(box Type::Float, l_bound),
                        Type::Array(box Type::Float, r_bound),
                    ) if l_bound == r_bound => {
                        Ok(AnalyzedArithOp::ArrayPlus(Box::new(arith_op), relation))
                    }

                    (Type::Int, Type::Float) => Ok(AnalyzedArithOp::Plus(
                        Box::new(arith_op.cast(Type::Float)),
                        relation,
                    )),
                    (Type::Float, Type::Int) => Ok(AnalyzedArithOp::Plus(
                        Box::new(arith_op),
                        relation.cast(Type::Float),
                    )),

                    (Type::Array(box Type::Int, _), Type::Int) => Ok(
                        AnalyzedArithOp::ArrayScalarPlus(Box::new(arith_op), relation),
                    ),
                    (Type::Array(box Type::Int, _), Type::Float) => {
                        Ok(AnalyzedArithOp::ArrayScalarPlus(
                            Box::new(arith_op),
                            relation.cast(Type::Int),
                        ))
                    }
                    (Type::Array(box Type::Float, _), Type::Int) => {
                        Ok(AnalyzedArithOp::ArrayScalarPlus(
                            Box::new(arith_op),
                            relation.cast(Type::Float),
                        ))
                    }
                    (Type::Array(box Type::Float, _), Type::Float) => Ok(
                        AnalyzedArithOp::ArrayScalarPlus(Box::new(arith_op), relation),
                    ),

                    (Type::Int, Type::Array(box Type::Int, _)) => Ok(
                        AnalyzedArithOp::ScalarArrayPlus(Box::new(arith_op), relation),
                    ),
                    (Type::Float, Type::Array(box Type::Int, _)) => {
                        Ok(AnalyzedArithOp::ScalarArrayPlus(
                            Box::new(arith_op.cast(Type::Int)),
                            relation,
                        ))
                    }
                    (Type::Int, Type::Array(box Type::Float, _)) => {
                        Ok(AnalyzedArithOp::ScalarArrayPlus(
                            Box::new(arith_op.cast(Type::Float)),
                            relation,
                        ))
                    }
                    (Type::Float, Type::Array(box Type::Float, _)) => Ok(
                        AnalyzedArithOp::ScalarArrayPlus(Box::new(arith_op), relation),
                    ),
                    (l_type, r_type) => Err(SemanticsError::TypeMismatch(l_type, r_type)),
                }
            }
            ArtihOp::SubOp(box arith_op, relation) => {
                let arith_op = AnalyzedArithOp::analyze_expression(arith_op, context)?;
                let relation = AnalyzedRelation::analyze_expression(relation, context)?;

                let arith_type = arith_op.get_type(context)?;
                let relation_type = relation.get_type(context)?;
                match (arith_type, relation_type) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(AnalyzedArithOp::Minus(Box::new(arith_op), relation))
                    }
                    (Type::Array(box Type::Int, l_bound), Type::Array(box Type::Int, r_bound))
                    | (
                        Type::Array(box Type::Float, l_bound),
                        Type::Array(box Type::Float, r_bound),
                    ) if l_bound == r_bound => {
                        Ok(AnalyzedArithOp::ArrayMinus(Box::new(arith_op), relation))
                    }

                    (Type::Int, Type::Float) => Ok(AnalyzedArithOp::Minus(
                        Box::new(arith_op.cast(Type::Float)),
                        relation,
                    )),
                    (Type::Float, Type::Int) => Ok(AnalyzedArithOp::Minus(
                        Box::new(arith_op),
                        relation.cast(Type::Float),
                    )),

                    (Type::Array(box Type::Int, _), Type::Int) => Ok(
                        AnalyzedArithOp::ArrayScalarMinus(Box::new(arith_op), relation),
                    ),
                    (Type::Array(box Type::Int, _), Type::Float) => {
                        Ok(AnalyzedArithOp::ArrayScalarMinus(
                            Box::new(arith_op),
                            relation.cast(Type::Int),
                        ))
                    }
                    (Type::Array(box Type::Float, _), Type::Int) => {
                        Ok(AnalyzedArithOp::ArrayScalarMinus(
                            Box::new(arith_op),
                            relation.cast(Type::Float),
                        ))
                    }
                    (Type::Array(box Type::Float, _), Type::Float) => Ok(
                        AnalyzedArithOp::ArrayScalarMinus(Box::new(arith_op), relation),
                    ),

                    (Type::Int, Type::Array(box Type::Int, _)) => Ok(
                        AnalyzedArithOp::ScalarArrayMinus(Box::new(arith_op), relation),
                    ),
                    (Type::Float, Type::Array(box Type::Int, _)) => {
                        Ok(AnalyzedArithOp::ScalarArrayMinus(
                            Box::new(arith_op.cast(Type::Int)),
                            relation,
                        ))
                    }
                    (Type::Int, Type::Array(box Type::Float, _)) => {
                        Ok(AnalyzedArithOp::ScalarArrayMinus(
                            Box::new(arith_op.cast(Type::Float)),
                            relation,
                        ))
                    }
                    (Type::Float, Type::Array(box Type::Float, _)) => Ok(
                        AnalyzedArithOp::ScalarArrayMinus(Box::new(arith_op), relation),
                    ),
                    (l_type, r_type) => Err(SemanticsError::TypeMismatch(l_type, r_type)),
                }
            }
            ArtihOp::Relation(relation) => Ok(AnalyzedArithOp::Relation(
                AnalyzedRelation::analyze_expression(relation, context)?,
            )),
        }
    }

    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedArithOp::Plus(_, relaton) => relaton.get_type(context),
            AnalyzedArithOp::ArrayScalarPlus(arith_op, _) => arith_op.get_type(context),
            AnalyzedArithOp::ScalarArrayPlus(_, relation) => relation.get_type(context),
            AnalyzedArithOp::ArrayPlus(_, relation) => relation.get_type(context),
            AnalyzedArithOp::Minus(_, relation) => relation.get_type(context),
            AnalyzedArithOp::ArrayScalarMinus(arith_op, _) => arith_op.get_type(context),
            AnalyzedArithOp::ScalarArrayMinus(_, relation) => relation.get_type(context),
            AnalyzedArithOp::ArrayMinus(_, relation) => relation.get_type(context),
            AnalyzedArithOp::Cast(_, value_type) => Ok(value_type.clone()),
            AnalyzedArithOp::Relation(relation) => relation.get_type(context),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedRelation {
    LessThan(Box<AnalyzedRelation>, AnalyzedTerm),
    LessThanEq(Box<AnalyzedRelation>, AnalyzedTerm),
    GreaterThan(Box<AnalyzedRelation>, AnalyzedTerm),
    GreaterThanEq(Box<AnalyzedRelation>, AnalyzedTerm),
    Equals(Box<AnalyzedRelation>, AnalyzedTerm),
    NotEquals(Box<AnalyzedRelation>, AnalyzedTerm),
    Cast(Box<AnalyzedRelation>, Type),
    Term(AnalyzedTerm),
}

impl AnalyzedRelation {
    pub fn cast(self, value_type: Type) -> AnalyzedRelation {
        AnalyzedRelation::Cast(Box::new(self), value_type)
    }

    pub fn try_compatible(
        relation: Relation,
        term: Term,
        equality: bool,
        context: &mut Context,
    ) -> Result<(AnalyzedRelation, AnalyzedTerm), SemanticsError> {
        let mut relation = AnalyzedRelation::analyze_expression(relation, context)?;
        let mut term = AnalyzedTerm::analyze_expression(term, context)?;

        relation = match relation.get_type(context)? {
            Type::Int | Type::Float => relation,
            Type::Bool => relation.cast(Type::Int),
            Type::String if equality => relation,
            value_type => {
                return Err(SemanticsError::InvalidType(
                    String::from("Relation"),
                    value_type,
                ))
            }
        };

        term = match term.get_type(context)? {
            Type::Int | Type::Float => term,
            Type::Bool => term.cast(Type::Int),
            Type::String if equality => term,
            value_type => {
                return Err(SemanticsError::InvalidType(
                    String::from("Relation"),
                    value_type,
                ))
            }
        };

        let relation_type = relation.get_type(context)?;
        let term_type = term.get_type(context)?;

        if relation_type == term_type {
            Ok((relation, term))
        } else {
            match (relation_type, term_type) {
                (Type::Int, Type::Float) => Ok((relation.cast(Type::Float), term)),
                (Type::Float, Type::Int) => Ok((relation, term.cast(Type::Float))),
                (l_type, r_type) => Err(SemanticsError::TypeMismatch(l_type, r_type)),
            }
        }
    }
}

impl AnalyzeExpression<Relation> for AnalyzedRelation {
    fn analyze_expression(value: Relation, context: &mut Context) -> Result<Self, SemanticsError> {
        match value {
            Relation::Term(term) => Ok(AnalyzedRelation::Term(AnalyzedTerm::analyze_expression(
                term, context,
            )?)),
            Relation::LessThan(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, false, context)?;
                Ok(AnalyzedRelation::LessThan(Box::new(relation), term))
            }
            Relation::LessThanEq(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, false, context)?;
                Ok(AnalyzedRelation::LessThanEq(Box::new(relation), term))
            }
            Relation::GreaterThan(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, false, context)?;
                Ok(AnalyzedRelation::GreaterThan(Box::new(relation), term))
            }
            Relation::GreaterThanEq(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, false, context)?;
                Ok(AnalyzedRelation::GreaterThanEq(Box::new(relation), term))
            }
            Relation::Equals(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, true, context)?;
                Ok(AnalyzedRelation::Equals(Box::new(relation), term))
            }
            Relation::NotEquals(box relation, term) => {
                let (relation, term) =
                    AnalyzedRelation::try_compatible(relation, term, true, context)?;
                Ok(AnalyzedRelation::NotEquals(Box::new(relation), term))
            }
        }
    }

    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedRelation::Term(term) => term.get_type(context),
            AnalyzedRelation::Cast(_, value_type) => Ok(value_type.clone()),
            _ => Ok(Type::Bool),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedTerm {
    Multiply(Box<AnalyzedTerm>, AnalyzedFactor),
    ArrayScalarMultiply(Box<AnalyzedTerm>, AnalyzedFactor),
    ScalarArrayMultiply(Box<AnalyzedTerm>, AnalyzedFactor),
    ArrayMultiply(Box<AnalyzedTerm>, AnalyzedFactor),

    Divide(Box<AnalyzedTerm>, AnalyzedFactor),
    ArrayScalarDivide(Box<AnalyzedTerm>, AnalyzedFactor),
    ScalarArrayDivide(Box<AnalyzedTerm>, AnalyzedFactor),
    ArrayDivide(Box<AnalyzedTerm>, AnalyzedFactor),

    Cast(Box<AnalyzedTerm>, Type),
    Factor(AnalyzedFactor),
}

impl AnalyzedTerm {
    pub fn cast(self, value_type: Type) -> AnalyzedTerm {
        AnalyzedTerm::Cast(Box::new(self), value_type)
    }
}

impl AnalyzeExpression<Term> for AnalyzedTerm {
    fn analyze_expression(value: Term, context: &mut Context) -> Result<Self, SemanticsError> {
        match value {
            Term::Factor(factor) => Ok(AnalyzedTerm::Factor(AnalyzedFactor::analyze_expression(
                factor, context,
            )?)),

            Term::MultTerm(term, factor) => {
                let term = AnalyzedTerm::analyze_expression(*term, context)?;
                let factor = AnalyzedFactor::analyze_expression(factor, context)?;

                let term_type = term.get_type(context)?;
                let factor_type = term.get_type(context)?;

                match (term_type, factor_type) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(AnalyzedTerm::Multiply(Box::new(term), factor))
                    }
                    (Type::Array(box Type::Int, l_bound), Type::Array(box Type::Int, r_bound))
                    | (
                        Type::Array(box Type::Float, l_bound),
                        Type::Array(box Type::Float, r_bound),
                    ) if l_bound == r_bound => {
                        Ok(AnalyzedTerm::ArrayMultiply(Box::new(term), factor))
                    }

                    (Type::Int, Type::Float) => Ok(AnalyzedTerm::Multiply(
                        Box::new(term.cast(Type::Float)),
                        factor,
                    )),
                    (Type::Float, Type::Int) => Ok(AnalyzedTerm::Multiply(
                        Box::new(term),
                        factor.cast(Type::Float),
                    )),

                    (Type::Array(box Type::Int, _), Type::Int) => {
                        Ok(AnalyzedTerm::ArrayScalarMultiply(Box::new(term), factor))
                    }
                    (Type::Array(box Type::Int, _), Type::Float) => Ok(
                        AnalyzedTerm::ArrayScalarMultiply(Box::new(term), factor.cast(Type::Int)),
                    ),
                    (Type::Array(box Type::Float, _), Type::Int) => Ok(
                        AnalyzedTerm::ArrayScalarMultiply(Box::new(term), factor.cast(Type::Float)),
                    ),
                    (Type::Array(box Type::Float, _), Type::Float) => {
                        Ok(AnalyzedTerm::ArrayScalarMultiply(Box::new(term), factor))
                    }

                    (Type::Int, Type::Array(box Type::Int, _)) => {
                        Ok(AnalyzedTerm::ScalarArrayMultiply(Box::new(term), factor))
                    }
                    (Type::Float, Type::Array(box Type::Int, _)) => Ok(
                        AnalyzedTerm::ScalarArrayMultiply(Box::new(term.cast(Type::Int)), factor),
                    ),
                    (Type::Int, Type::Array(box Type::Float, _)) => Ok(
                        AnalyzedTerm::ScalarArrayMultiply(Box::new(term.cast(Type::Float)), factor),
                    ),
                    (Type::Float, Type::Array(box Type::Float, _)) => {
                        Ok(AnalyzedTerm::ScalarArrayMultiply(Box::new(term), factor))
                    }

                    (l_type, r_type) => Err(SemanticsError::TypeMismatch(l_type, r_type)),
                }
            }
            Term::DivTerm(term, factor) => {
                let term = AnalyzedTerm::analyze_expression(*term, context)?;
                let factor = AnalyzedFactor::analyze_expression(factor, context)?;

                let term_type = term.get_type(context)?;
                let factor_type = term.get_type(context)?;

                match (term_type, factor_type) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(AnalyzedTerm::Divide(Box::new(term), factor))
                    }
                    (Type::Array(box Type::Int, l_bound), Type::Array(box Type::Int, r_bound))
                    | (
                        Type::Array(box Type::Float, l_bound),
                        Type::Array(box Type::Float, r_bound),
                    ) if l_bound == r_bound => {
                        Ok(AnalyzedTerm::ArrayDivide(Box::new(term), factor))
                    }

                    (Type::Int, Type::Float) => Ok(AnalyzedTerm::Divide(
                        Box::new(term.cast(Type::Float)),
                        factor,
                    )),
                    (Type::Float, Type::Int) => Ok(AnalyzedTerm::Divide(
                        Box::new(term),
                        factor.cast(Type::Float),
                    )),

                    (Type::Array(box Type::Int, _), Type::Int) => {
                        Ok(AnalyzedTerm::ArrayScalarDivide(Box::new(term), factor))
                    }
                    (Type::Array(box Type::Int, _), Type::Float) => Ok(
                        AnalyzedTerm::ArrayScalarDivide(Box::new(term), factor.cast(Type::Int)),
                    ),
                    (Type::Array(box Type::Float, _), Type::Int) => Ok(
                        AnalyzedTerm::ArrayScalarDivide(Box::new(term), factor.cast(Type::Float)),
                    ),
                    (Type::Array(box Type::Float, _), Type::Float) => {
                        Ok(AnalyzedTerm::ArrayScalarDivide(Box::new(term), factor))
                    }

                    (Type::Int, Type::Array(box Type::Int, _)) => {
                        Ok(AnalyzedTerm::ScalarArrayDivide(Box::new(term), factor))
                    }
                    (Type::Float, Type::Array(box Type::Int, _)) => Ok(
                        AnalyzedTerm::ScalarArrayDivide(Box::new(term.cast(Type::Int)), factor),
                    ),
                    (Type::Int, Type::Array(box Type::Float, _)) => Ok(
                        AnalyzedTerm::ScalarArrayDivide(Box::new(term.cast(Type::Float)), factor),
                    ),
                    (Type::Float, Type::Array(box Type::Float, _)) => {
                        Ok(AnalyzedTerm::ScalarArrayDivide(Box::new(term), factor))
                    }

                    (l_type, r_type) => Err(SemanticsError::TypeMismatch(l_type, r_type)),
                }
            }
        }
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedTerm::Multiply(_, factor) => factor.get_type(context),
            AnalyzedTerm::ArrayScalarMultiply(term, _) => term.get_type(context),
            AnalyzedTerm::ScalarArrayMultiply(_, factor) => factor.get_type(context),
            AnalyzedTerm::ArrayMultiply(_, factor) => factor.get_type(context),
            AnalyzedTerm::Divide(_, factor) => factor.get_type(context),
            AnalyzedTerm::ArrayScalarDivide(term, _) => term.get_type(context),
            AnalyzedTerm::ScalarArrayDivide(_, factor) => factor.get_type(context),
            AnalyzedTerm::ArrayDivide(_, factor) => factor.get_type(context),
            AnalyzedTerm::Cast(_, value_type) => Ok(value_type.clone()),
            AnalyzedTerm::Factor(factor) => factor.get_type(context),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedFactor {
    NestedExpression(Box<AnalyzedExpression>),
    ProcedureCall(AnalyzedProcedureCall),
    Name(AnalyzedName),
    NegatedName(AnalyzedName),
    Number(AnalyzedNumber),
    NegatedNumber(AnalyzedNumber),
    String(String),
    True,
    False,

    Cast(Box<AnalyzedFactor>, Type),
}

impl AnalyzedFactor {
    pub fn cast(self, value_type: Type) -> AnalyzedFactor {
        AnalyzedFactor::Cast(Box::new(self), value_type)
    }
}

impl AnalyzeExpression<Factor> for AnalyzedFactor {
    fn analyze_expression(value: Factor, context: &mut Context) -> Result<Self, SemanticsError> {
        match value {
            Factor::NestedExpression(box expression) => Ok(AnalyzedFactor::NestedExpression(
                Box::new(AnalyzedExpression::analyze_expression(expression, context)?),
            )),
            Factor::ProcedureCall(proc_call) => Ok(AnalyzedFactor::ProcedureCall(
                AnalyzedProcedureCall::analyze_expression(proc_call, context)?,
            )),
            Factor::Name { negate, name } => {
                if negate {
                    let name = AnalyzedName::analyze_expression(name, context)?;
                    let value_type = name.get_type(context)?.clone();

                    if matches!(value_type, Type::Int | Type::Float) {
                        Ok(AnalyzedFactor::NegatedName(name))
                    } else {
                        Err(SemanticsError::InvalidType(
                            String::from("Number"),
                            value_type,
                        ))
                    }
                } else {
                    Ok(AnalyzedFactor::Name(AnalyzedName::analyze_expression(
                        name, context,
                    )?))
                }
            }
            Factor::Number { negate, number } => {
                if negate {
                    Ok(AnalyzedFactor::NegatedNumber(
                        AnalyzedNumber::analyze_expression(number, context)?,
                    ))
                } else {
                    Ok(AnalyzedFactor::Number(AnalyzedNumber::analyze_expression(
                        number, context,
                    )?))
                }
            }
            Factor::String(value) => Ok(AnalyzedFactor::String(value.literal_string)),
            Factor::TrueLit => Ok(AnalyzedFactor::True),
            Factor::FalseLit => Ok(AnalyzedFactor::False),
        }
    }

    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedFactor::NestedExpression(box expression) => expression.get_type(context),
            AnalyzedFactor::ProcedureCall(proc_call) => proc_call.get_type(context),
            AnalyzedFactor::Name(name) | AnalyzedFactor::NegatedName(name) => {
                name.get_type(context)
            }
            AnalyzedFactor::Number(number) | AnalyzedFactor::NegatedNumber(number) => {
                number.get_type(context)
            }
            AnalyzedFactor::String(_) => Ok(Type::String),
            AnalyzedFactor::True | AnalyzedFactor::False => Ok(Type::Bool),
            AnalyzedFactor::Cast(_, value_type) => Ok(value_type.clone()),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedNumber {
    Integer(i64),
    Float(f64),
}

impl AnalyzeExpression<Number> for AnalyzedNumber {
    fn analyze_expression(value: Number, context: &mut Context) -> Result<Self, SemanticsError> {
        if value.literal_string.contains(".") {
            Ok(AnalyzedNumber::Float(value.try_into()?))
        } else {
            Ok(AnalyzedNumber::Integer(value.try_into()?))
        }
    }

    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedNumber::Integer(_) => Ok(Type::Int),
            AnalyzedNumber::Float(_) => Ok(Type::Float),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedName {
    Name(String),
    Indexed(String, Box<AnalyzedExpression>),
}

impl AnalyzeExpression<Name> for AnalyzedName {
    fn analyze_expression(value: Name, context: &mut Context) -> Result<Self, SemanticsError> {
        if let Some(box expression) = value.expression {
            let expression = AnalyzedExpression::analyze_expression(expression, context)?;
            let exp_type = expression.get_type(context)?;

            if exp_type != Type::Int {
                Err(SemanticsError::NonIntIndex(
                    value.identifier.identifier_string,
                    exp_type,
                ))
            } else {
                Ok(AnalyzedName::Indexed(
                    value.identifier.identifier_string,
                    Box::new(expression),
                ))
            }
        } else {
            Ok(AnalyzedName::Name(value.identifier.identifier_string))
        }
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedName::Name(identifier) => {
                context.get_variable_type(identifier).map(Type::clone)
            }
            AnalyzedName::Indexed(identifier, _) => {
                let array_type = context.get_variable_type(identifier)?;
                match array_type {
                    Type::Array(_, _) => Ok(array_type.clone()),
                    _ => Err(SemanticsError::IndexOnNonArray(identifier.to_owned())),
                }
            }
        }
    }
}
