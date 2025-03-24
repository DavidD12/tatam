use super::*;
use crate::error::*;
use crate::model::*;
use crate::typing::*;
use crate::*;

impl Expression {
    pub fn get_type(&self, model: &Model) -> Type {
        match self {
            Expression::Bool(_) => Type::Bool,
            Expression::Int(value) => Type::IntInterval(*value, *value),
            Expression::Real(_) => Type::Real,
            //
            Expression::PrefixUnary(op, expr) => match op {
                PrefixUnaryOperator::Not => {
                    let typ = expr.get_type(model);
                    match typ {
                        Type::Bool => typ,
                        _ => Type::Undefined,
                    }
                }
                PrefixUnaryOperator::Neg => {
                    let typ = expr.get_type(model);
                    match typ {
                        Type::Int => typ,
                        Type::Real => typ,
                        Type::IntInterval(min, max) => Type::IntInterval(-max, -min),
                        _ => Type::Undefined,
                    }
                }
            },
            Expression::Binary(left, op, right) => match op {
                BinaryOperator::Eq => Type::Bool,
                BinaryOperator::Ne => Type::Bool,
                BinaryOperator::Lt => Type::Bool,
                BinaryOperator::Le => Type::Bool,
                BinaryOperator::Ge => Type::Bool,
                BinaryOperator::Gt => Type::Bool,
                BinaryOperator::Implies => Type::Bool,
                BinaryOperator::Min => match (left.get_type(model), right.get_type(model)) {
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Real, Type::Real) => Type::Real,
                    (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                        Type::IntInterval(min1.min(min2), max1.min(max2))
                    }
                    (Type::IntInterval(min, max), Type::Int) => Type::IntInterval(min, max),
                    (Type::IntInterval(min, max), Type::Real) => Type::IntInterval(min, max),
                    (Type::Int, Type::IntInterval(min, max)) => Type::IntInterval(min, max),
                    (Type::Real, Type::IntInterval(min, max)) => Type::IntInterval(min, max),
                    _ => Type::Undefined,
                },
                BinaryOperator::Max => match (left.get_type(model), right.get_type(model)) {
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Real, Type::Real) => Type::Real,
                    (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                        Type::IntInterval(min1.max(min2), max1.max(max2))
                    }
                    (Type::IntInterval(min, max), Type::Int) => Type::IntInterval(min, max),
                    (Type::IntInterval(min, max), Type::Real) => Type::IntInterval(min, max),
                    (Type::Int, Type::IntInterval(min, max)) => Type::IntInterval(min, max),
                    (Type::Real, Type::IntInterval(min, max)) => Type::IntInterval(min, max),
                    _ => Type::Undefined,
                },
            },
            Expression::Nary(op, kids) => match op {
                NaryOperator::And => Type::Bool,
                NaryOperator::Or => Type::Bool,
                NaryOperator::Mul => {
                    if let Some((first, others)) = kids.split_first() {
                        let mut t = first.get_type(model);
                        for e in others.iter() {
                            match (&t, e.get_type(model)) {
                                (Type::Int, _) => {}
                                (Type::Real, _) => {}
                                (Type::IntInterval(_, _), Type::Int) => t = Type::Int,
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    t = Type::IntInterval(min1 * min2, max1 * max2)
                                }
                                _ => return Type::Undefined,
                            }
                        }
                        t
                    } else {
                        Type::Undefined
                    }
                }
                NaryOperator::Add => {
                    if let Some((first, others)) = kids.split_first() {
                        let mut t = first.get_type(model);
                        for e in others.iter() {
                            match (&t, e.get_type(model)) {
                                (Type::Int, _) => {}
                                (Type::Real, _) => {}
                                (Type::IntInterval(_, _), Type::Int) => t = Type::Int,
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    t = Type::IntInterval(min1 + min2, max1 + max2)
                                }
                                _ => return Type::Undefined,
                            }
                        }
                        t
                    } else {
                        Type::Undefined
                    }
                }
                NaryOperator::Sub => {
                    if let Some((first, others)) = kids.split_first() {
                        let mut t = first.get_type(model);
                        for e in others.iter() {
                            match (&t, e.get_type(model)) {
                                (Type::Int, _) => {}
                                (Type::Real, _) => {}
                                (Type::IntInterval(_, _), Type::Int) => t = Type::Int,
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    t = Type::IntInterval(min1 - max2, max1 - min2)
                                }
                                _ => return Type::Undefined,
                            }
                        }
                        t
                    } else {
                        Type::Undefined
                    }
                }
            },
            //
            Expression::EnumerateElement(id) => Type::Enumerate(id.enumerate_id()),
            Expression::Declaration(id) => model.get(*id).unwrap().get_type(model),
            Expression::Definition(id) => model.get(*id).unwrap().get_type(model),
            Expression::FunDec(id) => model.get(*id).unwrap().get_type(model),
            Expression::FunDef(id) => model.get(*id).unwrap().get_type(model),
            //
            Expression::Parameter(param) => param.get_type(model),
            //
            Expression::Apply(fun, _) => {
                if let Type::Function(_, ret) = fun.get_type(model) {
                    *ret
                } else {
                    Type::Undefined
                }
            }
            Expression::As(_, typ, _) => typ.get_type(model),
            //
            Expression::Following(kid) => kid.get_type(model),
            Expression::State(kid, _, __) => kid.get_type(model),
            Expression::Scope(_, e) => e.get_type(model),
            //
            Expression::IfThenElse(_, te, list, ee) => {
                let mut res = te.get_type(model);
                for (_, x) in list.iter() {
                    res = res.common_type(model, &x.get_type(model));
                }
                res = res.common_type(model, &ee.get_type(model));
                res
            }
            Expression::Quantifier(op, _, e) => match op {
                QtOperator::Forall => Type::Bool,
                QtOperator::Exists => Type::Bool,
                QtOperator::Sum => match e.get_type(model) {
                    t @ Type::Real => t,
                    t @ Type::Int => t,
                    Type::Interval(_) => Type::Int,
                    Type::IntInterval(_, _) => Type::Int,
                    _ => Type::Undefined,
                },
                QtOperator::Prod => match e.get_type(model) {
                    t @ Type::Real => t,
                    t @ Type::Int => t,
                    Type::Interval(_) => Type::Int,
                    Type::IntInterval(_, _) => Type::Int,
                    _ => Type::Undefined,
                },
                QtOperator::Min => e.get_type(model),
                QtOperator::Max => e.get_type(model),
            },
            //
            Expression::LTLunary(_, _) => Type::Bool,
            Expression::LTLbinary(_, _, _) => Type::Bool,
            Expression::LTLVariable(_) => Type::Bool,
            //
            Expression::Unresolved(_) => Type::Undefined,
        }
    }
}

impl Expr {
    pub fn get_type(&self, model: &Model) -> Type {
        self.expression().get_type(model)
    }

    pub fn check_type(&self, model: &Model) -> Result<(), Error> {
        match self.expression() {
            Expression::Bool(_) => Ok(()),
            Expression::Int(_) => Ok(()),
            Expression::Real(_) => Ok(()),
            //
            Expression::PrefixUnary(op, kid) => {
                kid.check_type(model)?;
                match op {
                    PrefixUnaryOperator::Not => kid.check_is_bool(model)?,
                    PrefixUnaryOperator::Neg => kid.check_is_number(model)?,
                }
                Ok(())
            }
            Expression::Binary(left, op, right) => {
                left.check_type(model)?;
                right.check_type(model)?;
                let l_type = left.get_type(model);
                // Eq/Ne
                if [BinaryOperator::Eq, BinaryOperator::Ne].contains(op) {
                    right.check_compatible(model, l_type)
                }
                // Compare
                else if [
                    BinaryOperator::Lt,
                    BinaryOperator::Le,
                    BinaryOperator::Ge,
                    BinaryOperator::Gt,
                ]
                .contains(op)
                {
                    left.check_is_number(model)?;
                    right.check_is_number(model)?;
                    right.check_compatible(model, l_type)
                }
                // Bool
                else if [BinaryOperator::Implies].contains(op) {
                    left.check_is_bool(model)?;
                    right.check_is_bool(model)
                } else {
                    panic!("undefined")
                }
            }
            Expression::Nary(op, kids) => {
                if [NaryOperator::And, NaryOperator::Or].contains(op) {
                    for e in kids.iter() {
                        e.check_type(model)?;
                        e.check_is_bool(model)?;
                    }
                    Ok(())
                } else if [NaryOperator::Mul, NaryOperator::Add, NaryOperator::Sub].contains(op) {
                    Expr::check_all_integer_or_real(model, kids)
                } else {
                    panic!("undefined")
                }
            }
            //
            Expression::EnumerateElement(_) => Ok(()),
            Expression::Declaration(_) => Ok(()),
            Expression::Definition(_) => Ok(()),
            Expression::FunDec(_) => Ok(()),
            Expression::FunDef(_) => Ok(()),
            //
            Expression::Parameter(_) => Ok(()),
            //
            Expression::Apply(fun, params) => {
                fun.check_type(model)?;
                for p in params.iter() {
                    p.check_type(model)?;
                }
                let fun_type = fun.get_type(model);
                if let Type::Function(p, _) = fun_type {
                    for (t, p) in p.into_iter().zip(params.iter()) {
                        p.check_subtype(model, &t)?;
                    }
                    Ok(())
                } else {
                    Err(Error::Type {
                        expr: *fun.clone(),
                        typ: fun_type,
                        expected: vec![],
                    })
                }
            }
            Expression::As(kid, typ, default) => {
                let t = typ.get_type(model);
                match t {
                    Type::IntInterval(_, _) => {}
                    _ => {
                        return Err(Error::Type {
                            expr: self.clone(),
                            typ: t,
                            expected: vec![Type::IntInterval(0, 0)],
                        })
                    }
                }
                kid.check_is_integer(model)?;
                default.check_subtype(model, &t)
            }
            //
            Expression::Following(kid) => kid.check_type(model),
            Expression::State(kid, _, default) => {
                kid.check_type(model)?;
                if let Some(default) = default {
                    default.check_type(model)?;
                    default.check_subtype(model, &kid.get_type(model))?;
                }
                Ok(())
            }
            Expression::Scope(_, e) => e.check_is_bool(model),
            //
            Expression::IfThenElse(ce, te, list, ee) => {
                ce.check_type(model)?;
                ce.check_is_bool(model)?;
                te.check_type(model)?;
                for (ce, ee) in list.iter() {
                    ce.check_type(model)?;
                    ce.check_is_bool(model)?;
                    ee.check_type(model)?;
                }
                ee.check_type(model)?;
                //
                let t = self.get_type(model);
                te.check_subtype(model, &t)?;
                for (_, ee) in list.iter() {
                    ee.check_subtype(model, &t)?;
                }
                ee.check_subtype(model, &t)
            }
            Expression::Quantifier(op, _, e) => {
                e.check_type(model)?;
                match op {
                    QtOperator::Forall => e.check_is_bool(model),
                    QtOperator::Exists => e.check_is_bool(model),
                    QtOperator::Sum => e.check_is_number(model),
                    QtOperator::Prod => e.check_is_number(model),
                    QtOperator::Min => e.check_is_number(model),
                    QtOperator::Max => e.check_is_number(model),
                }
            }
            //
            Expression::LTLunary(_, kid) => {
                kid.check_type(model)?;
                kid.check_is_bool(model)
            }
            Expression::LTLbinary(left, _, right) => {
                left.check_type(model)?;
                right.check_type(model)?;
                left.check_is_bool(model)?;
                right.check_is_bool(model)
            }
            Expression::LTLVariable(_) => Ok(()),
            //
            Expression::Unresolved(_) => Ok(()),
        }
    }

    pub fn check_subtype(&self, model: &Model, supertype: &Type) -> Result<(), Error> {
        let my_type = self.get_type(model);
        if my_type.is_subtype_of(&supertype) {
            Ok(())
        } else {
            Err(Error::Type {
                expr: self.clone(),
                typ: my_type.clone(),
                expected: vec![supertype.clone()],
            })
        }
    }

    pub fn check_compatible(&self, model: &Model, other: Type) -> Result<(), Error> {
        let my_type = self.get_type(model);
        if my_type.is_compatible_with(&other) {
            Ok(())
        } else {
            Err(Error::Type {
                expr: self.clone(),
                typ: my_type.clone(),
                expected: vec![other],
            })
        }
    }

    pub fn check_is_bool(&self, model: &Model) -> Result<(), Error> {
        self.check_subtype(model, &Type::Bool)
    }

    pub fn check_is_number(&self, model: &Model) -> Result<(), Error> {
        let my_type = self.get_type(model);
        let int_type = Type::Int;
        let real_type = Type::Real;
        if my_type.is_subtype_of(&int_type) || my_type.is_subtype_of(&real_type) {
            Ok(())
        } else {
            Err(Error::Type {
                expr: self.clone(),
                typ: my_type.clone(),
                expected: vec![int_type, real_type],
            })
        }
    }

    pub fn check_is_integer(&self, model: &Model) -> Result<(), Error> {
        match self.get_type(model) {
            Type::Int => Ok(()),
            Type::IntInterval(_, _) => Ok(()),
            t => Err(Error::Type {
                expr: self.clone(),
                typ: t,
                expected: vec![Type::Int],
            }),
        }
    }

    pub fn check_is_real(&self, model: &Model) -> Result<(), Error> {
        match self.get_type(model) {
            Type::Real => Ok(()),
            t => Err(Error::Type {
                expr: self.clone(),
                typ: t,
                expected: vec![Type::Real],
            }),
        }
    }

    pub fn check_all_integer_or_real(model: &Model, exprs: &Vec<Expr>) -> Result<(), Error> {
        if let Some((first, others)) = exprs.split_first() {
            let is_integer = match first.get_type(model) {
                Type::Int => true,
                Type::IntInterval(_, _) => true,
                Type::Real => false,
                t => {
                    return Err(Error::Type {
                        expr: first.clone(),
                        typ: t,
                        expected: vec![Type::Int, Type::Real],
                    })
                }
            };
            for e in others.iter() {
                if is_integer {
                    e.check_is_integer(model)?;
                } else {
                    e.check_is_real(model)?;
                }
            }
        }
        Ok(())
    }
}
