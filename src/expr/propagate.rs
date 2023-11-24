use fraction::Fraction;
use fraction::Zero;

use super::*;
use crate::model::*;
use crate::typing::*;
use crate::*;

impl Expr {
    pub fn propagate(&self, model: &Model) -> Expr {
        match self.expression() {
            Expression::Bool(_) => self.clone(),
            Expression::Int(_) => self.clone(),
            Expression::Real(_) => self.clone(),
            //
            Expression::PrefixUnary(op, kid) => {
                let kid = kid.propagate(model);
                match op {
                    PrefixUnaryOperator::Not => {
                        if let Expression::Bool(value) = kid.expression() {
                            (!value).into()
                        } else {
                            !kid
                        }
                    }
                    PrefixUnaryOperator::Neg => match kid.expression() {
                        Expression::Int(value) => (-value).into(),
                        Expression::Real(value) => (-value).into(),
                        _ => {
                            if let Type::IntInterval(min, max) = kid.get_type(model) {
                                if min == max {
                                    return (-min).into();
                                }
                            }
                            -kid
                        }
                    },
                }
            }
            Expression::Binary(left, op, right) => {
                let left = left.propagate(model);
                let right = right.propagate(model);
                match op {
                    BinaryOperator::Eq => {
                        match (left.expression(), right.expression()) {
                            (Expression::Bool(l), Expression::Bool(r)) => {
                                if l == r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l == r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l == r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::EnumerateElement(l), Expression::EnumerateElement(r)) => {
                                if l == r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if max1 < min2 || max2 < min1 {
                                        return false.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.eq(right)
                    }
                    BinaryOperator::Ne => {
                        match (left.expression(), right.expression()) {
                            (Expression::Bool(l), Expression::Bool(r)) => {
                                if l != r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l != r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l != r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::EnumerateElement(l), Expression::EnumerateElement(r)) => {
                                if l != r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if max1 < min2 || max2 < min1 {
                                        return true.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.ne(right)
                    }
                    BinaryOperator::Lt => {
                        match (left.expression(), right.expression()) {
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l < r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l < r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if max1 < min2 {
                                        return true.into();
                                    }
                                    if min1 >= max2 {
                                        return false.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.lt(right)
                    }
                    BinaryOperator::Le => {
                        match (left.expression(), right.expression()) {
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l <= r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l <= r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if max1 <= min2 {
                                        return true.into();
                                    }
                                    if min1 > max2 {
                                        return false.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.le(right)
                    }
                    BinaryOperator::Ge => {
                        match (left.expression(), right.expression()) {
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l >= r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l >= r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if min1 >= max2 {
                                        return true.into();
                                    }
                                    if max1 < min2 {
                                        return false.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.ge(right)
                    }
                    BinaryOperator::Gt => {
                        match (left.expression(), right.expression()) {
                            (Expression::Int(l), Expression::Int(r)) => {
                                if l > r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            (Expression::Real(l), Expression::Real(r)) => {
                                if l > r {
                                    return true.into();
                                } else {
                                    return false.into();
                                }
                            }
                            _ => match (left.get_type(model), right.get_type(model)) {
                                (Type::IntInterval(min1, max1), Type::IntInterval(min2, max2)) => {
                                    if min1 > max2 {
                                        return true.into();
                                    }
                                    if max1 <= min2 {
                                        return false.into();
                                    }
                                }
                                _ => {}
                            },
                        }
                        left.gt(right)
                    }
                    BinaryOperator::Implies => {
                        if let Expression::Bool(false) = left.expression() {
                            return true.into();
                        }
                        if let Expression::Bool(true) = right.expression() {
                            return true.into();
                        }
                        if let (Expression::Bool(true), Expression::Bool(false)) =
                            (left.expression(), right.expression())
                        {
                            return false.into();
                        }
                        left.implies(right)
                    }
                }
                //
            }
            Expression::Nary(op, kids) => match op {
                NaryOperator::And => {
                    let mut v = Vec::new();
                    for e in kids.iter() {
                        let kid = e.propagate(model);
                        if let Expression::Bool(false) = kid.expression() {
                            return false.into();
                        }
                        if let Expression::Bool(true) = kid.expression() {
                        } else {
                            v.push(kid);
                        }
                    }
                    Expr::and(v)
                }
                NaryOperator::Or => {
                    let mut v = Vec::new();
                    for e in kids.iter() {
                        let kid = e.propagate(model);
                        if let Expression::Bool(true) = kid.expression() {
                            return true.into();
                        }
                        if let Expression::Bool(false) = kid.expression() {
                        } else {
                            v.push(kid);
                        }
                    }
                    Expr::or(v)
                }
                NaryOperator::Add => {
                    let mut int_value = 0;
                    let mut has_int = false;
                    let mut has_real = false;
                    let mut real_value = Fraction::zero();
                    let mut v = Vec::new();
                    for e in kids.iter() {
                        let kid = e.propagate(model);
                        match kid.expression() {
                            Expression::Int(v) => {
                                int_value += v;
                                has_int = true
                            }
                            Expression::Real(v) => {
                                real_value += v;
                                has_real = true
                            }
                            _ => v.push(kid),
                        }
                    }
                    if v.is_empty() && has_int {
                        return int_value.into();
                    }
                    if v.is_empty() && has_real {
                        return real_value.into();
                    }
                    if int_value != 0 {
                        v.push(int_value.into());
                    }
                    if real_value != Fraction::zero() {
                        v.push(real_value.into());
                    }
                    Expr::add(v)
                }
                NaryOperator::Sub => {
                    if let Some((first, others)) = kids.split_first() {
                        let first = first.propagate(model);
                        let first_type = first.get_type(model);

                        if first_type == Type::Real {
                            let mut v = Vec::new();
                            let mut value = Fraction::zero();
                            for e in others.iter() {
                                let e = e.propagate(model);
                                if let Expression::Real(v) = e.expression() {
                                    value += v;
                                } else {
                                    v.push(e);
                                }
                            }
                            if v.is_empty() {
                                if value.is_zero() {
                                    return first;
                                }
                                if let Expression::Real(f) = first.expression() {
                                    return (f - value).into();
                                }
                            }
                            if !value.is_zero() {
                                v.push(value.into());
                            }
                            v.insert(0, first);
                            return Expr::sub(v);
                        }
                        if first_type.is_integer() {
                            let mut v = Vec::new();
                            let mut value = 0;
                            for e in others.iter() {
                                let e = e.propagate(model);
                                if let Expression::Int(v) = e.expression() {
                                    value += v;
                                } else {
                                    v.push(e);
                                }
                            }
                            if v.is_empty() {
                                if value == 0 {
                                    return first;
                                }
                                if let Expression::Int(f) = first.expression() {
                                    return (f - value).into();
                                }
                            }
                            if value != 0 {
                                v.push(value.into());
                            }
                            v.insert(0, first);
                            return Expr::sub(v);
                        }
                        let mut v = vec![first];
                        v.extend(others.iter().map(|e| e.propagate(model)));
                        return Expr::sub(v);
                    }
                    self.clone()
                }
                NaryOperator::Mul => {
                    let mut int_value = 0;
                    let mut has_int = false;
                    let mut has_real = false;
                    let mut real_value = Fraction::zero();
                    let mut v = Vec::new();
                    for e in kids.iter() {
                        let kid = e.propagate(model);
                        match kid.expression() {
                            Expression::Int(v) => {
                                int_value *= v;
                                has_int = true
                            }
                            Expression::Real(v) => {
                                real_value *= v;
                                has_real = true
                            }
                            _ => v.push(kid),
                        }
                    }
                    if v.is_empty() && has_int {
                        return int_value.into();
                    }
                    if v.is_empty() && has_real {
                        return real_value.into();
                    }
                    if int_value != 0 {
                        v.push(int_value.into());
                    }
                    if real_value != Fraction::zero() {
                        v.push(real_value.into());
                    }
                    Expr::mul(v)
                }
            },
            //
            Expression::EnumerateElement(_) => self.clone(),
            Expression::Declaration(_) => self.clone(),
            Expression::Definition(id) => {
                let def = model.get(*id).unwrap();
                match def.get_type(model) {
                    Type::IntInterval(min, max) => {
                        if min == max {
                            return min.into();
                        }
                    }
                    _ => {}
                }
                def.expr().propagate(model)
            }
            Expression::FunDec(_) => self.clone(),
            Expression::FunDef(_) => self.clone(),
            //
            Expression::Parameter(_) => self.clone(),
            //
            Expression::Apply(fun, params) => {
                let fun = fun.propagate(model);
                let params = params
                    .iter()
                    .map(|e| e.propagate(model))
                    .collect::<Vec<_>>();
                if let Expression::FunDef(id) = fun.expression() {
                    let fun = model.get(*id).unwrap();
                    let list: Vec<(Expr, Expr)> = fun
                        .parameters()
                        .iter()
                        .map(|p| p.clone().into())
                        .zip(params)
                        .collect::<Vec<_>>();
                    fun.expr().substitute_all(list)
                } else {
                    Expression::Apply(Box::new(fun), params).into()
                }
            }
            Expression::As(kid, typ, default) => {
                let kid = kid.propagate(model);
                let default = default.propagate(model);
                let expression = Expression::As(Box::new(kid), typ.clone(), Box::new(default));
                Expr::new(expression, self.position().clone())
            }
            //
            Expression::Following(kid) => {
                let kid = kid.propagate(model);
                let expression = Expression::Following(Box::new(kid));
                Expr::new(expression, self.position().clone())
            }
            Expression::State(kid, state) => {
                let kid = kid.propagate(model);
                let expression = Expression::State(Box::new(kid), *state);
                Expr::new(expression, self.position().clone())
            }
            //
            Expression::IfThenElse(ce, te, list, ee) => {
                let ce = ce.propagate(model);
                if let Expression::Bool(true) = ce.expression() {
                    return te.propagate(model);
                }

                let mut v = vec![];
                for (ce, ee) in list.iter() {
                    let ce = ce.propagate(model);
                    if let Expression::Bool(false) = ce.expression() {
                    } else {
                        v.push((ce, ee.propagate(model)));
                    }
                }
                // All False ?
                let mut all_false = false;

                if let Expression::Bool(false) = ce.expression() {
                    all_false = true;
                }
                for (ce, ee) in v.iter() {
                    if let Expression::Bool(true) = ce.expression() {
                        if all_false {
                            return ee.clone();
                        }
                    } else {
                        all_false = false;
                    }
                }
                let ee = ee.propagate(model);
                if all_false {
                    return ee;
                }
                let te = te.propagate(model);
                let expression =
                    Expression::IfThenElse(Box::new(ce), Box::new(te), v, Box::new(ee));
                Expr::new(expression, None)
            }
            Expression::Quantifier(op, params, e) => {
                let e = e.propagate(model);
                let expression = op.new(params.clone(), e);
                Expr::new(expression, self.position().clone())
            }
            //
            Expression::LTLunary(op, kid) => {
                let kid = kid.propagate(model);
                let expression = Expression::LTLunary(*op, Box::new(kid));
                Expr::new(expression, self.position().clone())
            }
            Expression::LTLbinary(left, op, right) => {
                let left = left.propagate(model);
                let right = right.propagate(model);
                let expression = Expression::LTLbinary(Box::new(left), *op, Box::new(right));
                Expr::new(expression, self.position().clone())
            }
            Expression::LTLVariable(_) => self.clone(),
            //
            Expression::Unresolved(_) => self.clone(),
        }
    }
}
